use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};
pub fn my_serialize<T: ?Sized>(value: &T) -> bincode::Result<Vec<u8>>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    Ok(serialized)
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> bincode::Result<T>
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    Ok(deserialized)
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
