pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![1; nums.len()];
    let mut prefix = 1;
    for (i, n) in nums.iter().enumerate() {
        res[i] = prefix;
        prefix *= n;
    }
    let mut postfix = 1;
    for (i, n) in nums.iter().enumerate().rev() {
        res[i] *= postfix;
        postfix *= n;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mock = vec![1, 2, 3, 4];
        let result = product_except_self(mock);
        assert_eq!(result, vec![24, 12, 8, 6])
    }
}
