use std::collections::{HashMap, HashSet};

type Grid = HashMap<(i16, i16), usize>;
type Wire = Vec<Trace>;
type Trace = (char, usize);
type Point = (i16, i16);

fn manhatten_distance(a: Point, b: Point) -> i16 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_intersections(grid: &Grid) -> Vec<Point> {
    grid.iter()
        .filter(|(pos, count)| **count > 1)
        .map(|(pos, count)| *pos)
        .collect()
}

fn find_closest_intersection_distance(port: Point, grid: &Grid) -> i16 {
    grid.iter()
        .filter(|(pos, count)| **count > 1)
        .map(|(pos, count)| manhatten_distance(port, *pos))
        .min()
        .unwrap()
}

fn wires(input: &str) -> Vec<Wire> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            line.split(",")
                .map(|trace| {
                    let direction = trace.chars().nth(0).unwrap();
                    let steps = trace[1..trace.len()].parse::<usize>().unwrap();
                    (direction, steps)
                })
                .collect()
        })
        .collect()
}

fn steps(wires: &Vec<Wire>, point: Point) {
    for wire in wires {

    }
}

fn trace<F>(wire: Wire, reducer: &mut F)
    where F: FnMut(Point) -> () {
    let mut x = 0;
    let mut y = 0;

    for (direction, steps) in wire {
        for _ in 0..steps {
            match direction {
                'U' => y += 1,
                'D' => y -= 1,
                'R' => x += 1,
                'L' => x -= 1,
                _ => ()
            }
            let pos = (x, y);
            reducer(pos);
            // match grid.get_mut(&pos) {
            //     Some(c) => { 
            //         if !visited.contains(&pos) {
            //             *c += 1
            //         }
            //     },
            //     None => {
            //         grid.insert(pos, 1);
            //     }
            // }
            // visited.insert(pos);
        }
    }
}

fn find_closest_distance(origin: Point) -> i16 {
    let input = include_str!("./input.txt");
    let wires = wires(input);
    let mut grid = Box::new(Grid::new());
    let mut visited = HashSet::new();
    {
        for wire in wires {
            trace(wire, |pos| {
                match grid.get_mut(&pos) {
                    None => { grid.insert(pos, 1); () },
                    Some(c) => {
                        if !visited.contains(&pos) {
                            *c += 1;
                        }
                    }
                }
                visited.insert(pos);
            });
        }
    }
    //find_closest_intersection_distance(origin, &grid)
    0
}

pub fn solve() {
    let central_port = (0, 0);
    let distance = find_closest_distance(central_port);
    println!("{}", distance);
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     fn compute_distance(input: &str) -> i16 {
//         let wires = wires(input);
//         let mut grid = Grid::new();
//         trace(&mut grid, &wires);
//         let distance = find_closest_intersection_distance((0, 0), &grid);
//         distance
//     }

//     #[test]
//     fn example_1() {
//         let input = "\
// R75,D30,R83,U83,L12,D49,R71,U7,L72
// U62,R66,U55,R34,D71,R55,D58,R83";
//         let distance = compute_distance(input);
//         assert_eq!(159, distance);
//     }

//     #[test]
//     fn example_2() {
//         let input = "\
// R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
// U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
//         let distance = compute_distance(input);
//         assert_eq!(135, distance);
//     }

//     #[test]
//     fn example_3() {
//         let input = "\
// R75,D30,R83,U83,L12,D49,R71,U7,L72
// U62,R66,U55,R34,D71,R55,D58,R83";
//         let wires = wires(input);
//         assert_eq!(610, 0);
//     }

// }