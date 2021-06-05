use std::{hash::Hash, collections::HashMap};

#[derive(Debug, Clone)]
pub struct HashMapVec<K: PartialEq + Eq + Hash, V: Default + Clone> {
    inner: HashMap<K, Vec<V>>,
    default: V,
}

impl<K: PartialEq + Eq + Hash, V: Default + Clone> HashMapVec<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<K, Vec<V>>::new(),
            default: V::default(),
        }
    }

    pub fn nb_keys(&self) -> usize {
        self.inner.keys().count()
    }

    pub fn nb_elements(&self) -> usize {
        self.inner.values().next().map(|v| v.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.nb_keys() == 0
    }

    pub fn insert(&mut self, key: K, value: V) {
        let nb_elems = self.nb_elements();
        let default = self.default.clone();
        for (_k, v_list) in self.inner.iter_mut().filter(|(k, _)| **k != key) {
            v_list.push(self.default.clone());
        }
        self.inner
            .entry(key)
            .or_insert_with(|| vec![default; nb_elems])
            .push(value)
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
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
}
