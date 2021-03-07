/// 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 char[] 的形式给出
pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len / 2) {
        s.swap(i, len - i - 1);
    }
    // s.reverse();
}

#[test]
fn test_reverse_string() {
    let mut s = vec!['h', 'e',  'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'e', 'h']);
}
