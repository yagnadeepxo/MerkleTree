mod hasher;
use hasher::hash;

fn main() {
    
    let hash_value = hash("fukumean");
    println!("Hash: {}", hash_value);
}


