/// Fizz Buzz
/// 写一个程序，输出从 1 到 n 数字的字符串表示。
/// 1. 如果n是3的倍数，输出“Fizz”；
/// 2. 如果n是5的倍数，输出“Buzz”；
/// 3.如果n同时是3和5的倍数，输出 “FizzBuzz”。

pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n).map(|i| {
        if i % 3 == 0 && i % 5 == 0 { "FizzBuzz".to_string() }
        else if i % 3 == 0 { "Fizz".to_string() }
        else if i % 5 == 0 { "Buzz".to_string() }
        else { i.to_string() }
    }).collect()
}

#[test]
fn test_fizz_buzz() {
    assert_eq!(
        vec![  "1",
               "2",
               "Fizz",
               "4",
               "Buzz",
               "Fizz",
               "7",
        ],
        fizz_buzz(7)
    )
}
