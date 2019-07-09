// This doesn't correspond to a problem, essentially just a template file

use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn get_arg <T: FromStr> (arg_num: usize) -> T {
    let args: Vec<String> = env::args().collect();
    if args.len() <= arg_num {
        panic!("Not enough arguments, expected at least {}", arg_num);
    }
    return match (&args[arg_num]).parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not parse argument: {}", &args[arg_num])
    }
}

fn get_search_string() -> Vec<u64> {
    return
    "73167176531330624919225119674426574742355349194934
    96983520312774506326239578318016984801869478851843
    85861560789112949495459501737958331952853208805511
    12540698747158523863050715693290963295227443043557
    66896648950445244523161731856403098711121722383113
    62229893423380308135336276614282806444486645238749
    30358907296290491560440772390713810515859307960866
    70172427121883998797908792274921901699720888093776
    65727333001053367881220235421809751254540594752243
    52584907711670556013604839586446706324415722155397
    53697817977846174064955149290862569321978468622482
    83972241375657056057490261407972968652414535100474
    82166370484403199890008895243450658541227588666881
    16427171479924442928230863465674813919123162824586
    17866458359124566529476545682848912883142607690042
    24219022671055626321111109370544217506941658960408
    07198403850962455444362981230987879927244284909188
    84580156166097919133875499200524063689912560717606
    05886116467109405077541002256983155200055935729725
    71636269561882670428252483600823257530420752963450"
    .chars()
    .filter(|c| c.is_digit(10))
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect();
}

// finds the max product of n consecutive numbers in the search string
fn maximize_product(l: usize, search_string: Vec<u64>) -> u64 {
    let mut window_back: usize = 0;
    let mut window_front: usize = l - 1;
    // current_prod is the product if zeros were treated as ones
    let mut current_product: u64 = 1;
    // tracks the number of zeros in the product. If > 0 then the product is really 0
    let mut num_zeros_in_prod: u64 = 0;
    for i in window_back..=window_front {
        if search_string[i] != 0 {
            current_product *= search_string[i];
        }
        else {
            num_zeros_in_prod += 1;
        }
    }

    let mut max_product;
    if num_zeros_in_prod > 0 {
        max_product = 0;
    }
    else {
        max_product = current_product;
    }

    while window_front < search_string.len() - 1 {
        if search_string[window_back] != 0 {
            current_product /= search_string[window_back];
        }
        else {
            num_zeros_in_prod -= 1;
        }
        window_back += 1;
        window_front += 1;
        if search_string[window_front] != 0 {
            current_product *= search_string[window_front];
        }
        else {
            num_zeros_in_prod += 1;
        }
        if current_product > max_product && num_zeros_in_prod == 0 {
            max_product = current_product;
        }
    }
    return max_product;
}

// Doesn't do much
fn main() -> () {
    let search_string = get_search_string();
    println!("{}", maximize_product(get_arg(1), search_string));
}

#[cfg(test)]
mod tests {
    #[test]
    fn prod_test() {
        use super::maximize_product;
        assert_eq!(maximize_product(1, vec![1,2,3,4,5,6,7,8,0,0,0,9]), 9);
        assert_eq!(maximize_product(4, vec![0,9,8,9,9,0]), 5832);
        assert_eq!(maximize_product(2, vec![1,2,4,3,2,6,7,3,7,8,9,9,1,0]), 81);
        assert_eq!(maximize_product(3, vec![10,0,10,0,10]), 0);
    }
}
