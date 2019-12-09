use std::collections::{HashMap};

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

impl OrbitMap {
    fn total_orbits(&self, body: usize, depth: usize) -> usize {
        let orbits = self.orbits.iter().enumerate().filter(|(_, x)| x.body == body);
        let len = orbits.clone().count();
        if len == 0 {
            return depth;
        }
        depth + orbits.fold(0, |mut total, (_, x)| {
            total += self.total_orbits(x.satellite, depth + 1);
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