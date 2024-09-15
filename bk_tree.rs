mod lvd;

use std::collections::HashMap;
use std::io::{Read, Write};

const DEFAUTL_D: usize = 2;

fn main() {
    let mut file_name = None;
    let mut distance = DEFAUTL_D;
    let mut args = std::env::args().into_iter();
    args.next();

    if let Some(flag) = args.next() {
        if flag == "-f" {
            // println!("find -f");
            file_name = args.next();
        }

        if flag == "-d" {
            distance = args
                .next()
                .map_or(DEFAUTL_D, |x| x.parse::<usize>().unwrap_or(DEFAUTL_D));
        }
    }

    let words = if let Some(f) = file_name {
        let words = std::fs::read_to_string(f).unwrap();
        let words = words
            .split('\n')
            .map(|w| w.trim().to_lowercase())
            .collect::<Vec<String>>();
        words
    } else {
        vec![
            "book".into(),
            "books".into(),
            "earth".into(),
            "what".into(),
            "water".into(),
            "wavy".into(),
        ]
    };

    let mut bkt = BkTree::new("cook");

    for word in words {
        bkt.insert(&word);
    }

    print!(" > ");

    let _ = std::io::stdout().flush();
    for query in std::io::stdin().lines() {
        if let Ok(query) = query {
            if query == "exit" {
                std::process::exit(0x0100);
            }
            println!("Search for: {} with {distance}", query);
            bkt.search(&query, distance);
        }
        print!(" > ");
        let _ = std::io::stdout().flush();
    }
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

    fn search(&self, word: &str, distance: usize) {
        let d = lvd::lev(&self.v, word);

        if d <= distance {
            println!("{}", self.v);
        }

        let range = (d.abs_diff(distance), d + distance);

        // println!("{:?}", range);

        for (_, child) in self
            .childs
            .iter()
            .filter(|(k, _)| *k > &range.0 && *k < &range.1)
        {
            child.search(word, distance);
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
        println!("Adding {word} ..");
        self.root.insert(word);
    }

    fn search(&self, word: &str, d: usize) {
        self.root.search(word, d);
    }

    fn dump(&self) {
        self.root.dump(&0, 0);
    }
}
