use rand::prelude::{thread_rng, SliceRandom};
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
 /// 给你一个整数数组 nums ，设计算法来打乱一个没有重复元素的数组。
/// 实现 Solution class:
/// Solution(int[] nums) 使用整数数组 nums 初始化对象
/// int[] reset() 重设数组到它的初始状态并返回
/// int[] shuffle() 返回数组随机打乱后的结果

pub struct Solution {
    original: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self {
            original: nums.clone()
        }
    }
    
    // /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }
    
    // /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut rng  = thread_rng();
        let mut res = self.original.clone();
        res.shuffle(&mut rng);
        res
    }
}

#[test]
fn test_solution() {
    let s = Solution::new(vec![1,2,3]);
    s.shuffle();
    assert_eq!(s.reset(), vec![1,2,3]);
}


