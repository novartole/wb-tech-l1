use std::cmp::Ordering::*;

pub fn binary_search<T>(values: &[T], value: &T) -> Option<usize>
where
    T: Ord,
{
    if values.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = values.len();

    while left < right {
        // better than (left + right) / 2,
        // which can lead to overflow
        let mid = left + (right - left) / 2;

        match values[mid].cmp(value) {
            Less => left = mid + 1,
            Equal => return Some(mid),
            Greater => right = mid,
        }
    }

    None
}

#[cfg(test)]
mod l116 {
    use super::*;

    /// Compare to result of built-in binary serach.
    fn test<T>(sorted_vals: &[T], value: T)
    where
        T: Ord,
    {
        assert_eq!(
            binary_search(sorted_vals, &value),
            sorted_vals.binary_search(&value).map(Some).unwrap()
        );
    }

    #[test]
    fn one_char() {
        let sorted_chars = ['&'].as_slice();

        assert_eq!(binary_search(sorted_chars, &'&'), Some(0));
        assert_eq!(binary_search(sorted_chars, &' '), None);
    }

    #[test]
    fn numbers() {
        let sorted_nums = [-5, -4, -3, -2, 0, 1, 1, 2, 4, 5].as_slice();

        // immidiate
        test(sorted_nums, 0);
        // left
        test(sorted_nums, -5);
        // rigth
        test(sorted_nums, 5);
        // left + 1
        test(sorted_nums, -4);
        // right - 1
        test(sorted_nums, 4);
        // not found
        assert_eq!(binary_search(sorted_nums, &10), None);
    }

    #[test]
    fn strs() {
        let sorted_str = ["2nk", "4js", "4ki", "acx", "cg4", "npc", "zva"].as_slice();

        // immidiate
        test(sorted_str, "acx");
        // left
        test(sorted_str, "2nk");
        // rigth
        test(sorted_str, "zva");
        // left + 1
        test(sorted_str, "4js");
        // right - 1
        test(sorted_str, "npc");
        // not found
        assert_eq!(binary_search(sorted_str, &""), None);
    }
}
