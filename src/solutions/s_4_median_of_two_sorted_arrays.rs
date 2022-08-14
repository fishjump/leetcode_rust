/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

pub struct Solution {}

// @lc code=start

impl Solution {
    fn k_smallest(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        let (mut nums1, mut nums2) = (nums1, nums2);
        let mut k = k;

        while !nums1.is_empty() || !nums2.is_empty() {
            if nums1.len() > nums2.len() {
                let tmp = nums1;
                nums1 = nums2;
                nums2 = tmp;
            }

            if nums1.is_empty() {
                return nums2[k];
            }

            if k == 0 {
                return std::cmp::min(nums1[0], nums2[0]);
            }

            let m1 = std::cmp::min((k + 1) / 2, nums1.len());
            let m2 = (k + 1) - m1;

            if nums1[m1 - 1] < nums2[m2 - 1] {
                nums1 = &nums1[m1..];
                k -= m1;
            } else if nums1[m1 - 1] > nums2[m2 - 1] {
                nums2 = &nums2[m2..];
                k -= m2;
            } else {
                return nums1[m1 - 1];
            }
        }

        0
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = (nums1.as_slice(), nums2.as_slice());
        let len = nums1.len() + nums2.len();
        let mid = |x: usize| (x - 1) / 2;

        let ret = if len % 2 == 1 {
            Solution::k_smallest(nums1, nums2, mid(len)) as f64
        } else {
            let a = Solution::k_smallest(nums1, nums2, mid(len)) as f64;
            let b = Solution::k_smallest(nums1, nums2, mid(len) + 1) as f64;

            (a + b) / 2.0
        };

        ret
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::k_smallest(vec![1, 2].as_slice(), vec![3, 4].as_slice(), 0),
            1
        );
        assert_eq!(
            Solution::k_smallest(vec![1, 2].as_slice(), vec![3, 4].as_slice(), 1),
            2
        );
        assert_eq!(
            Solution::k_smallest(vec![1, 2].as_slice(), vec![3, 4].as_slice(), 2),
            3
        );
        assert_eq!(
            Solution::k_smallest(vec![1, 2].as_slice(), vec![3, 4].as_slice(), 3),
            4
        );

        assert_eq!(
            Solution::k_smallest(vec![1, 3].as_slice(), vec![2].as_slice(), 0),
            1
        );
        assert_eq!(
            Solution::k_smallest(vec![1, 3].as_slice(), vec![2].as_slice(), 1),
            2
        );
        assert_eq!(
            Solution::k_smallest(vec![1, 3].as_slice(), vec![2].as_slice(), 2),
            3
        );
    }
}
