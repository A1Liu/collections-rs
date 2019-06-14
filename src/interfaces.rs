use crate::generic_containers::*;
use core::borrow::Borrow;
use core::ops::{Index, IndexMut};

/// Trait for a container indexed by a value that implements `Copy` and `Eq`.
pub trait CopyMap<K, V>: Container
where
    K: Copy + Eq,
{
    /// Get a reference to a value from this map.
    ///
    /// Takes a key by value
    /// and returns a reference to the corresponding data,
    /// or `None` if the key doesn't exist in the map.
    fn get(&self, key: K) -> Option<&V>;

    /// Get a mutable reference to a value from this map.
    ///
    /// Takes a key by value
    /// and returns a mutable reference to the corresponding data,
    /// or `None` if the key doesn't exist in the map.
    fn get_mut(&mut self, key: K) -> Option<&mut V>;

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. Note that the key itself isn't necessarily updated.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Trait for a container indexed by a value that implements `Eq`.
pub trait Map<K, V>: Container
where
    K: Eq,
{
    /// Returns a reference to a value from this Map.
    ///
    /// Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if the key doesn't exist in the map.
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq;

    /// Returns a mutable reference to a value in this map.
    ///
    /// Takes a key by reference
    /// and returns a mutable reference to the corresponding data,
    /// or `None` if the key doesn't exist in the map.
    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq;

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. Note that the key itself isn't necessarily updated.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Key-value map that can dynamically change size, indexed by a key that implements
/// `Copy`.
pub trait CopyDictionary<K, V>: CopyMap<K, V> + DynamicContainer
where
    K: Copy + Eq,
{
    /// Returns true if this container contains the key.
    fn contains(&self, key: K) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    /// Removes an item from this container using the associated key,
    /// and returns it (if it existed).
    fn remove(&mut self, k: K) -> Option<V>;
}

/// Key-value map that can dynamically change size.
pub trait Dictionary<K, V>: Map<K, V> + DynamicContainer
where
    K: Eq,
{
    /// Returns true if this container contains the key.
    fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    /// Removes an item from this container using the associated key,
    /// and returns it (if it existed).
    fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq;
}

/// Statically-sized array of values.
pub trait Array<V>:
    CopyMap<usize, V> + Index<usize, Output = V> + IndexMut<usize, Output = V>
{
}

/// Dynamically-sized array of values.
pub trait DynamicArray<V>:
    CopyMap<usize, V> + DynamicContainer + Index<usize, Output = V> + IndexMut<usize, Output = V>
{
}
