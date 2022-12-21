// use std::collections::HashMap;
// use sha2::{Digest, Sha256};

// struct MPTIndexer {
//     data: HashMap<Vec<u8>, Vec<u8>>,
// }

// impl MPTIndexer {
//     fn new() -> Self {
//         MPTIndexer { data: HashMap::new() }
//     }

//     fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
//         self.data.insert(key, value);
//     }

//     fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
//         self.data.get(key)
//     }

//     fn compute_hash(data: &[u8]) -> Vec<u8> {
//         let mut hasher = Sha256::new();
//         hasher.input(data);
//         hasher.result().to_vec()
//     }
// }
//     fn main() {
//         let mut indexer = MPTIndexer::new();
//         indexer.insert(b"key1".to_vec(), b"value1".to_vec());
//         indexer.insert(b"key2".to_vec(), b"value2".to_vec());

//         let value = indexer.get(b"key1");
//         println!("{:?}", value); // Some("value1")

//         let hash = MPTIndexer::compute_hash(b"key1");
//         println!("{:?}", hash); // a91e38cdd5e2b2a5a7a14f6f29b86f1a3c3df3bea6c1b36d12cfc8e897b7d9b4
//     }

//     #[test]
//     fn test_insert_and_get() {
//         let mut indexer = MPTIndexer::new();
//         indexer.insert(b"key1".to_vec(), b"value1".to_vec());
//         let value = indexer.get(b"key1");
//         assert_eq!(value, Some(b"value1"));
//     }

//     #[test]
//     fn test_get_not_found() {
//         let indexer = MPTIndexer::new();
//         let value = indexer.get(b"key2");
//         assert_eq!(value, None);
//     }

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::vec::Vec;

struct MPTNode {
    key: Vec<u8>,
    value: Vec<u8>,
    children: HashMap<Vec<u8>, MPTNode>,
}
impl MPTNode {
    fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
        MPTNode {
            key,
            value,
            children: HashMap::new(),
        }
    }
}

struct MPTIndexer {
    root: MPTNode,
}

impl MPTIndexer {
    fn new() -> Self {
        MPTIndexer {
            root: MPTNode::new(Vec::new(), Vec::new()),
        }
    }

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let mut node = &mut self.root;
        for i in 0..key.len() {
            let child_key = &key[i..];
            let child = node
                .children
                .entry(child_key.to_vec())
                .or_insert(MPTNode::new(child_key.to_vec(), Vec::new()));
            if i == key.len() - 1 {
                child.value = value.clone();
            }
            node = child;
        }
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let mut node = &self.root;
        for i in 0..key.len() {
            let child_key = &key[i..];
            let child = node.children.get(child_key)?;
            if i == key.len() - 1 {
                return Some(&child.value);
            }
            node = child;
        }
        None
    }
}

impl Hash for MPTNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

fn main() {
    let mut indexer = MPTIndexer::new();
    indexer.insert(b"key1".to_vec(), b"value1".to_vec());
    let value = indexer.get(b"key1");
}
