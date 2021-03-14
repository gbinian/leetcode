/// 外观数列
/// 给定一个正整数 n ，输出外观数列的第 n 项。
///
/// 「外观数列」是一个整数序列，从数字 1 开始，序列中的每一项都是对前一项的描述。
///
/// 你可以将其视作是由递归公式定义的数字字符串序列：
///
/// countAndSay(1) = "1"
/// countAndSay(n) 是对 countAndSay(n-1) 的描述，然后转换成另一个数字字符串。
///
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

#[test]
fn test_count_and_say() {
    assert_eq!(count_and_say(1), "1");
    assert_eq!(count_and_say(2), "11");
    assert_eq!(count_and_say(5), "111221");
    assert_eq!(count_and_say(6), "312211");
}
