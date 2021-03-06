// 给定两个数组，编写一个函数来计算它们的交集。
pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    // 哈希表遍历 记录出现次数
    // use std::collections::HashMap;
    // let (min, max) = if nums1.len() < nums2.len() { (nums1, nums2)} else { (nums2, nums1) };
    // let mut  m = min.iter().fold(HashMap::with_capacity(min.len()), |mut a, b| {
    // 	if a.contains_key(b) {
    // 		let mut v = a.get_mut(b).unwrap();
    // 		*v += 1;
    // 	} else {
    // 		a.insert(*b, 1);
    // 	}
    // 	a
    // });
    // let mut v = Vec::with_capacity(min.len());
    // max.iter().for_each(|a| {
    // 	if m.contains_key(a) {
    // 		v.push(a.to_owned());
    // 		let v = m.get(a).unwrap();
    // 		if *v == 1 {
    // 			m.remove(a);
    // 		} else {
    // 			m.insert(*a,*v - 1);
    // 		}
    // 	}
    // });
    // v
    // 0 2.1

    // 排序 双指针
    use std::cmp::Ordering::*;
    let mut v = Vec::new();
    nums1.sort();
    nums2.sort();
    let (mut p1, mut p2) = (0, 0);
    while p1 < nums1.len() && p2 < nums2.len() {
        match nums1[p1].cmp(&nums2[p2]) {
            Greater => p2 += 1,
            Less => p1 += 1,
            Equal => {
                v.push(nums1[p1]);
                p1 += 1;
                p2 += 1;
            }
        }
    }
    v
}

#[test]
fn test_intersect() {
    let v1 = vec![3, 1, 2];
    let v2 = vec![1, 1];
    assert_eq!(intersect(v1, v2), vec![1]);
}
