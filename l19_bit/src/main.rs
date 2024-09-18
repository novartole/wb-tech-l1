fn main() {
    let (num, pos, bit) = parse_args();
    let res = set_bit(num, pos, bit);
    print(num, pos, bit, res);
}

#[derive(Clone, Copy)]
enum Bit {
    Zero,
    One,
}

fn set_bit(num: i64, pos: u8, bit: Bit) -> i64 {
    match bit {
        Bit::Zero => (1 << pos ^ -1) & num,
        Bit::One => 1 << pos | num,
    }
}

fn print(num: i64, pos: u8, bit: Bit, res: i64) {
    println!(
        "num = {0} ({0:04$b}),\n\
        pos = {1},\n\
        bit = {2},\n\
        res = {3} ({3:04$b})",
        num, pos, bit as u8, res, 64
    );
}

/// # Panic
/// A missed argument or wrong type causes panic.
fn parse_args() -> (i64, u8, Bit) {
    let mut args = std::env::args().skip(1);

    let num = args
        .next()
        .expect("missed: <number: i64>, _, _")
        .parse::<i64>()
        .expect("expected: <number: i64>, _, _");

    let pos = match args
        .next()
        .expect("missed: _, <position: 0..=63>, _")
        .parse::<u8>()
        .expect("expected: _, <position: 0..=63>, _")
    {
        val @ 0..=63 => val,
        wrong_val => panic!("expected: _, <position: 0..=63>, _, but got {}", wrong_val),
    };

    let bit = match args
        .next()
        .expect("missed: _, _, <bit: 0 | 1>")
        .parse::<u8>()
        .expect("expected: _, _, <bit: 0 | 1>")
    {
        0 => Bit::Zero,
        1 => Bit::One,
        wrong_val => panic!("expected: _, _, <bit: 0 | 1>, but got {}", wrong_val),
    };

    (num, pos, bit)
}

#[cfg(test)]
mod l19 {
    use super::{set_bit, Bit};

    #[test]
    fn set_last_be() {
        assert_eq!(set_bit(-1, 63, Bit::Zero), i64::MAX);
        assert_eq!(set_bit(0, 63, Bit::One), i64::MIN);
    }

    #[test]
    fn set_first_be() {
        assert_eq!(set_bit(1, 0, Bit::Zero), 0);
        assert_eq!(set_bit(8, 0, Bit::One), 9);
    }

    #[test]
    fn set_between_first_and_last_be() {
        assert_eq!(set_bit(9, 3, Bit::Zero), 1);
        assert_eq!(set_bit(-3, 1, Bit::One), -1);
    }
}
