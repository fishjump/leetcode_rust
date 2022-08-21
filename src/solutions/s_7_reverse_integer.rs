/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x > 0 { 1 } else { -1 };
        let mut x = sign * x as i64;
        let mut ret = 0;

        while x >= 10 {
            ret += x % 10;
            ret *= 10;
            x /= 10;
        }
        ret += x;
        ret *= sign;

        if ret > i32::max_value() as i64 || ret < i32::min_value() as i64 {
            0
        } else {
            ret as i32
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
