/**
 * [Medium] [309] Best Time to Buy and Sell Stock with Cooldown
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sold = 0;
        let mut hold = std::i32::MIN;
        let mut rest = 0;
        for i in (0..prices.len()) {
            let prev_sold = sold;
            sold = hold + prices[i];
            hold = max(hold, rest - prices[i]);
            rest = max(rest, prev_sold);
        }
        return max(sold, rest);
    }

    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        //  can buy stock or rest
        let mut buy = vec![0; prices.len()];
        // hold stock, can sell
        let mut hold = vec![0; prices.len()];
        // just sold, only can rest
        let mut sold = vec![0; prices.len()];

        buy[0] = 0;
        hold[0] = -prices[0];
        sold[0] = std::i32::MIN;
        for i in 1..prices.len() {
            buy[i] = max(buy[i - 1], sold[i - 1]);
            hold[i] = max(hold[i - 1], buy[i - 1] - prices[i]);
            sold[i] = hold[i - 1] + prices[i];
        }
        // no more profit after buy a stock, only find the maximum of s0 ans s2
        max(buy[prices.len() - 1], sold[prices.len() - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        println!("MAX: {}", std::i32::MAX);
        println!("MIN: {}", std::i32::MIN);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit_1(vec![1, 2, 3, 0, 2]), 3);
    }
}
