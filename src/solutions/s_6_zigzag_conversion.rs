/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;

        let mut buf = Vec::with_capacity(s.len());

        let ch_per_loop = num_rows * 2 - 2;
        let s = s.as_bytes();

        for i in 0..num_rows {
            let mut j = i;
            while j < s.len() {
                if i == 0 || i == num_rows - 1 {
                    buf.push(s[j]);
                    j += ch_per_loop;
                } else {
                    buf.push(s[j]);
                    if j + ch_per_loop - 2 * i < s.len() {
                        buf.push(s[j + ch_per_loop - 2 * i]);
                    }
                    j += ch_per_loop;
                }
            }
        }

        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => String::from(""),
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
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );

        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );

        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
    }
}
