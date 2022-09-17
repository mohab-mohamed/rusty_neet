use std::collections::HashMap;

pub fn get_char_string(str: String) -> String {
    let mut count: [u32; 26] = [0; 26];

    for c in str.chars() {
        count[c as usize - 'a' as usize] += 1;
    }
    let mut key = String::new();
    count
        .iter()
        .for_each(|n| key.push(char::from_digit(*n, 10).unwrap()));
    key
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    vec![
        vec![String::from("bat")],
        vec![String::from("nat"), String::from("tan")],
        vec![
            String::from("ate"),
            String::from("eat"),
            String::from("tea"),
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_string() {
        let mock = String::from("abc");
        let result = get_char_string(mock);
        assert_eq!(result, "11100000000000000000000000")
    }

    #[test]
    fn example_1() {
        let mock = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result = group_anagrams(mock);
        assert_eq!(
            result,
            vec![
                vec![String::from("bat")],
                vec![String::from("nat"), String::from("tan")],
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ]
            ]
        );
    }
}
