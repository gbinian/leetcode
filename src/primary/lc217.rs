/// 给定一个整数数组，判断是否存在重复元素
pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    // hash存储
    // let mut m = std::collections::HashMap::new();
    // nums.iter().find(|a| {
    // 	if m.contains_key(a) {
    // 		true
    // 	}else {
    // 		m.insert(a.to_owned(), 0);
    // 		false
    // 	}
    // }).is_some()
    // 8ms 4.3mb

    // use std::collections::HashSet;
    // use std::iter::FromIterator;
    // let s = HashSet::from_iter(nums.iter());
    // s.len() < nums.len()
    // 4ms 3mb

    // let len = nums.len();
    // nums.sort();
    // nums.dedup();
    // nums.len() < len
    // 0ms 2.6mb

    nums.sort();
    nums.windows(2)
        .any(|a| {
            let mut res = false;
            if let [i, j] = a {
                res = i == j
            }
            res
        })
    // nums.len() < len
    // 0ms 2.6mb
}

#[test]
fn test_contains_duplicate() {
    let v = vec![1, 2, 3, 1];
    let is_duplic = contains_duplicate(v);
    assert_eq!(is_duplic, true);
}
