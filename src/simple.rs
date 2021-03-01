/// 删除排序数组中的重复项
/// 给定一个排序数组，你需要在原地删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度
/// 不要使用额外的数组空间，你必须在原地修改输入数组 并在使用 O(1) 额外空间的条件下完成。
///
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut p1 = 0;
    let mut p2 = 1;
    let mut i = 0;
    while p2 < nums.len() {
        if nums[p2] == nums[p1] {
            p2 += 1;
        } else {
            p1 += 1;
            nums[p1] = nums[p2];
            i += 1;
        }
    }
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        let mut v = vec![1, 1, 2, 3, 4, 4, 5];
        let res = remove_duplicates(&mut v);
        assert_eq!(res, 5);
    }
}
