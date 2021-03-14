
/// 汉明距离
/// 两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut n = x ^ y;
    let mut count = 0;
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}
fn a() -> i32 {
    println!("{:?}", 1);
    1
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(1,4), 2);
}

