fn main() {
    let n = 22_000;
    let nums = &Vec::from_iter(1..=n);

    let mut sum = solution_1::sum_of_pow2s(nums);
    println!("1 + ... + {} = {}", n, sum);

    sum = solution_2::sum_of_pow2s(nums, 3);
    println!("1 + ... + {} = {}", n, sum);
}

// one thread
mod solution_1 {
    use std::{sync::mpsc, thread};

    pub fn sum_of_pow2s(nums: &[u64]) -> u64 {
        let mut sum = 0;

        thread::scope(|scp| {
            let (tx, rx) = mpsc::channel();

            scp.spawn({
                let sum = &mut sum;
                move || {
                    for val in rx.iter() {
                        *sum += val;
                    }
                }
            });

            for number in nums {
                tx.send(number * number).unwrap();
            }
        });

        sum
    }
}

// include workers
mod solution_2 {
    use std::{
        sync::{
            atomic::{AtomicU64, Ordering},
            mpsc, Arc, Mutex,
        },
        thread,
    };

    pub fn sum_of_pow2s(nums: &[u64], workers: usize) -> u64 {
        assert_ne!(workers, 0, "at least 1 worker is expected");

        let sum = AtomicU64::new(0);

        thread::scope(|scp| {
            let (tx, rx) = {
                let (tx, rx) = mpsc::channel();
                (tx, Arc::new(Mutex::new(rx)))
            };

            let sum = &sum;

            for _ in 0..workers {
                scp.spawn({
                    let rx_ = Arc::clone(&rx);
                    move || loop {
                        let maybe_val = {
                            let rx_lock = rx_.lock().unwrap();
                            rx_lock.iter().next()
                        };
                        match maybe_val {
                            Some(val) => sum.fetch_add(val, Ordering::Relaxed),
                            None => return,
                        };
                    }
                });
            }

            for num in nums {
                tx.send(num * num).unwrap();
            }
        });

        sum.into_inner()
    }
}

#[cfg(test)]
mod l13 {
    use super::solution_1;
    use super::solution_2;

    fn formula(n: u64) -> u64 {
        n * (n + 1) * (2 * n + 1) / 6
    }

    #[test]
    fn solution_1_with_22k_numbers() {
        let n = 22_000;
        let nums = &Vec::from_iter(1..=n);
        assert_eq!(solution_1::sum_of_pow2s(nums), formula(n));
    }

    #[test]
    fn solution_1_with_100k_numbers() {
        let n = 100_000;
        let nums = &Vec::from_iter(1..=n);
        assert_eq!(solution_1::sum_of_pow2s(nums), formula(n));
    }

    #[test]
    fn solution_2_with_100k_numbers_5_workers() {
        let n = 100_000;
        let nums = &Vec::from_iter(1..=n);
        assert_eq!(solution_2::sum_of_pow2s(nums, 5), formula(n));
    }

    #[test]
    #[should_panic]
    fn solution_2_with_1_number_0_worker() {
        let n = 1;
        let nums = &Vec::from_iter(1..=n);
        assert_eq!(solution_2::sum_of_pow2s(nums, 0), formula(n));
    }
}
