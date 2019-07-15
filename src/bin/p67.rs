#[allow(unused_imports)]
use project_euler_practice::util::{get_arg, read_input};

type Cell = (usize, usize);
type Triangle = Vec<Vec<u64>>;

fn parse_input(triangle: &str) -> Vec<Vec<u64>> {
    triangle
        .lines()
        .map(|r| {
            r.trim()
                .split(" ")
                .map(|n| {n.trim().parse::<u64>().unwrap()})
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn get(cell: Option<Cell>, triangle: &Triangle) -> u64 {
    match cell {
        None => 0,
        Some((r,c)) => triangle[r][c]
    }
}

fn left_parent(cell: Cell) -> Option<Cell> {
    match cell {
        (_, 0) => None,
        (n, m) => Some((n-1, m-1))
    }
}

fn right_parent(cell: Cell) -> Option<Cell> {
    match cell {
        (n, m) if n == m => None,
        (n, m) => Some((n-1, m))
    }
}

fn max_parent(cell: Cell, tri: &Triangle) -> u64 {
    let left = get(left_parent(cell), tri);
    let right = get(right_parent(cell), tri);
    std::cmp::max(left, right)
}

fn solve(tri: &mut Triangle) -> u64 {
    for r in 0..tri.len() {
        for c in 0..=r {
            tri[r][c] += max_parent((r,c), &tri);
        }
    }
    return *tri[tri.len() - 1].iter()
        .max()
        .unwrap();
}

fn main() -> () {
    let mut tri = parse_input(&read_input("p67"));
    let result = solve(&mut tri);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn parent_test() {
        use super::left_parent as l;
        use super::right_parent as r;
        assert_eq!(l((3,3)), Some((2,2)));
        assert_eq!(r((3,3)), None);
        assert_eq!(l((3,0)), None);
        assert_eq!(r((3,0)), Some((2,0)));
    }
    #[test]
    fn triangle_test() {
        use super::{solve, parse_input};
        let t = "3
                 7 4
                 2 4 6
                 8 5 9 3";
        let mut tri = parse_input(t);
        assert_eq!(tri, vec![vec![3], vec![7,4], vec![2,4,6], vec![8,5,9,3]]);
        assert_eq!(solve(&mut tri), 23);
    }
}
