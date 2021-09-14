/*
 * @Author: bill
 * @Date: 2021-07-02 14:24:45
 * @LastEditors: bill
 * @LastEditTime: 2021-09-14 17:43:11
 * @Description:
 * @FilePath: /leetcode-rust/src/problems/p0011_container_with_most_water.rs
 */

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut res) = (0, height.len() - 1, 0);
        while left < right {
            res = res.max(height[left].min(height[right]) * (right - left) as i32);
            if height[left] < height[right] {
                left = left + 1;
            } else {
                right = right - 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(1, Solution::max_area(vec![1, 1]));
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
        assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
    }
}
