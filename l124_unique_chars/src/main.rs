/*
* Разработать программу, которая проверяет,
* что все символы в строке уникальные (true — если уникальные, false etc).
* Функция проверки должна быть регистронезависимой.
* Например: abcd — true abCdefAaf — false aabcd — false
*/

fn main() {
    let mut args = std::env::args();
    let process = args.next().unwrap();
    let string = args
        .next()
        .unwrap_or_else(|| panic!(r#"expected string; example: {} "some string""#, process));

    if string.chars().all(|ch| ch.is_ascii_alphabetic()) {
        println!(
            "solution 1 (ASCII): {} - {}",
            string,
            solution_1::only_unique_chars(&string)
        );
    } else {
        println!("[warn]: solution 1 supports only ASCII alphabetic");
    }

    println!(
        "solution 2 (Unicode): {} - {}",
        string,
        solution_2::only_unique_chars(&string)
    );
}

// ASCII abc
mod solution_1 {
    pub fn only_unique_chars(str: &str) -> bool {
        assert!(
            str.chars().all(|ch| ch.is_ascii_alphabetic()),
            "expected only ASCII alphabetic"
        );

        // let's count only [A..Za..z] (26 x 2)
        let mut abc = [0; 52];

        for ch in str.chars() {
            // 6 chars between 'Z' and 'a' lead to offset
            let offset = if ch.is_ascii_uppercase() { 0 } else { 6 };
            // ASCII can safely be represented as u8
            abc[(ch as u8 - b'A' - offset) as usize] += 1;
        }

        abc.into_iter().all(|n| n <= 1)
    }
}

// Unicode
mod solution_2 {
    use std::collections::HashMap;

    pub fn only_unique_chars(str: &str) -> bool {
        let mut ch_counts = HashMap::new();

        for ch in str.chars() {
            ch_counts
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        ch_counts.into_values().all(|count| count <= 1)
    }
}

#[cfg(test)]
mod l124 {
    use super::{solution_1, solution_2};

    #[test]
    fn solution_1_from_description() {
        assert!(solution_1::only_unique_chars("abcd"));
        assert!(!solution_1::only_unique_chars("abCdefAaf"));
    }

    #[test]
    fn solution_2_from_description() {
        assert!(solution_2::only_unique_chars("abcd"));
        assert!(!solution_2::only_unique_chars("abCdefAaf"));
    }

    #[test]
    fn solution_2_en_ru_zh_symb_emoji() {
        assert!(solution_2::only_unique_chars("Gы_=圭👾"));
        assert!(!solution_2::only_unique_chars("Gы_=🚴圭👾🚴"));
    }
}
