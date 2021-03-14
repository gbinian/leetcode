
/// 买卖股票的最佳时机 II
/// 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
///
/// 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
///
/// 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

pub fn max_profit(prices: Vec<i32>) -> i32 {
    // use std::cmp::max;
    // if prices.len() < 2 {
    //     0
    // } else {
    //     let mut res: Vec<(i32, i32)> = Vec::new();
    //     res.push((0, -*prices.first().unwrap()));
    //     for price in prices {
    //         let prev = *res.last().unwrap();
    //         res.push((
    //             max(prev.0, prev.1 + price),
    //             max(prev.1, prev.0 - price),
    //             ));
    //     }
    //     res.last().unwrap().0
    // }
    prices.windows(2).map(|a|{
        if a[1] > a[0] { a[1] - a[0] } else { 0 }
    }).sum()
}

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(vec![1,2,3,4,5]), 4);
    assert_eq!(max_profit(vec![7,1,5,3,6,4]), 7);
}