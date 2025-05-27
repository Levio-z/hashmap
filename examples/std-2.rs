extern crate hashmap;
use crate::hashmap::HashMap;
fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
    *map.entry("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);

    let mut map = HashMap::new();
    let value = "hoho";
    map.entry("poneyland").or_insert_with(|| value);
    assert_eq!(map["poneyland"], "hoho");

    // let mut map: HashMap<&str, usize> = HashMap::new();
    // map.entry("poneyland")
    //     .or_insert_with_key(|key| key.chars().count());
    // assert_eq!(map["poneyland"], 9);

    // let mut map: HashMap<&str, u32> = HashMap::new();
    // assert_eq!(map.entry("poneyland").key(), &"poneyland");

    // let mut map: HashMap<&str, u32> = HashMap::new();
    // map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
    // assert_eq!(map["poneyland"], 42);
    // map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
    // assert_eq!(map["poneyland"], 43);

    // let mut map: HashMap<&str, String> = HashMap::new();
    // let entry = map.entry("poneyland").insert_entry("hoho".to_string());
    // assert_eq!(entry.key(), &"poneyland");

    // let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    // map.entry("poneyland").or_default();
    // assert_eq!(map["poneyland"], None);
}
