
/// 最长公共前缀
/// 编写一个函数来查找字符串数组中的最长公共前缀。
//
/// 如果不存在公共前缀，返回空字符串 ""。
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


#[test]
fn test_longest_common_prefix() {
    let s = vec!["ab".to_string(), "a".to_string()];
    assert_eq!(longest_common_prefix(s), "a".to_string());
    let s = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!(longest_common_prefix(s), "fl".to_string());
}

