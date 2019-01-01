//! A two-way map data structure for cloneable keys and values.
//!
//! Most functions come in `_fwd` and `_rev` variants; where the `_fwd` variant acts on the second
//! entry given the first, and `_rev` is the opposite.
//!
//! This crate is best for values that are cheap to clone, since internally it stores two copies
//! of each element. To use it with large values, consider wrapping them in `Rc` to make them cheap
//! to clone.

use std::borrow::Borrow;
use std::collections::hash_map::*;
use std::default::Default;
use std::hash::{BuildHasher, Hash};

#[derive(Clone)]
pub struct Bimap<K, V, S = RandomState> {
    fwd: HashMap<K, V, S>,
    rev: HashMap<V, K, S>,
}

impl<K, V> Bimap<K, V, RandomState>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Creates an empty `Bimap`.
    pub fn new() -> Self {
        Self {
            fwd: HashMap::new(),
            rev: HashMap::new(),
        }
    }
}

impl<K, V, S> Bimap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
    S: BuildHasher + Clone + Default,
{
    /// Creates a `Bimap` with the given hasher.
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            fwd: HashMap::with_hasher(hash_builder.clone()),
            rev: HashMap::with_hasher(hash_builder),
        }
    }

    /// Creates a bimap from a `HashMap`.
    pub fn from_hash_map(fwd: HashMap<K, V, S>) -> Self {
        let rev = fwd.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
        Self { fwd, rev }
    }

    /// Returns the number of elements in the bimap.
    pub fn len(&self) -> usize {
        self.fwd.len()
    }

    /// Returns whether the bimap is empty.
    pub fn is_empty(&self) -> bool {
        self.fwd.is_empty()
    }

    /// Removes all elements from the bimap.
    pub fn clear(&mut self) {
        self.fwd.clear();
        self.rev.clear();
    }

    /// Inserts a (key, value) pair into the bimap. Panics if either the key or value is already
    /// present in the bimap; to change a key or value, call either `remove_fwd` or
    /// `remove_rev` before inserting the new (key, value) pair.
    pub fn insert(&mut self, k: K, v: V) {
        match self.fwd.entry(k.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(v.clone());
            }
            Entry::Occupied(_) => panic!("Element aready in bimap"),
        }
        match self.rev.entry(v) {
            Entry::Vacant(entry) => {
                entry.insert(k);
            }
            Entry::Occupied(_) => panic!("Element aready in bimap"),
        }
    }

    /// Gets the value corresponding to a key.
    pub fn get_fwd<KeyBorrow: ?Sized>(&self, k: &KeyBorrow) -> Option<&V>
    where
        K: Borrow<KeyBorrow>,
        KeyBorrow: Hash + Eq,
    {
        self.fwd.get(k)
    }

    /// Gets the key corresponding to a value.
    pub fn get_rev<ValBorrow: ?Sized>(&self, v: &ValBorrow) -> Option<&K>
    where
        V: Borrow<ValBorrow>,
        ValBorrow: Hash + Eq,
    {
        self.rev.get(v)
    }

    /// Removes the (key, value) pair with the given key; returns the corresponding value.
    pub fn remove_fwd<KeyBorrow: ?Sized>(&mut self, k: &KeyBorrow) -> V
    where
        K: Borrow<KeyBorrow>,
        KeyBorrow: Hash + Eq,
    {
        let v = self.fwd.remove(k).unwrap();
        self.rev.remove(&v);
        v
    }

    /// Removes the (key, value) pair with the given value; returns the corresponding key.
    pub fn remove_rev<ValBorrow: ?Sized>(&mut self, v: &ValBorrow) -> K
    where
        V: Borrow<ValBorrow>,
        ValBorrow: Hash + Eq,
    {
        let k = self.rev.remove(v).unwrap();
        self.fwd.remove(&k);
        k
    }

    /// Returns whether the bimap contains a (key, value) pair with the given key.
    pub fn contains_fwd<KeyBorrow: ?Sized>(&self, k: &KeyBorrow) -> bool
    where
        K: Borrow<KeyBorrow>,
        KeyBorrow: Hash + Eq,
    {
        self.fwd.contains_key(k)
    }

    /// Returns whether the bimap contains a (key, value) pair with the given value.
    pub fn contains_rev<ValBorrow: ?Sized>(&self, v: &ValBorrow) -> bool
    where
        V: Borrow<ValBorrow>,
        ValBorrow: Hash + Eq,
    {
        self.rev.contains_key(v)
    }

    /// Iterates over all (key, value) pairs in the bimap.
    pub fn iter(&self) -> Iter<K, V> {
        self.fwd.iter()
    }
}

impl<K, V, S> Default for Bimap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
    S: BuildHasher + Clone + Default,
{
    fn default() -> Self {
        Bimap::with_hasher(Default::default())
    }
}
