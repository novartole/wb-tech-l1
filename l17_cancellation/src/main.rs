fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build runtime for demo");

    runtime.block_on(async {
        println!("# solution 1:");
        solution_1::one_worker_waits_for_cancellation().await;
    });

    println!();

    runtime.block_on(async {
        let workers = 3;
        println!("# solution 2 with {} workers:", workers);
        solution_2::workers_wait_for_closing_channel(workers).await;
    });

    println!();

    runtime.block_on(async {
        let workers = 2;
        println!("# solution 3 with {} workers:", workers);
        solution_3::workers_wait_for_token(workers).await;
    });
}

mod solution_1 {
    pub async fn one_worker_waits_for_cancellation() {
        let (tx, mut rx) = tokio::sync::oneshot::channel();

        let worker = tokio::spawn(async move {
            println!("W: worker is up");

            loop {
                tokio::select! {
                    _ = tokio::task::yield_now() => continue,
                    _ = &mut rx => break println!("W: worker is cancelling"),
                }
            }
        });

        // here could be any event to react on
        let cancell_request = async {
            println!("wait for ctrl-c signal");

            match tokio::signal::ctrl_c().await {
                Ok(_) => println!(), // print an empty line for beauty
                Err(e) => println!("failed to get cancell request: {}", e),
            };
        };

        // worker runs in background,
        // while main thread is waiting for cancellation
        cancell_request.await;

        println!("graceful shutdown");
        // no need the value back
        let _ = tx.send(()).is_ok();
        worker
            .await
            .expect("cannot sync worker with the main thread");

        println!("all is sync'ed");
    }
}

mod solution_2 {
    use std::{sync::Arc, time::Duration};
    use tokio::sync::Mutex;
    use tokio_util::task::TaskTracker;

    pub async fn workers_wait_for_closing_channel(workers: usize) {
        let (tx, rx) = {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            (tx, Arc::new(Mutex::new(rx)))
        };

        let tracker = TaskTracker::new();

        for worker_id in 0..workers {
            tracker.spawn({
                let rx_ = Arc::clone(&rx);
                async move {
                    while let Some(val) = rx_.lock().await.recv().await {
                        println!("worker_{} got {}", worker_id, val);
                    }

                    println!("worker_{}: channel is empty", worker_id);
                }
            });
        }

        let one_sec = Duration::from_secs(1);
        for num in (1..=5).rev() {
            tx.send(num).unwrap();
            tokio::time::sleep(one_sec).await;
        }

        drop(tx);
        tracker.close();
        tracker.wait().await;

        println!("all workers finished");
    }
}

mod solution_3 {
    use std::time::Duration;

    use tokio_util::{sync::CancellationToken, task::TaskTracker};

    pub async fn workers_wait_for_token(workers: usize) {
        let cancellation_token = CancellationToken::new();
        let tracker = TaskTracker::new();

        for worker_id in 0..workers {
            tracker.spawn({
                let cancellation_token_ = cancellation_token.clone();
                async move {
                    println!("woker_{} is up", worker_id);

                    loop {
                        tokio::select! {
                            _ = tokio::task::yield_now() => continue,
                            _ = cancellation_token_.cancelled() => break,
                        }
                    }

                    println!("worker_{} was requested to cancell", worker_id);
                }
            });
        }

        let secs = 3;
        println!("sleep for {} seconds.. zZz", secs);
        tokio::time::sleep(Duration::from_secs(secs)).await;

        cancellation_token.cancel();
        tracker.close();
        tracker.wait().await;

        println!("all workers finished");
    }
}
