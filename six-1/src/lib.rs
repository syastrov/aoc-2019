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
    children: HashMap<&'a str, Vec<&'a str>>,
    parents: HashMap<&'a str, &'a str>,
}

impl<'a> OrbitalTree<'a> {
    pub fn new(orbits: &Vec<Orbit<'a>>) -> Self {
        let mut children = HashMap::new();
        let mut parents = HashMap::new();
        for Orbit { parent, child } in orbits {
            let parent_owned = parent.to_owned();
            let child_owned = child.to_owned();

            children.entry(parent_owned).or_insert_with(|| Vec::new()).push(child_owned);

            // Ensure that there is an entry for each child despite them not having any children themselves
            children.entry(child_owned).or_insert_with(|| Vec::new());

            parents.entry(child_owned).or_insert(parent_owned);
        }
        OrbitalTree { children, parents }
    }

    pub fn count_orbits(&self) -> u32 {
        let mut count = 0;
        self.count_orbits_recursively("COM", &mut count, 1);
        count
    }

    fn count_orbits_recursively(&self, parent: &str, count_from_root: &mut u32, distance: u32) -> () {
        for &child in self.children.get(parent).unwrap().iter() {
            *count_from_root += distance;
            println!("Child {}, {}", child, count_from_root);
            self.count_orbits_recursively(child, count_from_root, distance + 1);
        }
    }


    /// Get all of the ancestors of a given node
    pub fn get_ancestors(&self, node: &str) -> Vec<&str> {
        let mut ancestors = Vec::new();
        let mut node = node;
        while let Some(&parent) = self.parents.get(node) {
            ancestors.push(parent);
            node = parent;
        }
        ancestors
    }

    /// Find the common ancestor of the two nodes and return it along with the sum of the distance
    /// from the nodes.
    pub fn common_ancestor(&self, a: &str, b: &str) -> Option<(&str, u32)> {
        let ancestors_of_a = self.get_ancestors(a);
        let ancestors_of_b = self.get_ancestors(b);
//        println!("Ancestors A: {:?} B: {:?}", ancestors_of_a, ancestors_of_b);
        for (dist_from_a, &a) in ancestors_of_a.iter().enumerate() {
            for (dist_from_b, &b) in ancestors_of_b.iter().enumerate() {
                if a == b {
                    return Some((a, (dist_from_a + dist_from_b) as u32));
                }
            }
        }
        None
    }
}