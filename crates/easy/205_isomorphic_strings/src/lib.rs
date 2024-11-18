use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut char_index_s: HashMap<char, usize> = HashMap::new();
        let mut char_index_t: HashMap<char, usize> = HashMap::new();
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for i in 0..s.len() {
            let char_s = s_bytes[i] as char;
            let char_t = t_bytes[i] as char;
            if !char_index_s.contains_key(&char_s) {
                char_index_s.insert(char_s, i);
            }
            if !char_index_t.contains_key(&char_t) {
                char_index_t.insert(char_t, i);
            }

            let idx_s = char_index_s.get(&char_s).unwrap();
            let idx_t = char_index_t.get(&char_t).unwrap();

            if idx_s != idx_t {
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
        let input_1 = String::from("egg");
        let input_2 = String::from("add");
        let result = Solution::is_isomorphic(input_1, input_2);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input_1 = String::from("foo");
        let input_2 = String::from("bar");
        let result = Solution::is_isomorphic(input_1, input_2);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input_1 = String::from("paper");
        let input_2 = String::from("title");
        let result = Solution::is_isomorphic(input_1, input_2);
        let output = true;
        assert_eq!(result, output);
    }
}
