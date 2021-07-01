/*
 * @Author: bill
 * @Date: 2021-06-30 18:21:00
 * @LastEditors: bill
 * @LastEditTime: 2021-07-01 11:07:37
 * @Description: 
 * @FilePath: /leetcode-rust/src/array/p0034_find_first_and_last_position_of_element_in_sorted_array.rs
 */
pub struct Solution {}

// TODO
impl Solution {
	// 暴力法
	pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut left = -1;
		let mut right = -1;
		for (index, num) in nums.into_iter().enumerate() {
			if num == target {
				let i = index as i32;
				if left == -1 {
					left = i
				}
				right = i
			}
		}
		vec![left, right]
	}

	// 二分查找
	pub fn search_range_1(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
		// let left = self.search_first_equal_element(&nums, target);
		// println!("{:?}", left);
		// let right = self.search_last_equal_element(&nums, target);
		// println!("{:?}", right);
		// vec![left, right]
		vec![self.search_first_equal_element(&nums, target), self.search_last_equal_element(&nums, target)]
	}

	// 二分查找第一个等于target的元素
	fn search_first_equal_element(&self, nums: &Vec<i32>, target: i32) -> i32 {
		let  (mut left, mut mid, mut right) = (0, 0, nums.len() - 1);

		let mut index: i32 = -1;
		while left <= right {
			mid = (left + right)/2;
			if nums[mid] > target {
				right = mid - 1;
			} else if nums[mid] < target {
				left = mid + 1;
			} else {
				if mid == 0 || nums[mid-1] != target {
					index  = mid as i32;
					break
				} else {
					right = mid - 1;
				}
			}
		}
		index
	}

	// 二分查找最后一个等于target的元素
	fn search_last_equal_element(&self, nums: &Vec<i32>, target: i32) -> i32 {
		let (mut left, mut mid, mut right) = (0, 0, nums.len() - 1);
		let mut index: i32 = -1;
		while left <= right {
			mid = (left + right)/2;
			if nums[mid] > target {
				right = mid - 1;
			} else if nums[mid] < target {
				left = mid + 1;
			} else {
				if mid == nums.len() - 1 || nums[mid+1] != target {
					index  = mid as i32;
					break
				} else {
					left = mid + 1;
				}
			}
		}
		index
	}

	// 二分查找第一个大于等于target的元素
	fn search_first_greater_equal_element(&self, nums: &Vec<i32>, target: i32) -> i32 {
		let (mut left, mut mid, mut right) = (0, 0, nums.len() - 1);
		let mut index: i32 = -1;
		while left <= right {
			mid = (left + right)/2;
			if nums[mid] < target {
				left = mid + 1;
			} else {
				if mid == 0 || nums[mid-1] < target {
					index = mid as i32;
					break
				} else {
					right = mid - 1;
				}
			}
		}
		index
	}

	// 二分查找最后一个小于等于target的元素
	fn search_last_less_equal_element(&self, nums: &Vec<i32>, target: i32) -> i32 {
		let (mut left, mut mid, mut right) = (0, 0, nums.len() - 1);
		let mut index: i32 = -1;
		while left <= right {
			mid = (left + right)/2;
			if nums[mid] > target {
				right = mid - 1;
			} else {
				if mid == nums.len() - 1 || nums[mid+1] > target {
					index = mid as i32;
					break
				} else {
					left = mid + 1;
				}
			}
		}
		index
	}

}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_34() {
		assert_eq!(vec![3, 4], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
		assert_eq!(vec![-1, -1], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6));
	}

	#[test]
	fn test_34_1() {
		let solution = Solution{};
		assert_eq!(vec![3, 4], solution.search_range_1(vec![5, 7, 7, 8, 8, 10], 8));
		assert_eq!(vec![-1, -1], solution.search_range_1(vec![5, 7, 7, 8, 8, 10], 6));
	}
}