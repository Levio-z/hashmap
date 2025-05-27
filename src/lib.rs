use std::borrow::Borrow;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Index, IndexMut},
};
const ININIAL_NBUCKETS: usize = 1;
#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    item: usize,
}
pub struct OccupiedEntry<'a, K, V> {
    element: &'a mut (K, V),
}
pub struct VacantEntry<'a, K, V> {
    key: K,
    map: &'a mut HashMap<K, V>,
    index: usize,
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + Eq,
    {
        self.map.insert(self.key, value);
        &mut self.map.buckets[self.index].last_mut().unwrap().1
    }
}
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.element.1,
            Entry::Vacant(e) => e.insert(value),
        }
    }
    pub fn or_insert_with<F>(self, make: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.element.1,
            Entry::Vacant(e) => e.insert(make()),
        }
    }
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(|| Default::default())
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            item: 0,
        }
    }
}
impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        if self.buckets.is_empty() || self.item > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let index = self.bucket(&key);

        if let Some(pos) = self.buckets[index].iter().position(|(x, _)| x == &key) {
            let element = &mut self.buckets[index][pos];
            return Entry::Occupied(OccupiedEntry { element });
        }
        Entry::Vacant(VacantEntry {
            key,
            map: self,
            index,
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.item > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let index = self.bucket(&key);
        let bucket = &mut self.buckets[index];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                use core::mem;
                return Some(mem::replace(evalue, value));
            }
        }
        self.item += 1;
        bucket.push((key, value));
        None
    }
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => ININIAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (k, v) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((k, v));
        }
        self.buckets = new_buckets;
    }

    pub fn bucket<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.bucket(key);
        self.buckets[index]
            .iter()
            .find(|&&(ref x, _)| x.borrow() == (key))
            .map(|&(_, ref v)| v)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.bucket(key);
        self.buckets[index]
            .iter_mut()
            .find(|&&mut (ref x, _)| x.borrow() == key)
            .map(|&mut (_, ref mut v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.bucket(key);
        self.buckets[index]
            .iter()
            .find(|&&(ref x, _)| x.borrow() == key)
            .is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.bucket(key);
        let bucket = &mut self.buckets[index];
        let i = bucket.iter().position(|&(ref x, _)| x.borrow() == key)?;
        self.item -= 1;
        Some(bucket.swap_remove(i).1)
    }
    pub fn len(&self) -> usize {
        self.item
    }

    pub fn is_empty(&self) -> bool {
        self.item == 0
    }
}

pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.at) {
                    Some(&(ref k, ref v)) => {
                        self.at += 1;
                        break Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

impl<K, V> FromIterator<(K,V)> for  HashMap<K, V> 
where
    K: Hash + Eq,
{
    fn from_iter<I>(iter:I) ->Self
    where 
    I:IntoIterator<Item = (K,V)>
     {
        let mut map  = HashMap::new();
        for (k,v) in iter  {
            map.insert(k, v);
        }
        map
    }
}



impl<K, Q, V> Index<&Q> for HashMap<K, V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("Key not found")
    }
}

impl<K, Q, V> IndexMut<&Q> for HashMap<K, V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
{
    fn index_mut(&mut self, key: &Q) -> &mut Self::Output {
        self.get_mut(key).expect("Key not found")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.is_empty(), true);
        map.insert("key", "value");
        map.insert("key", "value1");
        map.insert("key2", "value1");
        // map.insert("key", "value");
        for i in &map {
            println!("{:?}", i)
        }

        assert_eq!(map.get(&"key"), Some(&"value1"));
        assert_eq!(map.remove(&"key"), Some("value1"));
        assert_eq!(map.get(&"key"), None);
        assert_eq!(map.len(), 1);
        for i in &map {
            println!("{:?}", i)
        }
    }
    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("foo", 23);
        map.insert("bar", 46);
        map.insert("baz", 23);
        for (&k, &v) in &map {
            match k {
                "foo" => assert_eq!(v, 23),
                "bar" => assert_eq!(v, 23),
                "baz" => assert_eq!(v, 23),
                _ => unreachable!(),
            }
        }
        assert_eq!((&map).into_iter().count(), 4);
    }
    #[test]
    fn index() {
        let mut map = HashMap::new();
        map.insert("foo", 23);
        assert_eq!(map["foo"], 23);
        map["foo"] = 3;
        assert_eq!(map["foo"], 3);
    }

    #[test]
    fn borrow() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), 23);
        assert_eq!(map.get("foo"), Some(&23));
        assert_eq!(map.contains_key("foo"), true);
    }
}
