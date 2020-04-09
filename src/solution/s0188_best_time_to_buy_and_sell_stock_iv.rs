/**
 * [Hard] [188] Best Time to Buy and Sell Stock IV
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::max;

impl Solution {
    /**
     * for i trade time j day
     * dp[i][j] = max(case1, case2)
     * case1: dp[i][j-1]; //we do not trade
     * case2: prices[j] - prices[k] + dp[i - 1][k-1] (k=[0..j-1]); //buy on day k, sell on day j
     */
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() || k == 0 {
            return 0;
        }

        let len = prices.len();

        // if k >= len/2, we try to make maximum number of trasactions
        if k >= len as i32 / 2 {
            let mut max = 0;
            for i in 1..len {
                if prices[i] > prices[i - 1] {
                    max += prices[i] - prices[i - 1];
                }
            }
            return max;
        }

        let mut dp = vec![vec![0; prices.len()]; k as usize + 1];
        for i in 1..k as usize + 1 {
            let mut localMax = dp[i - 1][0] - prices[0];
            for j in 1..len {
                localMax = max(localMax, dp[i - 1][j] - prices[j]);
                dp[i][j] = max(dp[i][j - 1], prices[j] + localMax);
            }
        }
        dp[k as usize][len - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_188() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
