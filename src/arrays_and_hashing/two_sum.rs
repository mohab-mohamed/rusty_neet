use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
    let mut complement_map: HashMap<&i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if complement_map.contains_key(&complement) {
            return (i, *complement_map.get(&complement).unwrap());
        }
        complement_map.insert(num, i);
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_case_1() {
        let mock_nums = vec![2, 7, 11, 15];
        let mock_target = 9;
        let result = two_sum(mock_nums, mock_target);
        assert_eq!(result, (1, 0));
    }

    #[test]
    fn two_sum_case_two() {
        let mock_nums = vec![3, 2, 4];
        let mock_target = 6;
        let result = two_sum(mock_nums, mock_target);
        assert_eq!(result, (2, 1));
    }
}
