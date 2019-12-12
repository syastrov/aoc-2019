use six::*;


fn main() {
    let input = get_input();
    let orbits: Vec<Orbit> = parse_orbitals(&input);
    let tree = OrbitalTree::new(&orbits);
    println!("Got {} orbits", tree.count_orbits());
}


//fn common_ancestor(map: &HashMap<&str, Vec<&str>>, a: &str, b: &str) -> &str {
//
//}
