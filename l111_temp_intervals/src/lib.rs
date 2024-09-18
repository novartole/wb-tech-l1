// No solution: restrictions are too weak.
// It's easy to create a vec of f64,
// whose elements DON'T behave as expected.
#[allow(dead_code)]
fn explanation() {
    // 1st: precision
    {
        // values from description
        let mut temps = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
        let mut ints = split_into_intervals(&temps);
        // this values don't give any problem
        assert_eq!(ints[0].0, "[-30, -20)");
        assert_eq!(ints[0].1.len(), 3);
        assert_eq!(ints[3].0, "[30, 40)");
        assert_eq!(ints[3].1.len(), 1);

        // Let's add a few values from [-30, 40) range:
        temps.extend([
            // should _defenetly_ go to [-30, -20):
            //   -20.000000000000001 < -20.0
            -20.0 - 1e-15,
            // should _defenetly_ go to [30, 40):
            //   39.999999999999999 < 40.0
            40.0 - 1e-15,
        ]);

        ints = split_into_intervals(&temps);
        // The following asserts pass,
        // that means -20.0 - 1e-15 went into [-20, -10):
        assert_eq!(ints[1].0, "[-20, -10)");
        assert_eq!(ints[1].1.len(), 1);
        assert_eq!(ints[0].0, "[-30, -20)");
        assert_ne!(ints[0].1.len(), 4);

        // and similar for 40 - 1e-15
        assert_eq!(ints[4].0, "[30, 40)");
        assert_eq!(ints[4].1.len(), 1);
    }

    // 2nd: too big
    {
        let mut big_val = 1e54_f64;
        for _ in 0..1_000_000 {
            assert!((big_val % 10.0).abs() > f64::EPSILON);
            big_val -= 1.0;
        }
    }
}

/// Go from [-30, -20) to [30, 40) and fill _result_ intervals.
fn split_into_intervals(temps: &[f64]) -> Vec<(String, Vec<f64>)> {
    let mut result = vec![];

    for (left, right) in (-30..=30)
        .step_by(10)
        .zip((-20..=40).step_by(10))
        .map(|(left, right)| (left as f64, right as f64))
    {
        let temps = Vec::from_iter(
            temps
                .iter()
                .copied()
                .filter(|&temp| left <= temp && temp < right),
        );

        if !temps.is_empty() {
            result.push((format!("[{}, {})", left as i32, right as i32), temps));
        }
    }

    result
}

#[cfg(test)]
mod l111 {
    #[test]
    fn prove() {
        super::explanation();
    }
}
