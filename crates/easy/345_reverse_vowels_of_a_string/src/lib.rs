#![allow(unused)]

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = [
            'a' as u8, 'e' as u8, 'i' as u8, 'o' as u8, 'u' as u8, 'A' as u8, 'E' as u8, 'I' as u8,
            'O' as u8, 'U' as u8,
        ];
        let mut positions: Vec<usize> = Vec::new();
        let mut reverse_vowels: Vec<u8> = Vec::new();
        let mut result = s.bytes().collect::<Vec<u8>>();
        for i in 0..s.len() {
            if vowels.contains(&result[i]) {
                positions.push(i);
                reverse_vowels.push(result[i]);
            }
        }

        reverse_vowels.reverse();

        for i in 0..positions.len() {
            let position = positions[i];
            let vowel = reverse_vowels[i];
            result[position] = vowel;
        }

        String::from_utf8(result).unwrap()
    }

    fn is_vowel(ch: u8) -> bool {
        match ch as char {
            'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => true,
            _ => false,
        }
    }

    pub fn reverse_vowels_optimize_space(s: String) -> String {
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut s = s.as_bytes().to_vec();

        while i < j {
            if Self::is_vowel(s[i]) && Self::is_vowel(s[j]) {
                (s[i], s[j]) = (s[j], s[i]);
                i += 1;
                j -= 1;
            } else if !Self::is_vowel(s[i]) {
                i += 1;
            } else {
                j -= 1;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("IceCreAm");
        let result = Solution::reverse_vowels(s);
        let output = String::from("AceCreIm");
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let s = String::from("leetcode");
        let result = Solution::reverse_vowels(s);
        let output = String::from("leotcede");
        assert_eq!(result, output);
    }
}
