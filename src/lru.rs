//! A least-recently-used cache, kept deliberately simple.

/// Holds up to `capacity` entries; adding one more drops the least recently used.
pub struct Lru<K, V> {
    capacity: usize,
    /// Most recently used last.
    entries: Vec<(K, V)>,
}

impl<K: PartialEq, V> Lru<K, V> {
    pub fn new(capacity: usize) -> Self {
        Lru {
            capacity,
            entries: Vec::with_capacity(capacity),
        }
    }

    // @card Why does `get` take `&mut self` in an LRU cache?
    // Reading an entry makes it the most recently used, so even a lookup changes
    // the cache's order.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let i = self.entries.iter().position(|(k, _)| k == key)?;
        let entry = self.entries.remove(i);
        self.entries.push(entry);
        self.entries.last().map(|(_, v)| v)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(i) = self.entries.iter().position(|(k, _)| *k == key) {
            self.entries.remove(i);
        } else if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }
        self.entries.push((key, value));
    }
}

#[cfg(test)]
mod tests {
    use super::Lru;

    #[test]
    fn get_protects_an_entry_from_eviction() {
        let mut cache = Lru::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("c", 3); // evicts "b", not the recently read "a"
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"a"), Some(&1));
    }
}
