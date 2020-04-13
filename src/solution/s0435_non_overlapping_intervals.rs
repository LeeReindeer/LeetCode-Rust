/**
 * [Medium] [435] Non-overlapping Intervals
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/non-overlapping-intervals/
// discuss: https://leetcode.com/problems/non-overlapping-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    /**
     * 1. sort intervals by end of interval, set x to min end interval
     * 2. delete interval that overlapping with x, update x
     */
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let len = intervals.len();
        let mut count = 1;
        let mut intervals = intervals.clone();
        // sort by end
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut end = intervals[0][1];
        for interval in intervals {
            let start = interval[0];
            if start >= end {
                count += 1;
                end = interval[1];
            }
        }
        len as i32 - count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_435() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
