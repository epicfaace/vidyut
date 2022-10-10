/// Splits Sanskrit expressions according to a list of sandhi rules.
use multimap::MultiMap;
use regex::Regex;
use std::cmp;

pub type SandhiMap = MultiMap<String, (String, String)>;

/// Returns all possible splits for the given input.
pub fn split(input: &str, rules: &SandhiMap) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let len_longest_key = rules.keys().map(|x| x.len()).max().expect("Map is empty");
    let len_input = input.len();

    // When iterating, prefer making the first item as long as possible, as longer
    // items are easier to rule out.
    for i in (1..=len_input).rev() {
        // Default: split as-is, no sandhi.
        res.push((
            String::from(&input[0..i]),
            String::from(&input[i..len_input]),
        ));

        for j in i..cmp::min(len_input, i + len_longest_key + 1) {
            let combination = &input[i..j];
            // println!("{}-{} : {}", i, j, combination);
            match rules.get_vec(combination) {
                Some(pairs) => {
                    for (f, s) in pairs {
                        let first = String::from(&input[0..i]) + f;
                        let second = String::from(s) + &input[j..len_input];
                        res.push((first, second))
                    }
                }
                None => continue,
            }
        }
    }
    res
}

/// Returns whether the first item in a sandhi split is OK according to some basic heuristics.
fn is_good_first(text: &str) -> bool {
    match text.chars().last() {
        // Vowels, standard consonants, and "s" and "r"
        Some(c) => "aAiIuUfFxXeEoOHkNwRtpnmsr".contains(c),
        None => true,
    }
}

/// Returns whether the second item in a sandhi split is OK according to some basic heuristics.
fn is_good_second(text: &str) -> bool {
    // Initial yrlv must not be followed by sparsha.
    let r = Regex::new(r"^[yrlv][kKgGNcCjJYwWqQRtTdDnpPbBm]").unwrap();
    !r.is_match(text)
}

/// Returns whether a given sandhi split is OK according to some basic heuristics.
///
/// Our sandhi splitting logic overgenerates, and some of its outputs are not phonetically valid.
/// For most use cases, we recommend filtering the results of `split` with this function.
pub fn is_good_split(text: &str, first: &str, second: &str) -> bool {
    // To avoid recursion, require that `second` is not just a repeat of the inital state.
    let is_recursive = text == second;
    is_good_first(first) && is_good_second(second) && !is_recursive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let mut rules = SandhiMap::new();
        rules.insert("e".to_string(), ("a".to_string(), "i".to_string()));
        let expected: Vec<(String, String)> = vec![
            ("ceti", ""),
            ("cet", "i"),
            ("ce", "ti"),
            ("c", "eti"),
            ("ca", "iti"),
        ]
        .iter()
        .map(|&(f, s)| (f.to_string(), s.to_string()))
        .collect();

        assert_eq!(split("ceti", &rules), expected);
    }

    #[test]
    fn test_is_good_first() {
        for word in vec![
            "rAma", "rAjA", "iti", "nadI", "maDu", "gurU", "pitf", "F", "laBate", "vE", "aho",
            "narO", "naraH", "vAk", "rAw", "prAN", "vit", "narAn", "anuzWup", "naram",
        ] {
            assert!(is_good_first(word));
        }
        for word in vec!["PalaM", "zaz", "vAc"] {
            assert!(!is_good_first(word));
        }
    }

    #[test]
    fn test_has_valid_start() {
        for word in vec![
            "yogena",
            "rAma",
            "leKaH",
            "vE",
            "kArtsnyam",
            "vraja",
            "vyajanam",
        ] {
            assert!(is_good_second(word));
        }
        for word in vec!["rmakzetre"] {
            assert!(!is_good_second(word));
        }
    }
}
