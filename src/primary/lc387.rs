
/// 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1
pub fn first_uniq_char(s: String) -> i32 {
	use std::collections::HashMap;
	s.chars().enumerate()
		.fold(HashMap::new(), |mut m, (i, c)|{
			m.entry(c).and_modify(|(_, e)| *e += 1 ).or_insert((i, 1));
			m
	}).values().fold(-1, |mut res, &(i,v) |{
		if v == 1 && (res == -1 || res > (i as i32))  {
			res = i as i32;
			res
		} else { res }
	})
}

#[test]
fn test_first_uniq_char() {
	let s = "leetcode";
	assert_eq!(first_uniq_char(s.into()), 0)
}