/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

pub struct Solution {}

// @lc code=start

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut begin = 0;
        let mut max_len = 0;

        s.chars().enumerate().for_each(|(i, c)| {
            if let Some(x) = map.get(&c) {
                begin = if begin > x + 1 { begin } else { x + 1 };
            }
            map.insert(c, i);

            max_len = if max_len > i - begin + 1 {
                max_len
            } else {
                i - begin + 1
            };
        });

        max_len as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
