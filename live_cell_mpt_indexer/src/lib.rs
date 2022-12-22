pub mod indexer;
pub mod trie;


use std::collections::BTreeMap;
use std::mem;
use std::clone::Clone;
use std::rc::Rc;

use rustc_serialize::Encodable;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
// use bincode::encode;
// use bincode::ErrorKind::SizeLimit;
// use rustc_serialize::json::encode;


/// Public Functios:
/// fn new() -> MerklePatriciaTree<T> create new node
/// fn get(&self, key: &'a [u8]) -> Option<&T> return value with a given key
/// fn insert(&mut self, key: &'a [u8], Option<T>) insert given
/// value with key into tree (panic if key exist)
/// fn remove(&mut self, key: &'a [u8]) -> Option<T> remove
/// (functions return None when value not exist)

pub trait Merkle<'a, T: 'a>
    where T: Encodable + serde::ser::Serialize
{
    fn new() -> MerklePatriciaTree<T>;
    fn insert(&mut self, key: &'a [u8], value: Option<T>);
    fn get(&self, key: &'a [u8]) -> Option<&T>;
    fn remove(&mut self, key: &'a [u8]) -> Option<T>;
}

pub struct MerklePatriciaTree<T>
    where T: Encodable + serde::ser::Serialize
{
    value: Option<T>,
    key: Vec<u8>,
    hash: [u8; 32],
    children: BTreeMap<u8, MerklePatriciaTree<T>>,
}

impl<'a, T> Merkle<'a, T> for MerklePatriciaTree<T>
    where T: Encodable + 'a  + serde::ser::Serialize
{
    fn new() -> MerklePatriciaTree<T> {
        MerklePatriciaTree::<T> {
            value: None,
            key: Vec::new(),
            children: BTreeMap::new(),
            hash: [0; 32],
        }
    }

    fn get(&self, key: &'a [u8]) -> Option<&T> {
        if self.key.is_empty() || key.as_ref().starts_with(&self.key) {
            if key.as_ref().len() == self.key.len() {
                return self.value.as_ref();
            } else {
                let temp = &key.as_ref()[self.key.len()..];
                let mut suffix: Vec<u8> = Vec::new();
                suffix.extend_from_slice(temp);
                return self.children[&suffix[0]].get(&Rc::new(suffix));
            }
        }
        None
    }

    fn insert(&mut self, key: &'a [u8], value: Option<T>) {
        if self.is_empty() {
            self.key.extend_from_slice(key);
            self.value = value;
        } else {
            let max_matched_length = |a: &[u8], b: &[u8]| {
                let mut count: usize = 0;
                while count < a.as_ref().len() && count < b.as_ref().len() && a[count] == b[count] {
                    count += 1;
                }
                count
            };

            let length = max_matched_length(&self.key, key);

            if length >= self.key.len() {
                if length == key.len() {
                    if self.value.is_some() {
                        panic!("key exists");
                    }

                    self.value = value;
                    self.update_hash();
                } else {
                    let temp = &key[length..];
                    let mut suffix: Vec<u8> = Vec::new();
                    suffix.extend_from_slice(temp);
                    self.insert_predecessor(&suffix, value.expect("this is the end"));
                }
            } else {
                let suffix = self.key[length..].to_vec();
                let prefix = self.key[0..length].to_vec();

                self.key = prefix;

                let temp = self.value.take();
                let mut node: MerklePatriciaTree<T> =
                    MerklePatriciaTree::<T>::create_node_with_args(temp,
                                                                   suffix,
                                                                   [0; 32],
                                                                   BTreeMap::new());

                mem::swap(&mut node.children, &mut self.children);
                node.update_hash();

                self.children.clear();
                self.children.insert(node.key[0], node);

                if length == key.len() {
                    self.value = value;
                } else {
                    self.value = None;
                    let temp = &key[length..];
                    let mut suffix: Vec<u8> = Vec::new();
                    suffix.extend_from_slice(temp);
                    self.insert_predecessor(&suffix, value.expect("this is the end"));
                }
            }
        }
        self.update_hash();
    }

    fn remove(&mut self, key: &'a [u8]) -> Option<T> {
        if self.key.is_empty() || key.as_ref().starts_with(&self.key) {
            if key.len() == self.key.len() {
                let value = self.value.take();
                self.value = None;
                self.try_to_compress();
                return value;
            } else if key.as_ref().len() > self.key.len() {
                let temp = &key[self.key.len()..];
                let mut suffix: Vec<u8> = Vec::new();
                suffix.extend_from_slice(temp);
                let mut value = None;
                if let Some(mut node) = self.children.get_mut(&suffix[0]) {
                    value = node.remove(&Rc::new(suffix.clone()));
                }

                if value.is_some() {
                    if self.children[&suffix[0]].is_empty() {
                        self.children.remove(&suffix[0]);
                    }
                    self.try_to_compress();
                    return value;
                }
            }
        }
        None
    }
}

impl<T> MerklePatriciaTree<T>
    where T: Encodable + serde::ser::Serialize
{
    fn create_node_with_args(value: Option<T>,
                             key: Vec<u8>,
                             hash: [u8; 32],
                             children: BTreeMap<u8, MerklePatriciaTree<T>>)
                             -> MerklePatriciaTree<T> {
        MerklePatriciaTree::<T> {
            value: value,
            key: key,
            hash: hash,
            children: children,
        }
    }

    fn add_hashed(&self, out: &mut Sha256, what: &str) {
        let mut hash = [0; 32];
        let mut hasher = Sha256::new();

        match what {
            "value" => {
                let input = bincode::serialize(&self.value).unwrap();

                hasher.input(&input);
                hasher.result(&mut hash);
            }
            "key" => {
                hasher.input(&self.key);
                hasher.result(&mut hash);
            }
            _ => panic!("incorrect value"),
        }

        out.input(&hash);

    }

    fn update_hash(&mut self) {
        if self.is_empty() {
            self.hash = [0; 32];
        } else {
            let mut hasher = Sha256::new();

            self.add_hashed(&mut hasher, "key");
            if self.value.is_some() {
                self.add_hashed(&mut hasher, "value");
            }

            for child in self.children.values() {
                hasher.input(&child.hash);
            }

            hasher.result(&mut self.hash);
        }
    }

    fn try_to_compress(&mut self) {
        if self.is_empty() {
            self.key.clear();
        } else if self.value.is_none() && self.children.len() == 1 {
            let mut new_children = BTreeMap::new();
            {
                let child;
                {
                    let (_, node) = self.children.iter_mut().next().unwrap();
                    child = node;
                }

                self.key.extend_from_slice(&child.key);
                self.value = child.value.take();
                mem::swap(&mut new_children, &mut &mut child.children);
            }
            mem::swap(&mut self.children, &mut new_children);
        }
        self.update_hash();
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty() && self.value.is_none()
    }

    fn insert_predecessor(&mut self, suffix: &Vec<u8>, value: T) {
        let child_to_push = match self.children.get_mut(&suffix[0]) {
            Some(pred) => {
                pred.insert(&suffix, Some(value));
                None
            }
            None => {
                let mut child = MerklePatriciaTree::new();
                child.insert(&suffix, Some(value));
                Some(child)
            }
        };
        if let Some(child) = child_to_push {
            self.children.insert(child.key[0], child);
        }
    }
}

impl<'a, T> PartialEq for MerklePatriciaTree<T>
    where T: Clone + Encodable + serde::ser::Serialize
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<'a, T> Eq for MerklePatriciaTree<T> where T: Clone + Encodable + serde::ser::Serialize {}

