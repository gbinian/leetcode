
/// 罗马数字转整数
/// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000

pub fn roman_to_int(s: String) -> i32 {
    s.chars().fold((0, ' '), |a, b|{
        match (a.1, b) {
            ('I', 'V') => (a.0 + 3, 'V'),
            ('I', 'X') => (a.0 + 8, 'X'),
            ('X', 'L') => (a.0 + 30, 'L'),
            ('X', 'C') => (a.0 + 80, 'C'),
            ('C', 'D') => (a.0 + 300, 'D'),
            ('C', 'M') => (a.0 + 800, 'M'),
            (_, 'I') => (a.0 + 1, 'I'),
            (_, 'V') => (a.0 + 5, 'V'),
            (_, 'X') => (a.0 + 10, 'X'),
            (_, 'L') => (a.0 + 50, 'L'),
            (_, 'C') => (a.0 + 100, 'C'),
            (_, 'D') => (a.0 + 500, 'D'),
            (_, 'M') => (a.0 + 1000, 'M'),
           _ => unreachable!()
        }
    }).0
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("IV".to_string()), 4);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCDLXXVI".to_string()), 1476);
}
