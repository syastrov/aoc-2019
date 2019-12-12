use std::collections::HashMap;
use std::fs;

fn get_input() -> &str {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    &contents
}

fn main() {
    let input = get_input();
    let orbits: Vec<(&str, &str)> = input.split('\n').map(|x: &str| {
        let values_vec = x.split(')').collect::<Vec<_>>();
        if values_vec.len() == 2 {
            let (parent, child): (&str, &str) = (values_vec[0], values_vec[1]);
            (parent, child)
        } else {
            panic!("Invalid value: {}", x);
        }
    }).collect();
    let mut map = HashMap::new();
    for (parent, child) in orbits {
        map.entry(parent).or_insert_with(|| Vec::new()).push(child);
        map.entry(child).or_insert_with(|| Vec::new());
    }
    let map = map;
    println!("Got {} orbits", count_orbits(&map));
}

fn count_orbits(map: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut count = 0;
    println!("{:?}", map);
    count_orbits_visitor(map, "SAN", &mut count, 1);
    println!("{:?}", count);
    count
//    *counts.get("COM").unwrap()
}

fn common_ancestor(map: &HashMap<&str, Vec<&str>>, a: &str, b: &str) -> &str {

}

fn count_orbits_visitor<'a>(map: &HashMap<&str, Vec<&'a str>>, parent: &'a str, count_from_root: &mut u32, distance: u32) -> () {
    let original = *count_from_root;
    for &child in map.get(parent).unwrap().iter() {
        *count_from_root += distance;
        println!("Child {}, {}", child, count_from_root);
        count_orbits_visitor(map, child, count_from_root, distance + 1);
    }
}
