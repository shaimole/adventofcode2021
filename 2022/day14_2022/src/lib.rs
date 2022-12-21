use std::collections::HashSet;
use std::path::Path;

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let map = as_map(filename);
    println!("{:?}", map);
    for i in 0..=9 {
        for j in 494..=503 {
            if map.contains(&(j, i)) {
                print!("x")
            } else {
                print!(".")
            }
        }
        println!("");
    }
    0
}
fn as_map<P>(filename: P) -> HashSet<(u32, u32)>
where
    P: AsRef<Path>,
{
    let lines = common::split_lines(common::read_lines(filename), " -> ");
    let mut map = HashSet::new();
    for cave in lines {
        let split: Vec<&str> = cave[0].split(",").collect();
        let mut x: u32 = split[0].clone().parse().unwrap();
        let mut y: u32 = split[1].clone().parse().unwrap();
        for i in 1..cave.len() {
            let split: Vec<&str> = cave[i].split(",").collect();
            let new_x = split[0].clone().parse().unwrap();
            let new_y = split[1].clone().parse().unwrap();
            println!("{:?}", vec![(x, new_x), (y, new_y)]);
            if x == new_x {
                for j in std::cmp::min(y, new_y)..=std::cmp::max(y, new_y) {
                    map.insert((x, j));
                }
            }

            if y == new_y {
                for j in std::cmp::min(x, new_x)..=std::cmp::max(x, new_x) {
                    map.insert((j, y));
                }
            }
            x = new_x;
            y = new_y;
        }
    }
    map
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 13)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 140)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
