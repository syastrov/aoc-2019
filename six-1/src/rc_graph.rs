mod rc_graph {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::collections::HashSet;

    pub struct Node {
        datum: &'static str,
        edges: Vec<Rc<RefCell<Node>>>,
    }

    impl Node {
        pub fn new(datum: &'static str) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                datum: datum,
                edges: Vec::new(),
            }))
        }

        pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
            where F: Fn(&'static str)
        {
            if seen.contains(&self.datum) {
                return;
            }
            f(self.datum);
            seen.insert(self.datum);
            for n in &self.edges {
                n.borrow().traverse(f, seen);
            }
        }

        pub fn first(&self) -> Rc<RefCell<Node>> {
            self.edges[0].clone()
        }
    }
}
//
//fn foo(node: &Node) {
//    println!("foo: {}", node.datum);
//}
//
//
//pub fn main() {
//    let g = init();
//    let g = g.borrow();
//    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
//    let f = g.first();
//    foo(&*f.borrow());
//}