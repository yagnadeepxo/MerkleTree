use sha2::{Digest, Sha256};
use hex;
pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input);

    let result = hasher.finalize();

    let hash_string = hex::encode(result);

    return hash_string;
}


