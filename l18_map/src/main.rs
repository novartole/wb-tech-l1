fn main() {
    let nums = &Vec::from_iter(0..7);

    println!("# solution 2");
    let result = solution_1::run_with_mutex_and_hashmap(nums, 5);
    println!("{:?}", result);

    println!();

    println!("# solution 2");
    let result = solution_2::run_with_dashmap(nums, 5);
    println!("{:?}", result);
}

mod solution_1 {
    use std::{
        collections::HashMap,
        sync::{Arc, Barrier, Mutex},
    };

    pub fn run_with_mutex_and_hashmap(nums: &[i32], workers: usize) -> HashMap<i32, usize> {
        assert_ne!(workers, 0, "at least 1 worker is expected");

        let map = Arc::new(Mutex::new(HashMap::with_capacity(nums.len())));
        // let's add some fairplay to sync workers at the start
        let barrier = Arc::new(Barrier::new(workers));

        std::thread::scope(|scp| {
            for _worker_id in 0..workers {
                scp.spawn({
                    let map_ = Arc::clone(&map);
                    let barrier_ = Arc::clone(&barrier);

                    move || {
                        println!("[{}]: wait for sync with others", _worker_id);
                        barrier_.wait();

                        println!("[{}]:   let's go!", _worker_id);
                        for &num in nums {
                            println!("[{}]: acquire the lock", _worker_id);
                            let mut map = map_.lock().unwrap();

                            println!("[{}]:   got the lock", _worker_id);
                            map.entry(num).and_modify(|val| *val += 1).or_insert(1);

                            println!("[{}]: released the lock", _worker_id);
                        }
                    }
                });
            }
        });

        Arc::into_inner(map).unwrap().into_inner().unwrap()
    }
}

mod solution_2 {
    use std::sync::{Arc, Barrier};

    use dashmap::DashMap;

    pub fn run_with_dashmap(nums: &[i32], workers: usize) -> DashMap<i32, usize> {
        assert_ne!(workers, 0, "at least 1 worker is expected");

        let map = Arc::new(DashMap::with_capacity(nums.len()));
        let barrier = Arc::new(Barrier::new(workers));

        std::thread::scope(|scp| {
            for _worker_id in 0..workers {
                scp.spawn({
                    let map_ = Arc::clone(&map);
                    let barrier_ = Arc::clone(&barrier);

                    move || {
                        println!("[{}]: wait for sync with others", _worker_id);
                        barrier_.wait();

                        println!("[{}]:   let's go!", _worker_id);
                        for &num in nums {
                            println!("[{}]:   try to modfy", _worker_id);
                            map_.entry(num).and_modify(|val| *val += 1).or_insert(1);

                            println!("[{}]: done", _worker_id);
                        }
                    }
                });
            }
        });

        Arc::into_inner(map).unwrap()
    }
}

#[cfg(test)]
mod l18 {
    use super::solution_1;
    use super::solution_2;

    #[test]
    fn solution_1_with_10k_numbers_and_3_workers() {
        let nums = &Vec::from_iter(0..10_000);
        let workers = 3;

        let map = solution_1::run_with_mutex_and_hashmap(nums, workers);
        assert!(!map.is_empty() && map.into_values().all(|count| count == workers));
    }

    #[test]
    fn solution_2_with_100k_numbers_and_5_workers() {
        let nums = &Vec::from_iter(0..100_000);
        let workers = 5;

        let map = solution_2::run_with_dashmap(nums, workers);
        assert!(!map.is_empty() && map.into_iter().all(|(_, count)| count == workers));
    }
}
