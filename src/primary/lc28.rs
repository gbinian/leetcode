/// 实现 strStr()
/// 给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如果不存在，则返回 -1
pub fn str_str(haystack: String, needle: String) -> i32 {
    // if let Some(n) = haystack.find(&needle) {
    // 	n as i32
    // } else {
    // 	-1
    // }
    if needle.is_empty() {
        0
    } else if needle.len() > haystack.len() {
        -1
    } else {
        let (mut p1, mut p2, mut i) = (0, 0, -1);
        let b1 = haystack.as_bytes();
        let b2 = needle.as_bytes();
        while p1 < b1.len() && p2 < b2.len() {
            if b1[p1] == b2[p2] {
                if i == -1 {
                    i = p1 as i32;
                }
                p1 += 1;
                p2 += 1;
            } else {
                i = -1;
                p1 = p1 - p2 + 1;
                p2 = 0;
            }
            dbg!(&p1, &p2, &i);
        }
        if p2 == needle.len() {
            i
        } else {
            -1
        }
    }
}

#[test]
fn test_str_str() {
    assert_eq!(str_str("hello".into(), "ll".into()), 2);
    assert_eq!(str_str("hello".into(), "".into()), 0);
    assert_eq!(str_str("hello".into(), "p".into()), -1);
    assert_eq!(str_str("mississippi".into(), "issip".into()), 4);
}