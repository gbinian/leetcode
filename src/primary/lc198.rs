use std::cmp::max;

/// 打家劫舍
/// 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
/// 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
///
/// dp[k] = max(dp[k - 1], H<sub>k-1</sub> + dp[k - 2])
///
/// 0, 1 -> 0, k<sub>1</sub>
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        if let Some(n) = nums.get(0) { *n } else { 0 }
    } else{
        let mut v = vec![0];
        v.push(nums[0].clone());
        for (i, n) in nums.iter().enumerate().skip(1) {
            v.push(max(v[i], *n + v[i - 1]));
        }
        *v.last().unwrap()
    }
}

#[test]
fn test_rob() {
    assert_eq!(rob(vec![2,7,9,3,1]), 12)
}