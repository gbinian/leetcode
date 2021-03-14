/// 缺失数字
/// 给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。

pub fn missing_number(nums: Vec<i32>) -> i32 {
    (1 + nums.len() as i32) * nums.len() as i32 / 2
    -
    nums.iter().fold(0, |a, b| a + * b)
}

#[test]
fn test_missing_number() {
    assert_eq!(missing_number(vec![0,2,3,4,5]), 1)
}
