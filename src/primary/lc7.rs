/// 给你一个 32 位的有符号整数 x ，返回 x 中每位上的数字反转后的结果。
/// 假设不能存储i64 溢出返回0
pub fn reverse(x: i32) -> i32 {
	x.abs()
		.to_string()
		.chars().rev()
		.collect::<String>()
		.parse::<i32>()
		.unwrap_or(0) * x.signum()
}


#[test]
fn test_reverse() {
	assert_eq!(reverse(123), 321);
	assert_eq!(reverse(-35), -53);
	
}