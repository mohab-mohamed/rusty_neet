use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash_set: HashSet<i32> = HashSet::new();

    for num in nums {
        if hash_set.contains(&num) {
            return true;
        }
        hash_set.insert(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_duplicate() {
        let mock_vec = vec![1, 2, 2, 3, 4, 5];
        let result = contains_duplicate(mock_vec);
        assert_eq!(result, true);
    }

    #[test]
    fn no_duplicate() {
        let mock_vec = vec![1, 2, 3, 4, 5];
        let result = contains_duplicate(mock_vec);
        assert_eq!(result, false);
    }
}
