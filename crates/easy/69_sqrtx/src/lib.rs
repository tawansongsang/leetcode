pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    let mut i = 1 as i64;
    while i * i <= x as i64 {
        i += 1
    }
    (i - 1).try_into().unwrap()
}

pub fn my_sqrt_best_time(x: i32) -> i32 {
    let x = x as i64;
    let mut lowest = 0 as i64;
    let mut highest = x as i64;
    let mut middle = highest / 2 as i64;
    if x > 46340 {
        highest = 46340; //Square of 46340 is highest possible i32
    };
    if x <= 3 {
        middle = 1;
    } else if x == 0 {
        middle = 0;
    } else if x == 2147483647 {
        middle = 46340;
    };
    while !(middle * middle <= x && (middle + 1) * (middle + 1) > x) {
        if middle * middle > x {
            highest = middle;
            middle = (middle + lowest) / 2;
        } else {
            lowest = middle;
            middle = (highest + middle) / 2;
        };
    }
    return middle as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = my_sqrt(4);
        let output = 2;
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let result = my_sqrt(8);
        let output = 2;
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let result = my_sqrt(2147395600);
        let output = 46340;
        assert_eq!(result, output);
    }
}
