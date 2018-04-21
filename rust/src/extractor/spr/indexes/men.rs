// character >> animation >> variation (usually orientation) >> frames

use std::collections::HashMap;

pub fn get_index<'a>() -> HashMap<&'a str, HashMap<&'a str, HashMap<&'a str, Vec<u32>>>> {
    hashmap!{
        "worker" => hashmap!{
            "walk" => hashmap!{
                "right" => vec![0, 2, 4, 6],
                "up" => vec![1, 3, 5, 7],
            },
            "idle" => hashmap!{
                "right" => vec![8, 9, 10, 9, 8],
                "down" => vec![11, 12, 13, 14, 15, 16, 17, 16, 15]
            }
        }
    }
}
