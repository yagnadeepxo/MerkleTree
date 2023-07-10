// mod hasher;
// use hasher::hash;

// fn main() {
//     let out : String = hash("5");
//     println!("{}",out);
// }

use sha2::{Digest, Sha256};
use hex;

#[derive(Debug)]
pub struct Node {
    hash: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn hash<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    let mut hasher = Sha256::new();

    hasher.update(input);

    let result = hasher.finalize();

    let hash_string = hex::encode(result);

    hash_string
}

pub fn build_tree<T>(data: &[T]) -> Option<Box<Node>>
where
    T: AsRef<[u8]>,
{
    if data.is_empty() {
        return None;
    }

    let mut hashes: Vec<String> = Vec::new();

    for item in data {
        hashes.push(hash(item));
    }

    if hashes.len() == 1 {
        return Some(Box::new(Node {
            hash: hashes[0].clone(),
            left: None,
            right: None,
        }));
    }

    let mut child_hashes: Vec<String> = Vec::new();

    if hashes.len() % 2 == 1 {
        hashes.push(hashes[hashes.len() - 1].clone());
    }

    for i in (0..hashes.len()).step_by(2) {
        let hash = hash_nodes(&hashes[i], &hashes[i + 1]);
        child_hashes.push(hash);
    }

    build_tree(&child_hashes).map(|subtree| {
        Some(Box::new(Node {
            hash: hash_nodes(&subtree.hash, &subtree.hash),
            left: Some(subtree),
            right: None,
        }))
    }).flatten()
}

fn hash_nodes(left: &str, right: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hex::decode(left).unwrap());
    hasher.update(hex::decode(right).unwrap());
    hex::encode(hasher.finalize())
}

fn main() {
    let data = [
        "hello".as_bytes(),
        "world".as_bytes(),
        "merkle".as_bytes(),
        "tree".as_bytes(),
    ];

    let root = build_tree(&data);

    println!("Merkle Root: {:?}", root);
}

