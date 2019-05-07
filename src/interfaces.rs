use crate::generic_containers::*;
use core::borrow::Borrow;
use core::ops::{Index, IndexMut};

/// Trait for a container indexed by a value. The value that
/// indexes the container must implement the Eq trait to.
/// Supports insertion,
pub trait Map<K, V, E = (K, V)>: Container<E>
where
    K: Eq,
{
    /// Get a value from this Map. Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if none exists.
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq;

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

    /// Returns a mutable reference to an object stored in
    /// this container based on the key given, or `None` if
    /// the key does not exist.
    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq;

    /// Adds a new item into this container with the associated key,
    /// and returns the previous value associated with that key, if it existed.
    fn insert(&mut self, k: K, v: V) -> Option<V>;

    /// Removes an item from this container using the associated key,
    /// and returns it (if it existed).
    fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>;
}

/// Key-value map that also uses the `[`bracket`]` operators to access and modify
/// the internal data.
pub trait Dictionary<K, V>:
    Map<K, V, K> + DynamicContainer<K> + Index<K, Output = V> + IndexMut<K, Output = V>
where
    K: Eq,
{
}

/// Statically-sized array stored in the heap.
pub trait Array<V>: Map<usize, V> + Index<usize, Output = V> + IndexMut<usize, Output = V> {}

/// Dynamically changing array of values.
pub trait DynamicArray<V>:
    Map<usize, V, V> + DynamicContainer<V> + Index<usize, Output = V> + IndexMut<usize, Output = V>
{
}
