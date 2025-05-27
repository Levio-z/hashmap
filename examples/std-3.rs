extern crate hashmap;
use crate::hashmap::HashMap;
fn main() {
    let vec = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<_, _> = vec.into_iter().collect();
    assert_eq!(map.get("a"), Some((&1)));
}
