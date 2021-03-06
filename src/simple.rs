/// 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1
pub fn first_uniq_char(s: String) -> i32 {
    let mut i = -1;
    let mut m = std::collections::HashMap::new();
    for c in s.chars() {
        if m.contains_key(&c) {
            let v = m.get_mut(&c).unwrap();
            *v += 1;
        } else {
            m.insert(c, 1);
        }
    }
    for (j, c) in s.chars().enumerate() {
        if *m.get(&c).unwrap() == 1 {
            i = j as i32;
            break;
        }
    }
    i
}

///给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
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

/// 外观数列
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".into();
    } else {
        let s = count_and_say(n - 1);
        let mut count = 0;
        let mut res = "".to_string();
        let mut prev = ' ';
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                prev = c;
                count += 1;
            } else {
                if prev == c {
                    count += 1;
                } else {
                    res.push_str(count.to_string().as_str());
                    res.push(prev);
                    prev = c;
                    count = 1;
                }
            }
        }
        res.push_str(count.to_string().as_str());
        res.push(prev);
        res
    }
}

/// 最长公共前缀
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        "".to_string()
    } else if strs.len() == 1 {
        strs.last().unwrap().to_string()
    } else {
        let mut res = (*strs.first().unwrap()).clone();
        for s in strs.iter().skip(1) {
            res = {
                let mut p1 = 0;
                if s.len() == 0 || res.len() == 0 {
                    return "".to_string();
                }
                while p1 < res.len() && p1 < s.len() {
                    if !res.starts_with(&s[0..p1 + 1]) {
                        break;
                    }
                    p1 += 1;
                }
                res[0..p1].to_string()
            };
            if res.eq("") {
                return "".to_string();
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_uniq_char() {
        let s = "leetcode";
        assert_eq!(first_uniq_char(s.into()), 0)
    }
    #[test]
    fn test_is_palindrome() {
        let s = "a.";
        assert_eq!(is_palindrome(s.into()), true);
    }

    #[test]
    fn test_is_anagram() {
        let s = "rat";
        let t = "car";
        assert_eq!(is_anagram(s.into(), t.into()), false);
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
    #[test]
    fn test_str_str() {
        assert_eq!(str_str("hello".into(), "ll".into()), 2);
        assert_eq!(str_str("hello".into(), "".into()), 0);
        assert_eq!(str_str("hello".into(), "p".into()), -1);
        assert_eq!(str_str("mississippi".into(), "issip".into()), 4);
    }
    #[test]
    fn test_count_and_say() {
        assert_eq!(count_and_say(1), "1");
        assert_eq!(count_and_say(2), "11");
        assert_eq!(count_and_say(5), "111221");
        assert_eq!(count_and_say(6), "312211");
    }
    #[test]
    fn test_longest_common_prefix() {
        let mut s = vec!["ab".to_string(), "a".to_string()];
        assert_eq!(longest_common_prefix(s), "a".to_string());
        let mut s = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(s), "fl".to_string());
    }
}
