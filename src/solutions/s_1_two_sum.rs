/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution {}
use std::{borrow::Borrow, collections::HashMap};

// @lc code=start

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        let mut ret = Vec::with_capacity(2);

        nums.iter().enumerate().for_each(|(i, x)| {
            let look_for = target - x;
            match map.get(look_for.borrow()) {
                Some(j) => {
                    ret.push(i as i32);
                    ret.push(*j as i32);
                }
                None => {
                    map.insert(x, i);
                }
            }
        });

        ret
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::utils::unordered_same::UnorderedSame;

    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::two_sum(vec![2, 7, 11, 15], 9).unordered_same(vec![0, 1]));
        assert!(Solution::two_sum(vec![3, 2, 4], 6).unordered_same(vec![1, 2]));
    }
}
