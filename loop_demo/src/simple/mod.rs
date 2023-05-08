use std::collections::HashMap;
use std::iter::Map;

// 给定一个整数数组 nums 和一个整数目标值 target，
// 请你在该数组中找出 和为目标值 target 的那两个整数，
// 并返回它们的数组下标。
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
//
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let map = HashMap::new();
    let enumerate = nums.iter().enumerate();


    vec![]
}

#[test]
fn tk() {
    two_sum(vec![2, 7, 11, 15], 9);
}