// The input file contains the signal as a series of characters.
//
// Approach:
// Parse the input file and build a file tree from that. Then iterate
// through the file tree, and enumerate all nodes that are directories
// and have a total size of < 10000.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

enum Node<'a> {
    File {
        name: String,
        parent: Option<&'a Node<'a>>,
        next_sibling: Option<&'a Node<'a>>,
        size: usize,
    },
    Directory {
        name: String,
        parent: Option<&'a Node<'a>>,
        next_sibling: Option<&'a Node<'a>>,
        first_child: Option<&'a Node<'a>>,
        // last_child: Option<Node>,
    },
}

impl<'a> Iterator for Node<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Node::File{name, parent, next_sibling, size} => {
                if let Some(node) = self.next_sibling {
                    return Some(node)
                }
                return self.parent
            },
            Node::Directory{name, parent, next_sibling, first_child} => {
                // If this directory has children then iterate
                // over those children first
                if let Some(node) = self.first_child {
                    return Some(&node)
                }
                // Otherwise iterate to the next sibling of this node
                // if there is such a sibling
                if let Some(node) = self.next_sibling {
                    return Some(node)
                }
                // Otherwise return to the parent level
                return self.parent
            },
        }
    }
}

impl Node<'_> {
    fn total_size(&self) -> usize {
        match self {
            Node::File { name, parent, next_sibling, size } => self.size,
            Node::Directory { name, parent, next_sibling, first_child } => {
                let total_size = 0;

                let child = self.first_child;
                loop {
                    match child {
                        Some(node) => {
                            total_size += child.total_size();
                            child = child.next_sibling;
                        },
                        None => break,
                    }
                }
                total_size
            }
        }
    }
}

// impl Node::Directory {
//     fn add_child(&mut self, child: Node) {
//         self.first_child = Some(child);
//         child.parent = &self;
//     }
// }

fn main() {

    // Build a file tree

    let tree = Node::Directory {
        name: "/",
        next_sibling: None,
        first_child: None,
    };

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {

        }
    }

}
