#[allow(unused_imports)]
use project_euler_practice::util::get_arg;

fn main() -> () {
    let result: u64 = get_arg(1);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn template_test() {
        assert_eq!(true, true);
        assert_eq!(false, false);
    }
}
