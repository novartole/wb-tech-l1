fn main() {
    let mut args = std::env::args();
    let prc = args.next().unwrap();
    let secs = args
        .next()
        .unwrap_or_else(|| panic!("expected seconds: {} <secs: u32>", prc))
        .parse()
        .expect("failed to parse seconds");

    println!("running with {} second(s)..", secs);
    countdown(secs);
}

fn countdown(secs: usize) {
    use std::{
        sync::mpsc,
        thread,
        time::{Duration, Instant},
    };

    thread::scope(|scp| {
        let (tx, rx) = mpsc::sync_channel(secs);

        scp.spawn(move || {
            let one_sec = Duration::from_secs(1);
            let now = Instant::now();
            for _ in 0..secs {
                thread::sleep(one_sec);
                tx.send(now).unwrap();
            }
        });

        while let Ok(now) = rx.recv() {
            println!("{:?}", now.elapsed().as_secs());
        }
    });
}
