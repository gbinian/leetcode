/// 有效的括号
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
// 有效字符串需满足：
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
pub fn is_valid(s: String) -> bool {
    let mut v = vec![];
    for a in s.chars() {
        match a {
            '(' | '{' | '[' => v.push(a),
            ')' => {
                match v.last() {
                    Some('(') => v.pop(),
                    _ => return false
                };
            },
            '}' => {
                match v.last() {
                    Some('{') => v.pop(),
                    _ => return false
                };
            },
            ']' => {
                match v.last() {
                    Some('[') => v.pop(),
                    _ => return false
                };
            },
            _ => unreachable!()
        };
    };
    v.is_empty()
}

#[test]
fn test_is_valid() {
    assert_eq!(is_valid("(({}))".to_string()), true);
    assert_eq!(is_valid("(({})".to_string()), false);
}