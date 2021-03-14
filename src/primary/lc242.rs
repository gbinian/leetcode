/// 有效的字母异位词
/// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
pub fn is_anagram(s: String, t: String) -> bool {
    let mut m: std::collections::HashMap<char, i32> =
        ('a'..='z').map(move |a| (a, 0_i32)).collect();
    if s.len() != t.len() {
        false
    } else {
        // 统计字符个数
        for x in s.chars() {
            let v = m.get_mut(&x).unwrap();
            *v += 1;
        }
        // 减去字符个数
        for x in t.chars() {
            let v = m.get_mut(&x).unwrap();
            *v -= 1;
        }
        m.values().all(|a| *a == 0_i32)
    }
}

#[test]
fn test_is_anagram() {
    let s = "rat";
    let t = "car";
    assert_eq!(is_anagram(s.into(), t.into()), false);
}