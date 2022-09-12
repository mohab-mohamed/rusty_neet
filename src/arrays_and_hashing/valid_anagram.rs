use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
    let mut char_map: HashMap<char, u32> = HashMap::new();

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram() {
        let mock_s = String::from("anagram");
        let mock_t = String::from("nagaram");
        let result = valid_anagram(mock_s, mock_t);
        assert_eq!(result, true);
    }

    #[test]
    fn not_anagram() {
        let mock_s = String::from("anagram");
        let mock_t = String::from("nagaram");
        let result = valid_anagram(mock_s, mock_t);
        assert_eq!(result, false);
    }
}
