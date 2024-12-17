/// Split values ('vals') into two vectors.
///
/// The first vector contains result ranges as pairs:
/// - low bound of the range,
/// - values of the range, which are less than the high range (= low bound + 10.0).
///
/// The second vector contains values, which cannot be categorized: ±Infinity, NaN.
pub fn split(vals: &[f64]) -> (Vec<(f64, Vec<f64>)>, Vec<f64>) {
    // collect values of each range in map
    let mut map = std::collections::HashMap::new();
    // prepare finite values, and defer the rest till the end
    let (finite_vals, inf_and_nan_vals) =
        vals.iter().partition::<Vec<f64>, _>(|val| val.is_finite());
    for val in finite_vals {
        // use bits representation as key for map
        let bits = {
            let offset = if val.is_sign_negative() { -1.0 } else { 0.0 };
            let low_bound = (val / 10.0 + offset).trunc() * 10.0;
            low_bound.to_bits()
        };
        map.entry(bits)
            .and_modify(|vals: &mut Vec<_>| vals.push(val))
            .or_insert(vec![val]);
    }
    let ranges = map
        .into_iter()
        // go back from bits to value
        .map(|(bits, vals)| (f64::from_bits(bits), vals))
        .collect();
    (ranges, inf_and_nan_vals)
}

#[allow(dead_code)]
fn print_ranges(ranges: &[(f64, Vec<f64>)]) {
    for (low_bound, vals) in ranges {
        let high_bound = low_bound + 10.0;
        println!("[{}, {}): {:?}", low_bound, high_bound, vals);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let values = [-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
        let (mut ranges, skipped_values) = split(&values);
        ranges.sort_unstable_by(|(a, _), (b, _)| a.total_cmp(b));
        assert_eq!(
            ranges,
            &[
                (-30.0, vec![-25.4, -27.0, -21.0]),
                (10.0, vec![13.0, 19.0, 15.5]),
                (20.0, vec![24.5]),
                (30.0, vec![32.5])
            ]
        );
        assert_eq!(skipped_values, &[]);
        print_ranges(&ranges);
    }

    #[test]
    fn subnormal() {
        let values = [
            f64::MIN_POSITIVE - f64::MIN_POSITIVE / 2.0,
            -f64::MIN_POSITIVE + f64::MIN_POSITIVE / 3.0,
        ];
        let (mut ranges, skipped_values) = split(&values);
        ranges.sort_unstable_by(|(a, _), (b, _)| a.total_cmp(b));
        assert_eq!(
            ranges,
            &[
                (-10.0, vec![-f64::MIN_POSITIVE + f64::MIN_POSITIVE / 3.0]),
                (0.0, vec![f64::MIN_POSITIVE - f64::MIN_POSITIVE / 2.0]),
            ]
        );
        assert_eq!(skipped_values, &[]);
        print_ranges(&ranges);
    }

    #[test]
    fn infinity_and_nan() {
        let values = [f64::INFINITY, f64::NEG_INFINITY, f64::NAN];
        let (ranges, skipped_values) = split(&values);
        assert_eq!(ranges, &[]);
        for (a, b) in values.into_iter().zip(skipped_values) {
            assert_eq!(a.total_cmp(&b), std::cmp::Ordering::Equal);
        }
    }
}
