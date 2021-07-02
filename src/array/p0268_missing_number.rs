/*
 * @Author: bill
 * @Date: 2021-07-02 14:24:45
 * @LastEditors: bill
 * @LastEditTime: 2021-07-02 15:36:42
 * @Description: 
 * @FilePath: /leetcode-rust/src/array/p0268_missing_number.rs
 */

pub struct Solution {}

impl Solution {
	// sum
	pub fn missing_number(nums: Vec<i32>) -> i32 {
		let mut sum: i32 = 0;
		let length: i32 = nums.len() as i32;
		for (i, num) in nums.into_iter().enumerate() {
			sum += num - (i as i32);
		}
		return length - sum;
	}

	// xor
	pub fn missing_number1(nums: Vec<i32>) -> i32 {
		let mut target: i32 = 0;
		let length: i32 = nums.len() as i32;
		for (i, num) in nums.into_iter().enumerate() {
			target = target^(i as i32)^num;
		}
		return target ^ length;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_268() {
		assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
		assert_eq!(2, Solution::missing_number(vec![0, 1]));
		assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
		assert_eq!(1, Solution::missing_number(vec![0]));
	}

	#[test]
	fn test_268_1() {
		assert_eq!(2, Solution::missing_number1(vec![3, 0, 1]));
		assert_eq!(2, Solution::missing_number1(vec![0, 1]));
		assert_eq!(8, Solution::missing_number1(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
		assert_eq!(1, Solution::missing_number1(vec![0]));
	}
}