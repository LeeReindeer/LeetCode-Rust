/**
 * [Medium] [714] Best Time to Buy and Sell Stock with Transaction Fee
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        if (prices.is_empty()) {
            return 0;
        }
        let mut can_buy = vec![0; prices.len()];
        let mut can_sell = vec![0; prices.len()];
        // init status
        can_sell[0] = -prices[0];
        for i in (1..prices.len()) {
            can_sell[i] = max(can_sell[i - 1], can_buy[i - 1] - prices[i]);
            can_buy[i] = max(can_buy[i - 1], can_sell[i - 1] + prices[i] - fee);
        }
        can_buy[prices.len() - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_714() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }
}
