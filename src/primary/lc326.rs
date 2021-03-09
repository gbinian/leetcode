/// 3 的幂
/// 给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。
pub fn is_power_of_three(n: i32) -> bool {
    return n > 0 && 1162261467 % n == 0;
}

#[test]
fn test_is_power_of_three() {
    assert_eq!(is_power_of_three(27), true);
}