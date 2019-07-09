#[allow(unused_imports)]
use project_euler_practice::util::get_arg;
use project_euler_practice::big_num::BigNum;

// Doesn't do much
fn main() -> () {
    let result: u64 = get_arg(1);
    println!("{}", BigNum::from_string("123456").to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn template_test() {
        assert_eq!(true, true);
        assert_eq!(false, false);
    }
}
