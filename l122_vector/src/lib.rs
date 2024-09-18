#![allow(dead_code)]

// obvious, but why not
mod solution_1 {
    pub fn remove<T>(slice: &mut [Option<T>], i: usize) {
        slice.get_mut(i).expect("index must be in range").take();
    }
}

// similar how rust does it, but less optimized way
mod solution_2 {
    pub fn remove_unstable<T>(vec: &mut Vec<T>, i: usize) {
        assert!(i < vec.len() && !vec.is_empty(), "index must be in range");

        let last = vec.len() - 1;
        vec.swap(i, last);
        vec.truncate(last);
    }
}

// playing with low lowel stuff to keep order
mod solution_3 {
    use std::ptr;

    pub fn remove<T>(vec: &mut Vec<T>, i: usize)
    where
        // Required for soundness.
        // At least in case of self-ref structures
        // such moving may lead to broken data.
        T: Unpin,
    {
        assert!(i < vec.len() && !vec.is_empty(), "index must be in range");

        let last = vec.len() - 1;

        if i < last {
            let src = &vec[i + 1] as *const _;
            let dst = &mut vec[i] as *mut _;
            let count = last - i;

            unsafe {
                // Safety: value hasn't been moved yet.
                ptr::drop_in_place(dst);
                // Safety: there is enough space to move values left.
                ptr::copy(src, dst, count);
                // Safety: len is big enough to be shorten.
                vec.set_len(vec.len() - 1);
            }
        } else {
            vec.truncate(last)
        }
    }
}

#[cfg(test)]
mod l122 {
    use std::cell::Cell;

    use super::{solution_1, solution_2, solution_3};

    #[test]
    fn solution_1_remove_first_mid_last() {
        let vec = &mut Vec::from_iter((0..=7).map(Some));

        solution_1::remove(vec, 0);
        assert_eq!(vec[0], None);

        solution_1::remove(vec, 3);
        assert_eq!(vec[3], None);

        solution_1::remove(vec, 7);
        assert_eq!(vec[7], None);
    }

    #[test]
    fn solution_2_remove_first_mid_last() {
        let vec = &mut vec![0, 1, 2, 3, 4, 5, 6, 7];

        solution_2::remove_unstable(vec, 0);
        assert_eq!(vec, &[7, 1, 2, 3, 4, 5, 6]);

        solution_2::remove_unstable(vec, 3);
        assert_eq!(vec, &[7, 1, 2, 6, 4, 5]);

        solution_2::remove_unstable(vec, 5);
        assert_eq!(vec, &[7, 1, 2, 6, 4]);
    }

    #[test]
    fn solution_3_remove_first_mid_last() {
        let vec = &mut vec![0, 1, 2, 3, 4, 5, 6, 7];

        solution_3::remove(vec, 0);
        assert_eq!(vec, &[1, 2, 3, 4, 5, 6, 7]);

        solution_3::remove(vec, 3);
        assert_eq!(vec, &[1, 2, 3, 5, 6, 7]);

        solution_3::remove(vec, 5);
        assert_eq!(vec, &[1, 2, 3, 5, 6]);
    }

    #[test]
    fn solution_3_check_drop_removing_5th() {
        struct Struct<'a> {
            count: &'a Cell<usize>,
        }

        impl<'a> Struct<'a> {
            fn new(count: &'a Cell<usize>) -> Self {
                count.set(count.get() + 1);

                Self { count }
            }
        }

        impl Drop for Struct<'_> {
            fn drop(&mut self) {
                let count = self.count.get();
                self.count.set(count - 1);
            }
        }

        let n = 7;
        let counter = Cell::new(0);

        let vec = &mut Vec::from_iter((1..=7).map(|_| Struct::new(&counter)));
        assert_eq!(counter.get(), n);

        solution_3::remove(vec, 5);
        assert_eq!(counter.get(), n - 1);
    }
}
