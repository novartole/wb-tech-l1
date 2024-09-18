fn main() {
    let mut args = std::env::args();
    let process = args.next().unwrap();
    let string = args
        .next()
        .unwrap_or_else(|| panic!(r#"expected string; example: {} "some string""#, process));

    println!("{} - {}", string, reverse_words(&string));
}

fn reverse_words(str: &str) -> String {
    str
        // take into account Unicode whitespaces
        .split_whitespace()
        // result of two or more whitespaces between words
        .filter(|word| !word.is_empty())
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod l119 {
    use super::*;

    #[test]
    fn one_word() {
        assert_eq!(reverse_words(""), String::new());
        assert_eq!(reverse_words("snow"), "snow".to_owned());
    }

    #[test]
    fn example_from_description() {
        assert_eq!(reverse_words("snow dog sun"), "sun dog snow".to_owned());
    }

    #[test]
    fn many_whitespaces() {
        assert_eq!(
            reverse_words("   snow     dog      sun   "),
            "sun dog snow".to_owned()
        );
    }
}
