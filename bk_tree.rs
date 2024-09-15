mod lvd;

use std::collections::HashMap;

const MAX_D: usize = 3;

fn main() {
    let mut bkt = BkTree::new("cook");

    bkt.insert("book");
    bkt.insert("books");
    bkt.insert("earth");
    bkt.insert("what");
    bkt.insert("water");
    bkt.insert("wavy");

    bkt.dump();

    let query = "wavy";
    println!("Search for: {}", query);
    bkt.search(query);
}

#[derive(Debug, Default)]
struct Node {
    v: String,
    childs: HashMap<usize, Node>,
}

impl Node {
    fn new(v: &str) -> Self {
        Self {
            v: v.into(),
            ..Default::default()
        }
    }

    fn insert(&mut self, word: &str) {
        let d = lvd::lev(&self.v, word);

        if let Some(e) = self.childs.get_mut(&d) {
            e.insert(word);
        } else {
            self.childs.insert(d, Node::new(word));
        }
    }

    fn dump(&self, d: &usize, level: usize) {
        for _ in 0..level * 4 {
            print!(" ");
        }
        println!("{}: {}", d, self.v);

        if self.childs.is_empty() {
            return;
        }

        for (k, v) in self.childs.iter() {
            let level = level + 1;
            v.dump(k, level);
        }
    }

    fn search(&self, word: &str) {
        println!("Try with {}", self.v);
        let d = lvd::lev(&self.v, word);

        if d < MAX_D {
            println!("{}", self.v);
        }

        let range = (d.abs_diff(MAX_D), d + MAX_D);

        for (_, child) in self
            .childs
            .iter()
            .filter(|(k, _)| *k > &range.0 && *k < &range.1)
        {
            child.search(word);
        }
    }
}

#[derive(Debug, Default)]
struct BkTree {
    root: Node,
}

impl BkTree {
    fn new(root: &str) -> Self {
        let root = Node::new(root);
        Self { root }
    }

    fn insert(&mut self, word: &str) {
        self.root.insert(word);
    }

    fn search(&self, word: &str) {
        self.root.search(word);
    }

    fn dump(&self) {
        self.root.dump(&0, 0);
    }
}
