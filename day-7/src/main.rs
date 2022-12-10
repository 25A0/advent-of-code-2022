// The input file contains the signal as a series of characters.
//
// Approach:
// Parse the input file and build a file tree from that. Then iterate
// through the file tree, and enumerate all nodes that are directories
// and have a total size of < 10000.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

struct Node<'a> {
    name: &'a str,
    // parent: Option<&Node<'a>>,
    node_content: NodeContent<'a>,
}

enum NodeContent<'a> {
    File {
        size: usize,
    },
    Directory {
        children: Vec<&'a Node<'a>>,
    },
}

impl<'a> Node<'a> {
    fn total_size(&self) -> usize {
        match &self.node_content {
            NodeContent::File { size } => *size,
            NodeContent::Directory { children } => {
                let mut total_size = 0;

                for node in children {
                    total_size += node.total_size();
                }

                return total_size;
            }
        }
    }

    fn new_dir(name: &'a str) -> Node<'a> {
        Node {
            name: name,
            node_content: NodeContent::Directory {
                children: Vec::new(),
            }
        }
    }

    fn new_file(name: &'a str, size: usize) -> Node<'a> {
        Node {
            name: name,
            node_content: NodeContent::File {
                size: size,
            }
        }
    }

    fn add_child(&mut self, child: &'a Node<'a>) {
        if let NodeContent::File {..} = &self.node_content {
            panic!("Cannot add child to file node");
        }

        if let NodeContent::Directory {ref mut children} = self.node_content {
            children.push(child);
        }
    }

    fn print_tree(&self, indentation: usize) {
        println!("{}|-{}", " ".repeat(indentation * 2), self.name);
        if let NodeContent::Directory {children} = &self.node_content {
            for node in children {
                node.print_tree(indentation + 1);
            }
        }
    }


}

fn main() {

    // Build a file tree

    let mut tree = Node::new_dir("/");

    let file = Node::new_file("foo", 123);

    tree.add_child(&file);

    tree.print_tree(0);

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
