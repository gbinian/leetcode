use std::ops::AddAssign;
/// 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。
/// 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
/// 你可以假设除了整数 0 之外，这个整数不会以零开头。
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = digits.clone();
    res.last_mut().unwrap().add_assign(1);
    let len = res.len();
    for i in (0..=len - 1).rev() {
        if res[i] > 9 {
            res[i] -= 10;
            if i == 0 {
                res.insert(0, 1)
            } else {
                res[i - 1] += 1;
            }
        }
    }
    res
}

#[test]
fn test_plus_one() {
    let digits = vec![9];
    assert_eq!(plus_one(digits), vec![1, 0])
}
