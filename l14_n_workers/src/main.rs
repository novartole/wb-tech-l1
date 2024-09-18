use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// Check the result with 3 active workers by calling:
/// ```bash
/// cat Cargo.toml | cargo run 3
/// ```
fn main() {
    let mut args = std::env::args();
    let prc = args.next().unwrap();
    let n = args
        .next()
        .unwrap_or_else(|| panic!("expected number of workers: {} <workers: u32>", prc))
        .parse()
        .expect("failed to parse number of workers");

    echo_with_workers(n);
}

fn echo_with_workers(n: usize) {
    thread::scope(|scp| {
        let (tx, rx) = {
            let (tx, rx) = mpsc::channel();
            (tx, Arc::new(Mutex::new(rx)))
        };

        for worker_id in 0..n {
            scp.spawn({
                let rx_ = Arc::clone(&rx);
                move || loop {
                    let maybe_word = {
                        let rx = rx_.lock().unwrap();
                        rx.iter().next()
                    };
                    match maybe_word {
                        Some(word) => println!("worker_{} got: {}", worker_id, word),
                        None => break,
                    }
                }
            });
        }

        let mut buf = String::new();
        while io::stdin().read_line(&mut buf).is_ok_and(|n| n > 0) {
            let word = buf.drain(..).as_str().trim_end().to_owned();
            tx.send(word).unwrap();
        }
    });
}
