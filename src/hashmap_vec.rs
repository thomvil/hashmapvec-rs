use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    usize,
};

#[derive(Default, Debug, Clone)]
pub struct HashMapVec<K: Clone + PartialEq + Eq + Hash, V: Default + Clone> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Clone + PartialEq + Eq + Hash, V: Default + Clone> HashMapVec<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<K, Vec<V>>::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::<K, Vec<V>>::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &Vec<V>> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Vec<V>> {
        self.inner.values_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &Vec<V>)> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut Vec<V>)> {
        self.inner.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.inner.keys().count()
    }

    pub fn nb_elements(&self) -> usize {
        self.inner.values().next().map(|v| v.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (K, Vec<V>)> + '_ {
        self.inner.drain()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        self.inner.get_mut(key)
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &Vec<V>)> {
        self.inner.get_key_value(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<Vec<V>> {
        self.inner.remove(key)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut Vec<V>) -> bool,
    {
        self.inner.retain(f)
    }

    pub fn insert(&mut self, key: K, value: V) {
        let nb_elems = self.nb_elements();
        for (_k, v_list) in self.inner.iter_mut().filter(|(k, _)| **k != key) {
            v_list.push(V::default());
        }
        self.inner
            .entry(key)
            .or_insert_with(|| vec![V::default(); nb_elems])
            .push(value)
    }

    pub fn add(&mut self, hm: HashMap<K, V>) {
        let (new_keys, missing_keys) = self.key_partition(&hm);
        let nb_elems = self.nb_elements();

        for key in new_keys {
            self.inner.insert(key.clone(), vec![V::default(); nb_elems]);
        }

        for (k, v) in hm {
            self.inner.get_mut(&k).unwrap().push(v);
        }

        for key in missing_keys {
            self.inner.get_mut(&key).unwrap().push(V::default());
        }
    }

    // private

    fn key_partition(&self, hm: &HashMap<K, V>) -> (HashSet<K>, HashSet<K>) {
        let new_keys = hm
            .keys()
            .cloned()
            .filter(|k| !self.inner.contains_key(k))
            .collect::<HashSet<_>>();
        let missing_keys = self
            .inner
            .keys()
            .cloned()
            .filter(|k| !hm.contains_key(k))
            .collect::<HashSet<_>>();
        (new_keys, missing_keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut hmv = HashMapVec::new();
        hmv.insert('a', 1);
        assert_eq!(Some(&vec![1]), hmv.get(&'a'));
        hmv.insert('a', 10);
        assert_eq!(Some(&vec![1, 10]), hmv.get(&'a'));
        hmv.insert('b', 8);
        assert_eq!(Some(&vec![1, 10, 0]), hmv.get(&'a'));
        assert_eq!(Some(&vec![0, 0, 8]), hmv.get(&'b'));
    }

    #[test]
    fn add() {
        let mut hmv = HashMapVec::new();
        hmv.insert('a', 1);
        assert_eq!(Some(&vec![1]), hmv.get(&'a'));

        let mut hm = HashMap::new();
        hm.insert('a', 1);
        hm.insert('b', 2);
        hmv.add(hm);
        assert_eq!(Some(&vec![1, 1]), hmv.get(&'a'));
        assert_eq!(Some(&vec![0, 2]), hmv.get(&'b'));

        let mut hm = HashMap::new();
        hm.insert('b', 22);
        hm.insert('c', 33);
        hmv.add(hm);
        assert_eq!(Some(&vec![1, 1, 0]), hmv.get(&'a'));
        assert_eq!(Some(&vec![0, 2, 22]), hmv.get(&'b'));
        assert_eq!(Some(&vec![0, 0, 33]), hmv.get(&'c'));
    }
}
