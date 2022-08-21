/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    fn expand_palindrome(s: &[u8], l: usize, r: usize) -> (usize, usize) {
        let l_limit = 0;
        let r_limit = s.len() - 1;
        let mut l = l;
        let mut r = r;
        let mut len = 0;

        while l_limit <= l && r <= r_limit {
            if s[l] != s[r] {
                len = r - l - 1;
                l += 1;
                break;
            }

            if l_limit == l || r == r_limit {
                len = r - l + 1;
                break;
            }

            l -= 1;
            r += 1;
        }

        (l, len)
    }

    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();

        let mut beg = 0;
        let mut max_len = 0;

        for (i, _) in s.iter().enumerate() {
            let (l, len) = Solution::expand_palindrome(s, i, i);
            if len > max_len {
                max_len = len;
                beg = l;
            }

            let (l, len) = Solution::expand_palindrome(s, i, i + 1);
            if len > max_len {
                max_len = len;
                beg = l;
            }
        }

        match std::str::from_utf8(&s[beg..beg + max_len]) {
            Ok(s) => String::from(s),
            Err(_) => String::new(),
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );

        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
    }
}
