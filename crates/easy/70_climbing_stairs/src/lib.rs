pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    };

    if n == 2 {
        return 2;
    }

    let mut steps = [1,2,0];

    for _ in 2..n {
        steps[2] = steps[0] + steps[1];
        steps[0] = steps[1];
        steps[1] = steps[2];
    }

    steps[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = climb_stairs(2);
        let output = 2;
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let result = climb_stairs(3);
        let output = 3;
        assert_eq!(result, output);
    }
}
