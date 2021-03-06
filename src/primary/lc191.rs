/// 位1的个数
/// 编写一个函数，输入是一个无符号整数（以二进制串的形式），返回其二进制表达式中数字位数为 '1' 的个数（也被称为汉明重量）
pub fn hamming_weight(n: u32) -> i32 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}

#[test]
fn test_hamming_weight() {
    assert_eq!(31, hamming_weight(4294967293))
}