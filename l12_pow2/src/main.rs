fn main() {
    let nums = &Vec::from_iter(1..=10);

    solution_1::print_pow2(nums);
    solution_2::print_pow2(nums, 3);
}

/// Calculate and print values in a spawned thread.
mod solution_1 {
    pub fn print_pow2(nums: &[u64]) {
        // keep _nums_ in scope till the end
        std::thread::scope(|scp| {
            scp.spawn(|| {
                #[cfg(test)]
                let mut sum = 0;

                for val in nums.iter().map(|num| num * num) {
                    println!("{}", val);

                    #[cfg(test)]
                    {
                        sum += val;
                    }
                }

                #[cfg(test)]
                assert_eq!(sum, {
                    let n = nums.len() as u64;
                    n * (n + 1) * (2 * n + 1) / 6
                });
            });
        });
    }
}

/// A sender sprays calculated values one-by-one, and multiply consumers fight for printing.
mod solution_2 {
    #[cfg(test)]
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    pub fn print_pow2(nums: &[u64], workers: usize) {
        assert_ne!(workers, 0, "at least 1 worker is expected");

        #[cfg(test)]
        let sum = AtomicU64::new(0);

        thread::scope(|scp| {
            let (tx, rx) = {
                let (tx, rx) = mpsc::channel();
                (tx, Arc::new(Mutex::new(rx)))
            };

            #[cfg(test)]
            let sum = &sum;

            for _ in 0..workers {
                scp.spawn({
                    let rx_ = Arc::clone(&rx);
                    move || loop {
                        // drop lock immidiatly once received a value
                        let maybe_val = {
                            let rx_lock = rx_.lock().unwrap();
                            rx_lock.iter().next()
                        };
                        match maybe_val {
                            Some(val) => {
                                println!("{}", val);

                                #[cfg(test)]
                                {
                                    // val is built iteratively - Ordering doesn't really matter
                                    sum.fetch_add(val, Ordering::Relaxed);
                                }
                            }
                            None => return,
                        }
                    }
                });
            }

            for val in nums.iter().map(|num| num * num) {
                tx.send(val).unwrap();
            }
        });

        #[cfg(test)]
        assert_eq!(sum.into_inner(), {
            let n = nums.len() as u64;
            // sum of squares of first n numbers
            n * (n + 1) * (2 * n + 1) / 6
        });
    }
}

#[cfg(test)]
mod l12 {
    use super::solution_1;
    use super::solution_2;

    #[test]
    fn solution_1_with_100k_numbers() {
        solution_1::print_pow2(&Vec::from_iter(1..=100_000));
    }

    #[test]
    fn soltion_1_with_1_number() {
        solution_1::print_pow2(&[1]);
    }

    #[test]
    fn solution_2_with_100k_numbers_5_workers() {
        solution_2::print_pow2(&Vec::from_iter(1..=100_000), 5);
    }

    #[test]
    #[should_panic]
    fn soltion_2_with_1_number_0_worker() {
        solution_2::print_pow2(&[1], 0);
    }
}
