use six::*;


fn main() {
    let input = get_input();
    let orbits: Vec<Orbit> = parse_orbitals(&input);
    let tree = OrbitalTree::new(&orbits);
    println!("Got {} orbits", tree.count_orbits());
    let ancestor = tree.common_ancestor("SAN", "YOU").expect("No common ancestor");
    println!("Got common ancestor {} at total distance {}", ancestor.0, ancestor.1);
}

