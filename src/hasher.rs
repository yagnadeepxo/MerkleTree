
use sha2::{Sha256};
use hex;

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
