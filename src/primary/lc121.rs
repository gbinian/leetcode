use std::cmp::{min, max};

/// 买卖股票的最佳时机
/// 给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。
/// 你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。
/// 返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_p = 0; // 最大利润
    let mut min_p = std::i32::MAX; // 当前最小价格
    for i in prices {
        max_p = max(i - min_p, max_p);
        min_p = min(i, min_p);
    }
    max_p
}

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
    assert_eq!(max_profit(vec![7,6,5,3,6,4]), 3);
}