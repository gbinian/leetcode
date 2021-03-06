#[inline]
pub fn reserve<T>(nums: &mut Vec<T>, m: usize, n: usize) {
    for i in m..(m + (n - m) / 2) {
        nums.swap(i, m + n - i - 1);
    }
}

/// 给定一个数组，将数组中的元素向右移动k个位置，其中k是非负数
/// [1,2,3,4,5] k = 2  -> [4,5,1,2,3,4]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    // 三次反转
    reserve(nums, 0, nums.len());
    reserve(nums, 0, k);
    reserve(nums, k, nums.len());
}

#[test]
fn test_rotate() {
    let mut v = vec![1, 2, 3, 4, 5];
    rotate(&mut v, 3);
    assert_eq!(v, vec![3, 4, 5, 1, 2]);
}

#[test]
fn test_reverse() {
    let mut v = vec![1, 2, 3, 4, 5];
    reserve(&mut v, 1, 5);
    assert_eq!(v, vec![1, 5, 4, 3, 2]);
}
