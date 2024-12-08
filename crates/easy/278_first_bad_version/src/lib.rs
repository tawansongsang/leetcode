#![allow(unused)]

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution {
    versions: Vec<bool>,
}

#[allow(non_snake_case)]
impl Solution {
    pub fn isBadVersion(&self, version: i32) -> bool {
        let version = (version) as usize;
        self.versions[version]
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left <= right {
            let middle = left + (right - left) / 2;
            let is_bad = self.isBadVersion(middle);
            if is_bad {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let versions = Solution {
            versions: vec![false, false, false, false, true, true],
        };
        let input = 5;
        let result = versions.first_bad_version(input);
        let output = 4;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let versions = Solution {
            versions: vec![false, true],
        };
        let input = 1;
        let result = versions.first_bad_version(input);
        let output = 1;
        assert_eq!(result, output);
    }
}
