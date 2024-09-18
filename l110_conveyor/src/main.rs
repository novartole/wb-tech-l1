fn main() {
    let n = 10;
    println!("# solution 1 with N={}", n);
    solution_1::run_with(n);

    println!();

    println!("# solution 2 with N={}", n);
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            solution_2::run_with(n).await;
        });
}

// use system threads
mod solution_1 {
    use std::{sync::mpsc, thread, time::Duration};

    pub fn run_with(n: usize) {
        let (num_tx, num_rx) = mpsc::channel();

        let num_handler = thread::spawn(move || {
            println!("num: start");

            let (val_tx, val_rx) = mpsc::channel();

            let val_handler = thread::spawn(move || {
                println!("  val: start");

                while let Some(val) = val_rx.iter().next() {
                    println!("- {}", val);
                }

                println!("  val: end");
            });

            while let Some(num) = num_rx.iter().next() {
                val_tx.send(num * num).unwrap();
            }

            drop(val_tx);
            val_handler.join().unwrap();

            println!("num: end");
        });

        let dur_of_100ms = Duration::from_millis(100);
        for num in 0..n {
            num_tx.send(num).unwrap();
            thread::sleep(dur_of_100ms);
        }

        drop(num_tx);
        num_handler.join().unwrap();
    }
}

// use tokio runtime
mod solution_2 {
    use std::time::Duration;

    use tokio::sync::mpsc;

    pub async fn run_with(n: usize) {
        let (num_tx, mut num_rx) = mpsc::unbounded_channel();

        let num_handler = tokio::spawn(async move {
            println!("num: start");

            let (val_tx, mut val_rx) = mpsc::unbounded_channel();

            let val_handler = tokio::spawn(async move {
                println!(" val: start");

                while let Some(val) = val_rx.recv().await {
                    println!("- {}", val);
                }

                println!(" val: end");
            });

            while let Some(num) = num_rx.recv().await {
                val_tx.send(num * num).unwrap();
            }

            drop(val_tx);
            val_handler.await.unwrap();

            println!("num: end");
        });

        let dur_of_100ms = Duration::from_millis(100);
        for num in 0..n {
            num_tx.send(num).unwrap();
            tokio::time::sleep(dur_of_100ms).await;
        }

        drop(num_tx);
        num_handler.await.unwrap();
    }
}
