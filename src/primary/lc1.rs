/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 的那 两个 整数，并返回它们的数组下标。
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut res = Vec::with_capacity(2);
    let mut m: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    nums.iter().enumerate().find(|&(i, &v)| {
        if m.contains_key(&(target - v)) {
            res.push(m.get(&(target - v)).unwrap().to_owned() as i32);
            res.push(i as i32);
            true
        } else {
            m.insert(v, i);
            false
        }
    });
    res
}

#[test]
fn test_two_sum() {
    let nums = vec![1, 3, 6];
    assert_eq!(two_sum(nums, 9), vec![1, 2]);
}
