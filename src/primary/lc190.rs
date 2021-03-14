/// 颠倒二进制位
/// 颠倒给定的 32 位无符号整数的二进制位。翻转二进制数
pub fn reverse_bits(x: u32) -> u32 {
    let mut res = 0;
    let mut n = x;
    let mut count = 31;
    while n > 0 {
        res += (n & 1) << count; // 获取最后一位数字 移位到正确位置
        n >>= 1; // 右移一位， 去掉末尾数字
        count -= 1;
    }
    res
}

#[test]
fn test_reverse_bits() {
    assert_eq!(reverse_bits(43261596),964176192)
}