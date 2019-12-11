use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("./inputs/3.txt");
    let grid = trace(input);
    let min_distance = min_distance(&grid);
    let min_steps = min_steps(&grid);

    println!("{}", min_distance);
    println!("{}", min_steps);
}

type Wire = Vec<Trace>;
type Trace = (char, usize);
type Point = (isize, isize);
type Grid = HashMap<Point, Cell>;

struct Cell {
    pub count: isize,
    pub steps: isize
}

fn manhatten_distance(a: Point, b: Point) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn min_steps(grid: &Grid) -> isize {
    grid.iter()
        .map(|(_, cell)| cell.steps)
        .min()
        .unwrap()
}

fn min_distance(grid: &Grid) -> isize {
    grid.iter()
        .map(|(point, _)| manhatten_distance((0, 0), *point))
        .min()
        .unwrap()
}

fn trace(input: &str) -> Grid {
    let mut grid = Grid::new();

    let wires: Vec<Wire> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|trace| {
                    let direction = trace.chars().nth(0).unwrap();
                    let steps = trace[1..trace.len()].parse::<usize>().unwrap();
                    (direction, steps)
                })
                .collect()
        })
        .collect();
    
    for wire in wires {
        let mut distance = 0;
        let mut x = 0;
        let mut y = 0;
        let mut visited: HashSet<Point> = HashSet::new();
        for (direction, n) in wire {
            for _ in 0..n {
                distance += 1;
                match direction {
                    'U' => y += 1,
                    'D' => y -= 1,
                    'R' => x += 1,
                    'L' => x -= 1,
                    _ => ()
                };
                let pos = (x, y);
                match grid.get_mut(&pos) {
                    Some(c) => {
                        if !visited.contains(&pos) {
                            c.count += 1;
                            c.steps += distance;
                        }
                    },
                    None => {
                        grid.insert(pos, Cell {
                            count: 1,
                            steps: distance
                        });
                    }
                }
                visited.insert(pos);
            }
        }
    }

    grid.retain(|_, cell| cell.count > 1);

    return grid;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn closest_point_has_distance_159() {
        let input = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let grid = trace(input);
        assert_eq!(159, min_distance(&grid));
    }

    #[test]
    fn closest_point_has_distance_135() {
        let input = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let grid = trace(input);
        assert_eq!(135, min_distance(&grid));
    }

    #[test]
    fn intersection_after_610_steps() {
        let input = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let grid = trace(input);
        assert_eq!(610, min_steps(&grid));
    }

    #[test]
    fn intersection_after_410_steps() {
        let input = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let grid = trace(input);
        assert_eq!(410, min_steps(&grid));
    }

}