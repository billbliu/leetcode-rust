/*
 * @Author: bill
 * @Date: 2021-07-02 14:24:45
 * @LastEditors: bill
 * @LastEditTime: 2021-09-16 17:46:49
 * @Description:
 * @FilePath: /leetcode-rust/src/problems/p0001_two_sum.rs
 */

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    // HashMap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(&other_index) = map.get(&(target - num)) {
                return vec![other_index as i32, index as i32];
            } else {
                map.insert(num, index);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
