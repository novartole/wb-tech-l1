use std::{
    ops::AddAssign,
    sync::{Arc, Barrier, Mutex},
    thread,
};

fn main() {
    let (number, workers) = parse_args();
    println!("{}", counter(number, workers));
}

/// # Panics
/// Panic if not enough arguments were provided
/// or an argument has an unexpected type.
fn parse_args() -> (u64, usize) {
    let mut args = std::env::args();

    let prc = args.next().unwrap();

    let n = args
        .next()
        .unwrap_or_else(|| panic!(r#"expected number; example: {} <number> _"#, prc))
        .parse()
        .expect("expected primitive number");

    let w = args
        .next()
        .unwrap_or_else(|| panic!(r#"expected number of workers; example: {} _ <number>"#, prc))
        .parse()
        .expect("expected primitive number");

    (n, w)
}

#[derive(Default)]
struct Counter<T> {
    val: T,
}

impl<T> Counter<T> {
    fn add(&mut self, val: T)
    where
        T: AddAssign,
    {
        self.val += val;
    }

    fn into_inner(self) -> T {
        self.val
    }
}

fn counter(n: u64, workers: usize) -> u64 {
    let count = Arc::new(Mutex::new(Counter::default()));
    let bar = Arc::new(Barrier::new(workers));

    thread::scope(|scp| {
        for _ in 0..workers {
            scp.spawn({
                let count_ = count.clone();
                let bar_ = Arc::clone(&bar);

                move || {
                    // start workers at the same time
                    bar_.wait();

                    for _ in 0..n {
                        count_.lock().unwrap().add(1);
                    }
                }
            });
        }
    });

    // wird way to take out the result behind Arc,
    // but it's safe because all other users are done with it
    Arc::into_inner(count)
        .expect("consuming Arc: no more users")
        .into_inner()
        .expect("consuming Mutex: no more users")
        .into_inner()
}

#[cfg(test)]
mod l117 {
    use super::*;

    #[test]
    fn solution_1_with_4_workers_and_1_mil_devided_by_4() {
        let workers = 4;
        let n = 1_000_000 / workers as u64;

        assert_eq!(counter(n, workers), n * workers as u64);
    }
}
