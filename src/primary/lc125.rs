/// 验证回文串
/// 给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
pub fn is_palindrome(s: String) -> bool {
    if s.len() <= 1 {
        true
    } else {
        let (mut p1, mut p2) = (0, s.len() - 1);
        let v: Vec<char> = s.chars().collect();
        while p2 > p1 {
            // 检验有效
            if !v[p1].is_ascii_alphanumeric() {
                p1 += 1;
                continue;
            }
            if !v[p2].is_ascii_alphanumeric() {
                p2 -= 1;
                continue;
            }
            if v[p1].to_ascii_lowercase() == v[p2].to_ascii_lowercase() {
                p1 += 1;
                p2 -= 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_is_palindrome() {
    let s = "a.";
    assert_eq!(is_palindrome(s.into()), true);
}
