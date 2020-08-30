use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Request {
    sub_header: [u8; 4],
    data_len: i8,
    data: RequestData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct RequestData {
    data: [u8; 4],
    command: [u8; 4],
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let a = "abcd".to_string();
        let world = Request {
            sub_header: a.as_bytes().try_into().unwrap(),
            data_len: 15,
            data: RequestData {
                data: b"abcd".clone(),
                command: b"efgh".clone(),
            },
        };

        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();
        println!("{:?}", encoded);

        // let bytes: [u8; 8] = [97, 98, 99, 100, 101, 102, 103, 104];
        let decoded: Request = bincode::deserialize(&encoded).unwrap();
        println!("{:?}", decoded);
    }
}
