/*
 * @Author: bill
 * @Date: 2021-07-02 14:24:45
 * @LastEditors: bill
 * @LastEditTime: 2021-09-16 17:54:34
 * @Description:
 * @FilePath: /leetcode-rust/src/problems/p0021_merge_two_sorted_lists.rs
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    // 注意一下所有权移动就好了
    pub fn merge_two_lists1(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut newNode = ListNode::new(0);
        let mut prev = &mut newNode;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                // 将较小链表连接到新链表尾节点，所有权移动
                prev.next = l1;
                // 将newNode尾节点指向它的后继节点
                prev = prev.next.as_mut().unwrap();
                // 将链表从尾节点取下来，将所有权返回给较小的链表
                l1 = prev.next.take();
            } else {
                // 将较小链表连接到新链表尾节点，所有权移动
                prev.next = l2;
                // 将newNode尾节点指向它的后继节点
                prev = prev.next.as_mut().unwrap();
                // 将链表从尾节点取下来，将所有权返回给较小的链表
                l2 = prev.next.take();
            }
        }
        prev.next = if l1.is_some() { l1 } else { l2 };
        newNode.next
    }

    // 通过递归和模式匹配，Rust的链表处理也可以很简洁清晰。注意到递归调用时需要使用 Self:: 来引用当前结构体的函数。
    pub fn merge_two_lists2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists2(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists2(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists1() {
        // how to test
        // assert_eq!(1, Solution::merge_two_lists1(vec![1, 1]));
        // assert_eq!(16, Solution::merge_two_lists1(vec![4, 3, 2, 1, 4]));
        // assert_eq!(2, Solution::merge_two_lists1(vec![1, 2, 1]));
    }
}
