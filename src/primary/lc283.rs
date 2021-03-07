/// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let (mut p1, mut p2) = (0, 0);
    while p2 < nums.len() {
        if nums[p2] != 0 {
            nums.swap(p1, p2);
            p1 += 1;
        }
        p2 += 1;
    }
}

#[test]
fn test_move_zeroes() {
    let mut nums = vec![1, 2, 0, 3, 0];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 0, 0]);
}
