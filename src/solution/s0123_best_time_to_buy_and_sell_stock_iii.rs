/**
 * [Hard] [123] Best Time to Buy and Sell Stock III
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::max;

impl Solution {
    /**
     * see  [188] Best Time to Buy and Sell Stock IV
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let len = prices.len();

        let mut dp = vec![vec![0; prices.len()]; 3];
        for i in 1..3 {
            let mut localMax = dp[i - 1][0] - prices[0];
            for j in 1..len {
                localMax = max(localMax, dp[i - 1][j] - prices[j]);
                dp[i][j] = max(dp[i][j - 1], prices[j] + localMax);
            }
        }
        dp[2][len - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
