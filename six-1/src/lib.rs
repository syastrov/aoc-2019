use std::fs;
use std::collections::HashMap;

pub fn get_input() -> String {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

pub struct Orbit<'a> {
    parent: &'a str,
    child: &'a str,
}

pub fn parse_orbitals(input: &str) -> Vec<Orbit> {
    input.split('\n').map(|x: &str| {
        let values_vec = x.split(')').collect::<Vec<_>>();
        if values_vec.len() == 2 {
            Orbit { parent: values_vec[0], child: values_vec[1] }
        } else {
            panic!("Invalid value: {}", x);
        }
    }).collect()
}

pub struct OrbitalTree<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> OrbitalTree<'a> {
    pub fn new(orbits: &Vec<Orbit<'a>>) -> Self {
        let mut map = HashMap::new();
        for Orbit { parent, child } in orbits {
            map.entry(parent.to_owned()).or_insert_with(|| Vec::new()).push(child.to_owned());
            map.entry(child.to_owned()).or_insert_with(|| Vec::new());
        }
        OrbitalTree {
            map
        }
    }

    pub fn count_orbits(&self) -> u32 {
        let mut count = 0;
        self.count_orbits_visitor("COM", &mut count, 1);
        count
    }

    fn count_orbits_visitor(&self, parent: &str, count_from_root: &mut u32, distance: u32) -> () {
        for &child in self.map.get(parent).unwrap().iter() {
            *count_from_root += distance;
            println!("Child {}, {}", child, count_from_root);
            self.count_orbits_visitor(child, count_from_root, distance + 1);
        }
    }
}