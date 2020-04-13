/**
 * [Easy] [455] Assign Cookies
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/assign-cookies/
// discuss: https://leetcode.com/problems/assign-cookies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g.clone();
        let mut s = s.clone();
        g.sort();
        s.sort();
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;
        while i < s.len() && j < g.len() {
            if s[i] >= g[j] {
                res += 1;
                i += 1;
                j += 1;
                // child content with current cookie
                continue;
            }
            while i < s.len() && s[i] < g[j] {
                i += 1; // try next cookie
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_455() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
