#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, get_arg_else, read_input};
#[allow(unused_imports)]
use rayon::prelude::*;

type Rectangle = (u64, u64);

// counts the number of places the given rectangle can fit
fn count_placements(large: Rectangle, small: Rectangle) -> u64 {
    (large.0 - small.0 + 1) * (large.1 - small.1 + 1)
}

fn count_sub_rectangles(rect: Rectangle) -> u64 {
    (1_u64..=rect.0)
        .map(|h| {
            (1_u64..=rect.1)
                .map(|w| count_placements(rect, (h, w)))
                .sum::<u64>()
        })
        .sum()
}

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn solve(target: u64) -> u64 {
    let mut best_grid: Rectangle = (0, 0);
    let mut best_delta = target; // current difference between grid count and target
    let mut w = 1;
    loop {
        let mut h = 1;
        let mut did_iterate = false;
        loop {
            let current_count = count_sub_rectangles((h, w));
            let current_delta = diff(target, current_count);
            if current_delta < best_delta {
                best_grid = (h, w);
                best_delta = current_delta;
            }
            if current_count > target {
                break;
            } else {
                h += 1;
                did_iterate = true;
            }
        }
        if !did_iterate {
            break;
        } else {
            w += 1;
        }
    }
    let (h, w) = best_grid;
    return h * w;
}

fn main() -> () {
    let result = solve(get_arg(1));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    #[test]
    fn count_placements_test() {
        use super::count_placements as count;
        let grid: Rectangle = (2, 3);
        assert_eq!(count(grid, (1, 1)), 6);
        assert_eq!(count(grid, (2, 3)), 1);
        assert_eq!(count(grid, (1, 2)), 4);
        assert_eq!(count(grid, (2, 1)), 3);
    }
    #[test]
    fn count_sub_rectangles_test() {
        use super::count_sub_rectangles as count;
        assert_eq!(count((2, 3)), 18);
        assert_eq!(count((2, 2)), 9);
        assert_eq!(count((1, 1)), 1);
        assert_eq!(count((3, 2)), 18);
    }
    #[test]
    fn solve_test() {
        use super::solve;
        assert_eq!(solve(10), 4);
        assert_eq!(solve(100), 16);
    }
}
