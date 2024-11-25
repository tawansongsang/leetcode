struct Solution;

#[allow(unused)]
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        nums.iter()
            .fold(Ranges::new(), |mut acc, num| {
                acc.add(*num);
                acc
            })
            .ranges
            .iter()
            .map(|r| r.to_string())
            .collect()
    }
}

struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    pub fn new() -> Ranges {
        Ranges { ranges: Vec::new() }
    }

    pub fn add(&mut self, x: i32) {
        match self.ranges.last_mut() {
            Some(Range(_low, high)) if *high + 1 == x => *high = x,
            _ => self.ranges.push(Range(x, x)),
        }
    }
}

struct Range(i32, i32);

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}->{}", self.0, self.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: Vec<i32> = vec![0, 1, 2, 4, 5, 7];
        let result: Vec<String> = Solution::summary_ranges(input);
        let output: Vec<String> = vec![
            String::from("0->2"),
            String::from("4->5"),
            String::from("7"),
        ];
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input: Vec<i32> = vec![0, 2, 3, 4, 6, 8, 9];
        let result: Vec<String> = Solution::summary_ranges(input);
        let output: Vec<String> = vec![
            String::from("0"),
            String::from("2->4"),
            String::from("6"),
            String::from("8->9"),
        ];
        assert_eq!(result, output);
    }
}
