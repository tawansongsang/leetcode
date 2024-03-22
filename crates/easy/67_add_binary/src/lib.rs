use std::collections::VecDeque;

pub fn add_binary(a: String, b: String) -> String {
    let a_vec_deque = VecDeque::from(a.chars().collect::<Vec<char>>());
    let b_vec_deque = VecDeque::from(b.chars().collect::<Vec<char>>());
    let mut a_iter = a_vec_deque.iter();
    let mut b_iter = b_vec_deque.iter();
    let max_length = std::cmp::max(a_vec_deque.len(), b_vec_deque.len());
    let mut output_vec_deque: VecDeque<char> = VecDeque::new();
    let mut carry = false;
    for _ in 0..max_length {
        let current_a = a_iter.next_back().unwrap_or(&'0');
        let current_b = b_iter.next_back().unwrap_or(&'0');

        if carry {
            if *current_a == '1' && *current_b == '1' {
                output_vec_deque.push_front('1');
                carry = true;
            } else if *current_a == '0' && *current_b == '0' {
                output_vec_deque.push_front('1');
                carry = false;
            } else {
                output_vec_deque.push_front('0');
                carry = true;
            }
        } else {
            if *current_a == '1' && *current_b == '1' {
                output_vec_deque.push_front('0');
                carry = true;
            } else if *current_a == '0' && *current_b == '0' {
                output_vec_deque.push_front('0');
                carry = false;
            } else {
                output_vec_deque.push_front('1');
                carry = false;
            }
        }
    }

    if carry {
        output_vec_deque.push_front('1');
    }

    output_vec_deque.into_iter().collect::<String>()
}

pub fn add_binary_best_time(a: String, b: String) -> String {
    let mut abits = a.chars().rev();
    let mut bbits = b.chars().rev();
    let mut carry = '0';
    let mut ret = String::new();
    loop {
        match (abits.next(), bbits.next(), carry) {
            (Some('0'), Some('0'), c) => {
                ret.push(c);
                carry = '0'
            }
            (Some('0'), Some('1'), '0') => ret.push('1'),
            (Some('0'), Some('1'), '1') => {
                ret.push('0');
                carry = '1'
            }
            (Some('1'), Some('0'), '0') => ret.push('1'),
            (Some('1'), Some('0'), '1') => {
                ret.push('0');
                carry = '1'
            }
            (Some('1'), Some('1'), '0') => {
                ret.push('0');
                carry = '1'
            }
            (Some('1'), Some('1'), '1') => {
                ret.push('1');
                carry = '1'
            }
            (Some(a), None, '0') => ret.push(a),
            (Some('0'), None, '1') => {
                ret.push('1');
                carry = '0'
            }
            (Some('1'), None, '1') => {
                ret.push('0');
                carry = '1'
            }
            (None, Some(b), '0') => ret.push(b),
            (None, Some('0'), '1') => {
                ret.push('1');
                carry = '0'
            }
            (None, Some('1'), '1') => {
                ret.push('0');
                carry = '1'
            }
            (None, None, '0') => return ret.chars().rev().collect(),
            (None, None, '1') => {
                ret.push('1');
                return ret.chars().rev().collect();
            }
            _ => panic!("oops"),
        }
    }
}

pub fn add_binary_best_mem(a: String, b: String) -> String {
    let mut result = String::from("");
    let mut arev = a.chars().rev();
    let mut brev = b.chars().rev();
    let mut helper = 0;
    loop {
        match (arev.next(), brev.next()) {
            (None, None) => {
                if helper == 1 {
                    result.push('1')
                }
                break;
            }
            (achar, bchar) => {
                let sum = achar.unwrap_or('0') as u32 + bchar.unwrap_or('0') as u32 + helper
                    - '0' as u32 * 2;
                helper = sum / 2;
                result.push_str(&(sum % 2).to_string());
            }
        }
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = format!("11");
        let b = format!("1");
        let result = add_binary(a, b);
        let output = format!("100");
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let a = format!("1010");
        let b = format!("1011");
        let result = add_binary(a, b);
        let output = format!("10101");
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let a = format!("11");
        let b = format!("1");
        let result = add_binary(a, b);
        let output = format!("100");
        assert_eq!(result, output);
    }

    // #[test]
    // fn test_4() {
    //     let a = format!("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101");
    //     let b = format!("110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011");
    //     let result = add_binary(a, b);
    //     let output = format!("10101");
    //     assert_eq!(result, output);
    // }
}
