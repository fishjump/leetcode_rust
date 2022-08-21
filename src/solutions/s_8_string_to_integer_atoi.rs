/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut res: i64 = 0;
        let mut sign = 1;
        let mut overflowed = false;

        for (i, ch) in s.chars().enumerate() {
            if i == 0 && ch == '+' {
                sign = 1;
                continue;
            }

            if i == 0 && ch == '-' {
                sign = -1;
                continue;
            }

            let x = ch as i64 - '0' as i64;
            if 0 <= x && x <= 9 {
                res *= 10;
                res += x;

                if sign * res < i32::min_value() as i64 || sign * res > i32::max_value() as i64 {
                    overflowed = true;
                    break;
                }

                continue;
            }

            break;
        }

        res *= sign;

        let res = if overflowed && sign == 1 {
            i32::max_value()
        } else if overflowed && sign == -1 {
            i32::min_value()
        } else {
            res as i32
        };

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("+-12")), 0);
        assert_eq!(
            Solution::my_atoi(String::from("18446744073709551617")),
            2147483647
        );
    }
}
