use std::collections::HashSet;


/// 计数质数
/// 统计所有小于非负整数 n 的质数的数量。
pub fn count_primes(n: i32) -> i32 {
    // 埃氏筛标记法
    let mut m = HashSet::with_capacity((n / 2) as usize);
    let mut res = 0;
    for i in 2..n {
        if m.contains(&i) { continue } else {
            if is_prime(i) {
                m.insert(i);
                res += 1;
                // 质数的倍数一定是合数
                let mut count = 2;
                let mut filter = i;
                while filter < n {
                    filter = count * i;
                    count += 1;
                    m.insert(filter);
                }
            }
        }
  }
    res
}

fn is_prime(n: i32) -> bool {
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0  { return false }
    }
    true
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(37),true);
}

#[test]
fn test_count_primes() {
    assert_eq!(4, count_primes(10));
    assert_eq!(0, count_primes(0));
    assert_eq!(10, count_primes(30));
}