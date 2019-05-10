use crate::generic_containers::*;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut, Index, IndexMut};

/// Trait for a container indexed by a value that implements `Copy` and `Eq`.
pub trait CopyMap<K, V, E = (K, V)>: Container<E>
where
    K: Copy + Eq,
{
    /// Get a value from this Map. Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if none exists.
    fn get<R>(&self, key: K) -> Option<R>
    where
        R: Deref<Target = V>;

    /// Returns a mutable reference to an object stored in
    /// this container based on the key given, or `None` if
    /// the key does not exist.
    fn get_mut<R>(&mut self, key: K) -> Option<R>
    where
        R: DerefMut<Target = V>;

    /// Adds a new item into this container with the associated key,
    /// and returns the previous value associated with that key, if it existed.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Trait for a container indexed by a value that implements `Eq`.
pub trait Map<K, V, E = (K, V)>: Container<E>
where
    K: Eq,
{
    /// Get a value from this Map. Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if none exists.
    fn get<Q, R>(&self, key: &Q) -> Option<R>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
        R: Deref<Target = V>;

    /// Returns a mutable reference to an object stored in
    /// this container based on the key given, or `None` if
    /// the key does not exist.
    fn get_mut<Q, R>(&mut self, k: &Q) -> Option<R>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
        R: DerefMut<Target = V>;

    /// Adds a new item into this container with the associated key,
    /// and returns the previous value associated with that key, if it existed.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Key-value map that also uses the `[`bracket`]` operators to access and modify
/// the internal data.
pub trait CopyDictionary<K, V>:
    CopyMap<K, V, K> + DynamicContainer<K> + Index<K, Output = V> + IndexMut<K, Output = V>
where
    K: Copy + Eq,
{
    /// Returns true if this container contains the key.
    fn contains<R>(&self, key: K) -> bool
    where
        R: Deref<Target = V>,
    {
        match self.get::<R>(key) {
            Some(_) => true,
            None => false,
        }
    }

    /// Removes an item from this container using the associated key,
    /// and returns it (if it existed).
    fn remove(&mut self, k: K) -> Option<V>;
}

/// Key-value map that also uses the `[`bracket`]` operators to access and modify
/// the internal data.
pub trait Dictionary<K, V>:
    Map<K, V, K> + DynamicContainer<K> + Index<K, Output = V> + IndexMut<K, Output = V>
where
    K: Eq,
{
    /// Returns true if this container contains the key.
    fn contains<Q, R>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
        R: Deref<Target = V>,
    {
        match self.get::<Q, R>(key) {
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

/// Statically-sized array stored in the heap.
pub trait Array<V>:
    CopyMap<usize, V> + Index<usize, Output = V> + IndexMut<usize, Output = V>
{
}

/// Dynamically changing array of values.
pub trait DynamicArray<V>:
    CopyMap<usize, V, V> + DynamicContainer<V> + Index<usize, Output = V> + IndexMut<usize, Output = V>
{
}
