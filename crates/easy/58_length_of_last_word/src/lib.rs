pub fn length_of_last_word(s: String) -> i32 {
    let s_trim = s.trim();
    let mut s_split = s_trim.split(' ');
    let last_word = s_split.next_back();

    match last_word {
        Some(last) => last.len() as i32,
        None => 0,
    }
}

pub fn length_of_last_word_best_mem(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("Hello World");
        let result = length_of_last_word(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let s = String::from("   fly me   to   the moon  ");
        let result = length_of_last_word(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let s = String::from("luffy is still joyboy");
        let result = length_of_last_word(s);
        assert_eq!(result, 6);
    }
}
