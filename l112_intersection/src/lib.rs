#![allow(dead_code)]

// required Eq + Hash
mod solution_1 {
    use std::{collections::HashSet, hash::Hash};

    pub fn intersect<'a, T>(a: &'a [T], b: &'a [T]) -> Vec<&'a T>
    where
        T: Eq + Hash,
    {
        let a_set = {
            let mut set = HashSet::with_capacity(a.len());
            set.extend(a);
            set
        };

        let b_set = {
            let mut set = HashSet::with_capacity(a.len());
            set.extend(b);
            set
        };

        a_set.intersection(&b_set).copied().collect()
    }
}

// naive implementation
mod solution_2 {
    pub fn intersect<'c, T>(mut a: &'c [T], mut b: &'c [T]) -> impl Iterator<Item = &'c T>
    where
        T: PartialEq,
    {
        if a.len() < b.len() {
            (a, b) = (b, a);
        }

        a.iter().filter(|t| b.contains(t))
    }
}

#[cfg(test)]
mod l112 {
    use super::{solution_1, solution_2};

    #[test]
    fn solution_1_strings() {
        let s = String::from("qaz");

        let a = ["asd", "zxc", "234", s.as_str()];
        let b = ["123", "234", s.as_str(), "345"];
        let c = solution_1::intersect(&a, &b);

        assert_eq!(c.len(), 2);
        assert!(c.contains(&&"234"));
        assert!(c.contains(&&s.as_str()));
    }

    #[test]
    fn solution_1_i32() {
        let a = [-2, 0, 2, 4];
        let b = [-3, 0, 1, 4];
        let c = solution_1::intersect(&a, &b);

        assert_eq!(c.len(), 2);
        assert!(c.contains(&&0));
        assert!(c.contains(&&4));
    }

    #[test]
    fn solution_2_f64() {
        let a = [-3.12f64, 0.1, 10.0, 10.0 + 1e-14];
        let b = [-2.2, 0.1, 2.1, 4.3, 10.0 - 1e-14];
        let c = Vec::from_iter(solution_2::intersect(&a, &b));

        assert_eq!(c.len(), 1);
        assert!(c.contains(&&a[1]));
        assert!(c.contains(&&b[1]));
    }
}
