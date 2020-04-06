/**
 * [Easy] [121] Best Time to Buy and Sell Stock
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = match prices.get(0) {
            Some(m) => m,
            None => return 0,
        };
        let mut max = 0;
        for i in prices.iter() {
            if i < min {
                min = i;
            } else if max < i - min {
                max = i - min;
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
