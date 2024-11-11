pub struct Solution {}

// #[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low_stock_price = i32::MAX;
        let mut current_profit = 0;
        let mut profit = 0;

        for price in prices {
            if low_stock_price > price {
                low_stock_price = price;
            }

            current_profit = price - low_stock_price;

            if current_profit > profit {
                profit = current_profit
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 0);
    }
}
