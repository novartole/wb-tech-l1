fn main() {
    let mut args = std::env::args();
    let process = args.next().unwrap();
    let string = args
        .next()
        .unwrap_or_else(|| panic!(r#"expected string; example: {} "some string""#, process));

    println!("{} - {}", string, string.reverse());
}

trait Reverse
where
    Self: ToOwned,
{
    fn reverse(&self) -> Self::Owned;
}

impl Reverse for str {
    fn reverse(&self) -> Self::Owned {
        self.chars().rev().collect()
        // equals to _self.chars().rev().collect::<Self::Owned>()_,
        // which equals to _self.chars().rev().collect::<String>()_,
    }
}

#[cfg(test)]
mod l118 {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!("".reverse(), String::new());
    }

    #[test]
    fn example_from_description() {
        assert_eq!("главрыба".reverse(), "абырвалг".to_owned());
    }

    #[test]
    fn ru_en_zh() {
        assert_eq!("ыq你".reverse(), "你qы".to_owned());
    }
}
