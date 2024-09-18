use std::mem;

pub trait Sort {
    type Item;

    fn quicksort(&mut self);
}

impl<T> Sort for [T]
where
    T: Ord,
{
    type Item = T;

    // Legend:
    // - 'lt' is 'less than',
    // - 'gt' is 'greater than'.
    // - 'len' is self.len()
    fn quicksort(&mut self)
    where
        Self::Item: Ord,
    {
        // swap if first item is gt second one
        if let [a, b] = self {
            if a > b {
                mem::swap(a, b);
            }
        }
        // put items, which are lt pivot, at left side
        else if self.len() > 2 {
            let mut cur = 0;
            // init pivot position doesn't really matter,
            // let's select last item
            let mut piv = self.len() - 1;

            while cur < piv {
                if self[cur] <= self[piv] {
                    cur += 1;
                    continue;
                }

                let pre_piv = piv - 1;
                self.swap(cur, pre_piv);
                self.swap(pre_piv, piv);

                piv = pre_piv;
            }

            // 'if statements' can help to avoid redundant calls if len = 0 | 1,
            // but let's keep them for easy reading
            self[..piv].quicksort();
            self[piv..].quicksort();
        } else {
            // nothing to do if len == 0 | 1
        }
    }
}

#[cfg(test)]
mod l115 {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn numbers() {
        test(vec![-3, -4, 4, 1, 0, -5, 1, 5, -2, 2]);
        test(vec![u64::MAX, 0, u64::MIN, u64::MAX]);
    }

    #[test]
    fn strings() {
        test(vec![
            "cPdOh2GBUZ",
            "zzwebL3XOq",
            "ZStnPMy7Vy",
            "",
            "yQhPVJ2j15",
            "moXU7erARR",
            "zzwebL3XOq",
        ]);
    }

    /// Compare to result of built-in .sort_unstable() method.
    fn test<T>(mut values: Vec<T>)
    where
        T: Ord + /* required for testing */ Clone + Debug,
    {
        let sorted = {
            let mut nums = values.clone();
            nums.sort_unstable();
            nums
        };

        values.quicksort();
        assert_eq!(sorted, values);
    }
}
