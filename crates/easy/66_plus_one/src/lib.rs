use std::collections::VecDeque;

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let length = digits.len();
    for i in (0..length).rev() {
        let current = digits[i];
        if current == 9 {
            digits[i] = 0;
            if i == 0 {
                digits.insert(0, 1);
            }
            continue;
        }

        digits[i] += 1;
        break;
    }

    digits
}

pub fn plus_one_best_time(mut digits: Vec<i32>) -> Vec<i32> {
    for x in digits.iter_mut().rev() {
        match *x == 9 {
            true => *x = 0,
            false => {
                *x += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}

pub fn plus_one_best_mem(digits: Vec<i32>) -> Vec<i32> {
    let mut target = VecDeque::from(digits);

    let mut result = VecDeque::<i32>::new();
    let mut prev_add_value = 1;
    while !target.is_empty() {
        let current = target.pop_back().unwrap();

        let mut sum = current + prev_add_value;

        if sum >= 10 {
            sum -= 10;
            prev_add_value = 1;
        } else {
            prev_add_value = 0;
        }

        result.push_front(sum);
    }

    if prev_add_value != 0 {
        result.push_front(1);
    }

    result.into_iter().collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let digits = vec![1, 2, 3];
        let output = vec![1, 2, 4];
        let result = plus_one(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let digits = vec![4, 3, 2, 1];
        let output = vec![4, 3, 2, 2];
        let result = plus_one(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let digits = vec![9];
        let output = vec![1, 0];
        let result = plus_one(digits);
        assert_eq!(result, output);
    }
}
