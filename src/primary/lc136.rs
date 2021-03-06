/// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in nums {
        res ^= i;
    }
    res
}

#[test]
fn test_single_number() {
    let nums = vec![1, 2, 2, 3, 3];
    assert_eq!(single_number(nums), 1);
}
