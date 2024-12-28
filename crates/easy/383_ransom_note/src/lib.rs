#![allow(unused)]

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_map = std::collections::HashMap::new();
        magazine.chars().for_each(|c| {
            magazine_map
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        ransom_note.chars().all(|c| {
            if let Some(count) = magazine_map.get_mut(&c) {
                if *count > 0 {
                    *count -= 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
    }
    pub fn optimize_time_can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];

        ransom_note
            .bytes()
            .map(|b| b - 'a' as u8)
            .for_each(|b| count1[b as usize] += 1);
        magazine
            .bytes()
            .map(|b| b - 'a' as u8)
            .for_each(|b| count2[b as usize] += 1);
        // println!("{:?}", count1);
        // println!("{:?}", count2);
        for (a, b) in count1.iter().zip(count2.iter()) {
            if b < a {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ransome_note = String::from("a");
        let magazine = String::from("b");
        let result = Solution::can_construct(ransome_note, magazine);
        let expected = false;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let ransome_note = String::from("aa");
        let magazine = String::from("ab");
        let result = Solution::can_construct(ransome_note, magazine);
        let expected = false;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_3() {
        let ransome_note = String::from("aa");
        let magazine = String::from("aab");
        let result = Solution::can_construct(ransome_note, magazine);
        let expected = true;
        assert_eq!(result, expected);
    }
}
