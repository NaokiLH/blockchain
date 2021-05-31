use bincode;
use serde::{Deserialize, Serialize};

pub fn my_serialize<T: ?Sized>(value: &T) -> bincode::Result<Vec<u8>>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    Ok(serialized)
}

pub fn my_deserialzie<'a, T>(bytes: &'a [u8]) -> bincode::Result<T>
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
