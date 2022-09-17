use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count: HashMap<i32, usize> = HashMap::new();
    let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
    let mut res = Vec::new();
    for num in nums {
        let count = count.entry(num).or_insert(0);
        *count += 1;
    }
    for (key, val) in count {
        bucket[val].push(key);
    }
    'outer: for i in bucket.iter().rev() {
        for j in i {
            if res.len() as i32 == k {
                break 'outer;
            }
            res.push(*j);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mock = vec![1, 1, 1, 2, 2, 3];
        let mock_k = 2;
        let result = top_k_frequent(mock, mock_k);
        assert_eq!(result, vec![1, 2]);
    }
    #[test]
    fn example_2() {
        let mock = vec![1];
        let mock_k = 1;
        let result = top_k_frequent(mock, mock_k);
        assert_eq!(result, vec![1]);
    }
}
