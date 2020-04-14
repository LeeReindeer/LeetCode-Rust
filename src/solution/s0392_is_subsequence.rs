/**
 * [Easy] [392] Is Subsequence
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/is-subsequence/
// discuss: https://leetcode.com/problems/is-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    //is s a sub sequence of t
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if t.is_empty() {
            return false;
        }
        let mut index: isize = -1;
        for c in s.chars() {
            let index_option = t
                .char_indices()
                .position(|ci| ci.0 >= (index + 1) as usize && ci.1 == c);
            if index_option.is_none() {
                return false;
            } else {
                index = index_option.unwrap() as isize;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
