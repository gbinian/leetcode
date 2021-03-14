/// 字符串转换整数(atoi)
/// 请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数（类似 C/C++ 中的 atoi 函数）。
pub fn my_atoi(s: String) -> i32 {
    // let mut negative = false;
    // let mut res = 0i64;
    // for (i, ch) in s.trim().chars().enumerate() {
    // 	if ch == '+' && i == 0 { continue; }
    // 	if ch == '-' && i == 0 { negative = true; continue; }
    // 	if !ch.is_digit(10) { break; }
    // 	res = 10i64 * res + ch.to_digit(10).unwrap() as i64;
    // 	if negative {
    // 		if -res < i32::min_value() as i64 {
    // 			return i32::min_value(); }
    // 	} else {
    // 		if res > i32::max_value() as i64 {
    // 			return i32::max_value();
    // 		}
    // 	}
    // }
    //
    // if negative { -res as i32 }
    // else { res as i32 }
    // 自动状态机
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    enum State {
        Start,
        Sign,
        Num,
        End,
    }
    use State::*;
    let mut m = std::collections::HashMap::with_capacity(4);
    let mut res = 0_i64;
    let mut sign = 1_i64;
    let mut state: State = Start;
    // start -> 0 Sign -> 1 Num -> 2 End -> 3
    m.insert(Start, vec![Start, Sign, Num, End]);
    m.insert(Sign, vec![End, End, Num, End]);
    m.insert(Num, vec![End, End, Num, End]);
    m.insert(End, vec![End, End, End, End]);
    for c in s.chars() {
        if state == End {
            break;
        }
        state = match c {
            ' ' => *(m.get(&state).unwrap().get(0).unwrap()),
            '+' => *(m.get(&state).unwrap().get(1).unwrap()),
            '-' => {
                let s = *(m.get(&state).unwrap().get(1).unwrap());
                if s != End {
                    sign = -1
                }
                s
            }
            n if n.is_ascii_digit() => {
                res = res * 10 + n.to_digit(10).unwrap() as i64;
                if sign == 1 && res > std::i32::MAX as i64 {
                    return std::i32::MAX;
                }
                if sign == -1 && -res < std::i32::MIN as i64 {
                    return std::i32::MIN;
                };
                *(m.get(&state).unwrap().get(2).unwrap())
            }
            _ => *(m.get(&state).unwrap().get(3).unwrap()),
        };
    }
    (sign * res) as i32
}

#[test]
fn test_my_atoi() {
    let s = "-91283472332";
    assert_eq!(my_atoi(s.into()), -2147483648);
    let s = "   -42";
    assert_eq!(my_atoi(s.into()), -42);
    let s = "42";
    assert_eq!(my_atoi(s.into()), 42);
    let s = "123-";
    assert_eq!(my_atoi(s.into()), 123);
}