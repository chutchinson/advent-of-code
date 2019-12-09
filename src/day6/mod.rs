use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Orbit {
    body: usize,
    satellite: usize
}

#[derive(Debug)]
struct Body {
    name: String
}

#[derive(Debug)]
struct OrbitMap {
    bodies: Vec<Body>,
    orbits: Vec<Orbit>
}

struct OrbitIterator<'a> {
    orbits: Vec<&'a Orbit>,
    index: usize
}

struct PathIterator<'a> {
    map: &'a OrbitMap,
    body: usize
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'a> Clone for OrbitIterator<'a> {
    fn clone(&self) -> Self {
        OrbitIterator {
            orbits: self.orbits.clone(),
            index: self.index
        }
    }
}

impl<'a> Iterator for OrbitIterator<'a> {
    type Item = &'a usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.orbits.len() {
            return None;
        }
        let orbit = self.orbits[self.index];
        self.index += 1;
        Some(&orbit.satellite)
    }
}

impl OrbitMap {

    fn satellites<'a>(&self, body: usize) -> OrbitIterator {
        let orbits = self.orbits.iter().filter(|x| x.body == body).collect();
        OrbitIterator {
            index: 0,
            orbits
        }
    }

    fn path_to_root<'a>(&self, body: usize) -> PathIterator {
        PathIterator {
            map: self,
            body
        }
    }

    fn shortest_path(&self, a: usize, b: usize) -> usize {
        let root = 0;
        let path1: HashSet<usize> = self.path_to_root(a).collect();
        let path2: HashSet<usize> = self.path_to_root(b).collect();
        let intersection = path1.intersection(&path2).next().unwrap();
        let index_1 = path1.iter().position(|x| x == intersection).unwrap();
        let index_2 = path2.iter().position(|x| x == intersection).unwrap();
        index_1 + index_2
    }

    fn total_orbits(&self, body: usize, depth: usize) -> usize {
        let satellites = self.satellites(body);
        let len = satellites.clone().count();
        if len == 0 {
            return depth;
        }
        depth + satellites.fold(0, |mut total, x| {
            total += self.total_orbits(*x, depth + 1);
            total
        })
    }

}

fn orbits(input: &str) -> OrbitMap {
    let mut bodies = Vec::new();
    let mut indices: HashMap<String, usize> = HashMap::new();
    let insert = |indices: &mut HashMap<String, usize>, bodies: &mut Vec<Body>, name: &String| {
        match indices.get(name) {
            Some(x) => *x,
            None => {
                let index = indices.len();
                indices.insert(name.clone(), index);
                bodies.push(Body {
                    name: name.clone()
                });
                index
            }
        }
    };
    let orbits: Vec<Orbit> = input.lines()
        .map(|line| {
            let mut parts = line.split(")");
            let body = parts.next().unwrap().to_string();
            let satellite = parts.next().unwrap().to_string();
            insert(&mut indices, &mut bodies, &body);
            insert(&mut indices, &mut bodies, &satellite);
            let body = *indices.get(&body).unwrap();
            let satellite = *indices.get(&satellite).unwrap();
            Orbit {
                body,
                satellite
            }
        })
        .collect();
    OrbitMap {
        bodies,
        orbits
    }
}

pub fn solve() {
    let input = include_str!("./input.txt");
    let orbits = orbits(input);
    let body = orbits.bodies.iter().position(|x| x.name == "COM").unwrap();
    let total_orbits = orbits.total_orbits(body, 0);
    println!("{:?}", total_orbits);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn calculates_total_orbits() {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let orbits = orbits(input);
        let total_orbits = orbits.total_orbits(0, 0);
        assert_eq!(42, total_orbits);
    }

}