#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn get_input(file: &str) -> Vec<String> {
    let input = read_input(file);
    input.split(',')
        .map(|s| s.trim()
             .chars()
             .filter(|&c| c != '"')
             .collect())
        .collect()
}

fn score_word(word: &str) -> u64 {
    word.chars()
        .map(|c| (c as u64) - ('A' as u64) + 1)
        .sum()
}

fn solve(file: &str) -> u64 {
    let mut input = get_input(file);
    input.sort_unstable();
    input.iter()
        .enumerate()
        .map(|(i, s)| (i as u64 + 1) * score_word(s))
        .sum()
}

fn main() -> () {
    let result: u64 = solve(&get_arg_else(1, "p22".to_owned()));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_input_test() {
        use super::get_input;
        let input = get_input("p22_test");
        assert_eq!(input[0], "MARY");
        assert_eq!(input[4], "ELIZABETH");
    }
    #[test]
    fn score_word_test() {
        use super::score_word;
        assert_eq!(score_word("COLIN"), 53);
    }
}
