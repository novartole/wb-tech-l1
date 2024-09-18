#[tokio::main]
async fn main() {
    println!("# solution 1 with 5 workers");
    solution_1::run_with(5).await;

    println!("# solution 2 with 5 workers");
    solution_2::run_with(5).await;
}

// use std::broadcast to get the signal on worker thread
mod solution_1 {
    pub async fn run_with(workers: usize) {
        use tokio::sync::broadcast;

        // just to avoid () for send
        #[derive(Clone)]
        struct ShutdownRequest;

        let (shutdown_tx, _) = broadcast::channel(workers);

        for worker_id in 0..workers {
            tokio::spawn({
                let mut shutdown_rx = shutdown_tx.subscribe();
                async move {
                    println!("worker_{} is up!", worker_id);

                    // emulate some hard task
                    let main_task_of_worker = async {
                        loop {
                            tokio::task::yield_now().await;
                        }
                    };

                    // work on main task but check for cancellation
                    tokio::select! {
                        _ = main_task_of_worker => { /* ... */ },
                        result = shutdown_rx.recv() => if let Err(e) = result {
                            println!("worker_{} failed to listen for shutdown: {}", worker_id, e);
                        }
                    }

                    /* space for soft shutdown */

                    println!("worker_{} is finishing..", worker_id);
                }
            });
        }

        if let Err(e) = tokio::signal::ctrl_c().await {
            println!("failed to wait for ctrl-c signal: {}", e);
        }
        if let Err(e) = shutdown_tx.send(ShutdownRequest) {
            println!("failed to send shutdown properly: {}", e);
        }

        // a way to sync worker threads
        while shutdown_tx.receiver_count() > 0 {
            tokio::task::yield_now().await;
        }

        println!("ALL workers finished!");
    }
}

// similar solution but with flume under the hood
mod solution_2 {
    pub async fn run_with(workers: usize) {
        let (tx, rx) = flume::unbounded();

        for worker_id in 0..workers {
            tokio::spawn({
                let tx_ = tx.clone();
                async move {
                    println!("worker_{} is up!", worker_id);

                    let main_task_of_worker = async {
                        loop {
                            tokio::task::yield_now().await;
                        }
                    };

                    tokio::select! {
                        _ = main_task_of_worker => { /* ... */ },
                        result = tokio::signal::ctrl_c() => if let Err(e) = result {
                            println!("failed to wait for ctrl-c signal: {}", e);
                        }
                    }

                    /* space for soft shutdown */

                    if let Err(e) = tx_.send_async(worker_id).await {
                        println!("failed to send shutdown properly: {}", e);
                    }
                }
            });
        }

        drop(tx);
        for worker_id in rx.iter() {
            println!("worker_{} finished", worker_id);
        }

        println!("ALL workers finished!");
    }
}
