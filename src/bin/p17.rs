#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};

fn count_letters_simple(n: u64) -> usize {
    assert!(n <= 19);
    match n {
        0 => 0, // hack to exclude in 20, 30, etc.
        1 => "one".len(),
        2 => "two".len(),
        3 => "three".len(),
        4 => "four".len(),
        5 => "five".len(),
        6 => "six".len(),
        7 => "seven".len(),
        8 => "eight".len(),
        9 => "nine".len(),
        10 => "ten".len(),
        11 => "eleven".len(),
        12 => "twelve".len(),
        13 => "thirteen".len(),
        14 => "fourteen".len(),
        15 => "fifteen".len(),
        16 => "sixteen".len(),
        17 => "seventeen".len(),
        18 => "eighteen".len(),
        19 => "nineteen".len(),
        _ => panic!("invalid simple input"),
    }
}

fn count_letters_to_ten(n: u64) -> usize {
    match n / 10 {
        0 | 1 => count_letters_simple(n),
        2 => "twenty".len() + count_letters_simple(n % 10),
        3 => "thirty".len() + count_letters_simple(n % 10),
        4 => "forty".len() + count_letters_simple(n % 10),
        5 => "fifty".len() + count_letters_simple(n % 10),
        6 => "sixty".len() + count_letters_simple(n % 10),
        7 => "seventy".len() + count_letters_simple(n % 10),
        8 => "eighty".len() + count_letters_simple(n % 10),
        9 => "ninety".len() + count_letters_simple(n % 10),
        _ => panic!("invalid tens input"),
    }
}

fn count_letters_in_hundred(n: u64) -> usize {
    match n / 100 {
        0 => 0,
        1..=9 => "hundred".len() + count_letters_simple(n / 100),
        _ => panic!("invalid hundreds input"),
    }
}

fn count_and(n: u64) -> usize {
    if n <= 100 {
        return 0;
    }
    if n % 100 == 0 {
        return 0;
    }
    return 3;
}

fn count_letters(n: u64) -> usize {
    if n == 1000 {
        "onethousand".len()
    } else {
        count_letters_to_ten(n % 100) 
            + count_letters_in_hundred(n)
            + count_and(n)
    }
}

fn solve(n: u64) -> usize {
    (1..=n)
        .map(|m| count_letters(m))
        .sum()
}

fn main() -> () {
    let result: usize = solve(get_arg_else(1, 1000));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_letters_test() {
        use super::count_letters as cl;
        assert_eq!(cl(51), "fiftyone".len());
        assert_eq!(cl(7), "seven".len());
        assert_eq!(cl(30), "thirty".len());
        assert_eq!(cl(35), "thirtyfive".len());
        assert_eq!(cl(100), "onehundred".len());
        assert_eq!(cl(101), "onehundredandone".len());
        assert_eq!(cl(256), "twohundredandfiftysix".len());
        assert_eq!(cl(347), "threehundredandfortyseven".len());
        assert_eq!(cl(512), "fivehundredandtwelve".len());
        assert_eq!(cl(610), "sixhundredandten".len());
        assert_eq!(cl(482), "fourhundredandeightytwo".len());
        assert_eq!(cl(718), "sevenhundredandeighteen".len());
        assert_eq!(cl(999), "ninehundredandninetynine".len());
        assert_eq!(cl(342), 23);
        assert_eq!(cl(115), 20);
    }
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(9), 36);
    }
}
