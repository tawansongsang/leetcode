pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return ();
    }

    for i in 0..n {
        let idx1 = i as usize;
        let idx2 = (i + m) as usize;
        nums1[idx2] = nums2[idx1];
    }

    nums1.sort();
}

pub fn merge_best_time(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut res = vec![0; (m + n) as usize];
    let mut i: usize = 0;
    let mut j: usize = 0;
    let s1: usize = m as usize;
    let s2: usize = n as usize;
    loop {
        if i == s1 {
            while j < s2 {
                res[i + j] = nums2[j];
                j += 1;
            }
            break;
        }
        if j == s2 {
            while i < s1 {
                res[i + j] = nums1[i];
                i += 1;
            }
            break;
        }
        if nums1[i] <= nums2[j] {
            res[i + j] = nums1[i];
            i += 1;
        } else {
            res[i + j] = nums2[j];
            j += 1;
        }
    }
    i = 0;
    while i < s1 + s2 {
        nums1[i] = res[i];
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut num1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut num2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut num1, m, &mut num2, n);
        let result = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(num1, result);
    }

    #[test]
    fn test_2() {
        let mut num1 = vec![1];
        let m = 1;
        let mut num2 = Vec::new();
        let n = 0;
        merge(&mut num1, m, &mut num2, n);
        let result = vec![1];
        assert_eq!(num1, result);
    }

    #[test]
    fn test_3() {
        let mut num1 = vec![0];
        let m = 0;
        let mut num2 = vec![1];
        let n = 1;
        merge(&mut num1, m, &mut num2, n);
        let result = vec![1];
        assert_eq!(num1, result);
    }
}
