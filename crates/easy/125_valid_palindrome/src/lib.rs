struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "")
            .chars()
            .collect::<Vec<char>>();
        if s.is_empty() {
            return true;
        }
        let mut start = 0;
        let mut end = s.len() - 1;
        while start < end {
            if s[start] != s[end] {
                return false;
            }

            start += 1;
            end -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("A man, a plan, a canal: Panama");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let input = String::from("race a car");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let input = String::from(" ");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }
}
