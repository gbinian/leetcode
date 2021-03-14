use std::cmp::max;

/// 最大子序和
/// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut v =  Vec::with_capacity(nums.len());
    let mut sum = 0;
    for i in nums {
        if v.is_empty() {
            v.push(i);
            sum = i;
        } else {
            let last = *v.last().unwrap();
            if i > (last + i) {
                v.push(i);
                sum = max(sum, i);
            } else {
                v.push(last + i);
                sum = max(sum, last + i);
            }
        }
    }
    dbg!(v);
    sum
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(max_sub_array(vec![1, 2]), 3)
}