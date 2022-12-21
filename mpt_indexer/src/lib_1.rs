use std::collections::HashMap;

enum Node {
    Branch(Vec<Option<Box<Node>>>),
    Leaf(Vec<u8>),
    Extension(Vec<u8>, Box<Node>),
}

struct MPT {
    root: Box<Node>,
    storage: HashMap<Vec<u8>, Vec<u8>>,
}

impl MPT {
    fn new() -> Self {
        let root = Box::new(Node::Branch(vec![Option::None; 16]));
        let storage = HashMap::new();
        MPT { root, storage }
    }

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let mut node = &mut self.root;
        let mut key_remainder = key;

        loop {
            match node {
                Node::Branch(children) => {
                    let next_child_index = key_remainder[0] as usize;
                    key_remainder = key_remainder[1..].to_vec();
                    match children[next_child_index] {
                        Some(ref mut child) => node = child,
                        None => {
                            let new_node = Box::new(Node::Leaf(key_remainder.clone()));
                            children[next_child_index] = Some(new_node);
                            self.storage.insert(key_remainder.clone(), value);
                            return;
                        }
                    }
                }
                Node::Leaf(stored_key) => {
                    if stored_key == key_remainder {
                        self.storage.insert(stored_key.clone(), value);
                        return;
                    } else {
                        let common_prefix_length = Self::common_prefix_length(stored_key, key_remainder.clone());
                        let mut new_extension_node = Node::Extension(
                            stored_key[..common_prefix_length].to_vec(),
                            Box::new(Node::Leaf(stored_key[common_prefix_length..].to_vec())),
                        );
                        std::mem::swap(node, &mut new_extension_node);
                        node = &mut new_extension_node;
                    }
                }
                Node::Extension(extension_key, ref mut child) => {
                    let common_prefix_length = Self::common_prefix_length(extension_key, key_remainder.clone());
                    if common_prefix_length == extension_key.len() {
                        key_remainder = key_remainder[common_prefix_length..].to_vec();
                        node = child;
                    } else {
                        let mut new_branch_node = Node::Branch(vec![None; 16]);
                        let mut new_extension_node = Node::Extension(
                            extension_key[common_prefix_length..].to_vec(),
                            Box::new(Node::Leaf(extension_key[common_prefix_length..].to_vec())),
                        );
                        let mut new_leaf_node = Node::Leaf(key_remainder[common_prefix_length..].to_vec());
                        std::mem::swap(child, &mut new_extension_node);
                        std::mem::swap(node, &mut new_branch_node);
                        new_branch_node.insert_at_index(extension_key[common_prefix_length - 1], new_extension_node);
                        new_branch_node.insert_at_index(key_remainder[common_prefix_length - 1], new_leaf_node);
                        self.storage.insert(key_remainder, value);
                        return;
                    }
                }
            }
        }
    }

    fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        let mut node = &self.root;
        let mut key_remainder = key;

        loop {
            match node {
                Node::Branch(children) => {
                    let next_child_index = key_remainder[0] as usize;
                    key_remainder = key_remainder[1..];
                    match children[next_child_index] {
                        Some(ref child) => node = child,
                        None => return None,
                    }
                }
                Node::Leaf(stored_key) => {
                    if stored_key == key_remainder {
                        return self.storage.get(stored_key);
                    } else {
                        return None;
                    }
                }
                Node::Extension(extension_key, ref child) => {
                    if extension_key == key_remainder[..extension_key.len()] {
                        key_remainder = key_remainder[extension_key.len()..];
                        node = child;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    fn common_prefix_length(key1: &[u8], key2: &[u8]) -> usize {
        let min_len = std::cmp::min(key1.len(), key2.len());
        for i in 0..min_len {
            if key1[i] != key2[i] {
                return i;
            }
        }
        min_len
    }
}

impl Node {
    fn insert_at_index(&mut self, index: u8, node: Node) {
        match self {
            Node::Branch(children) => {
                let index = index as usize;
                children[index] = Some(Box::new(node));
            }
            _ => panic!("Cannot insert at index for non-branch node"),
        }
    }
}

#[test]
fn test_insert_and_get(){
    let mut mpt = MPT::new();

    mpt.insert(vec![1, 2, 3], vec![4, 5, 6]);
    mpt.insert(vec![1, 2, 4], vec![7, 8, 9]);

    assert_eq!(mpt.get(&[1, 2, 3]), Some(&vec![4, 5, 6]));
    assert_eq!(mpt.get(&[1, 2, 4]), Some(&vec![7, 8, 9]));
    assert_eq!(mpt.get(&[1, 2, 5]), None);

}

#[test]
fn test_insert_duplicate_key() {
let mut mpt = MPT::new();
mpt.insert(vec![1, 2, 3], vec![4, 5, 6]);
mpt.insert(vec![1, 2, 3], vec![7, 8, 9]);

assert_eq!(mpt.get(&[1, 2, 3]), Some(&vec![7, 8, 9]));

}