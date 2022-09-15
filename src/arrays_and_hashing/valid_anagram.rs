use std::collections::HashMap;

pub fn valid_anagram(s: &str, t: &str) -> bool {
    if s.chars().count() != t.chars().count() {
        return false;
    }

    let mut s_map: HashMap<char, u32> = HashMap::new();
    let mut t_map: HashMap<char, u32> = HashMap::new();
    for char in s.chars() {
        let count = s_map.entry(char).or_insert(0);
        *count += 1;
    }
    for char in t.chars() {
        let count = t_map.entry(char).or_insert(0);
        *count += 1;
    }

    if s_map.len() != t_map.len() {
        return false;
    }
    for (key, _) in &s_map {
        if s_map.get(&key) != t_map.get(&key) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram() {
        let mock_s = "anagram";
        let mock_t = "nagaram";
        let result = valid_anagram(mock_s, mock_t);
        assert_eq!(result, true);
    }

    #[test]
    fn not_anagram() {
        let mock_s = "anagram";
        let mock_t = "uagaram";
        let result = valid_anagram(mock_s, mock_t);
        assert_eq!(result, false);
    }
}
