// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出
// 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]

use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for x in nums.iter().enumerate() {
        map.insert(x.1, x.0);
    }

    for x in nums.iter().enumerate() {
        if x.1 > &target {
            continue;
        }
        let n = &target - x.1;
        match map.get(&n) {
            None => {
                continue;
            }
            Some(index) => {
                return vec![*x.1, *index as i32];
            }
        }
    }
    vec![]
}


#[test]
fn test1() {
    let index = two_sum(vec![2, 7, 11, 15], 9);
    for x in index {
        println!("index is {}", x);
    }
}