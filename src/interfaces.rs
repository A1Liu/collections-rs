use crate::generic_containers::*;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut, Index, IndexMut};

/// Trait for a container indexed by a value that implements `Copy` and `Eq`.
pub trait CopyMap<'a, K, V, E = V, R = &'a V, Rm = &'a mut V>: Container<V>
where
    K: Copy + Eq,
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
{
    /// Get a value from this Map. Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if none exists.
    fn get(&self, key: K) -> Option<R>;

    /// Returns a mutable reference to an object stored in
    /// this container based on the key given, or `None` if
    /// the key does not exist.
    fn get_mut(&mut self, key: K) -> Option<Rm>;

    /// Adds a new item into this container with the associated key,
    /// and returns the previous value associated with that key, if it existed.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Trait for a container indexed by a value that implements `Eq`.
pub trait Map<'a, K, V, E = V, R = &'a V, Rm = &'a mut V>: Container<V>
where
    K: Eq,
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
{
    /// Get a value from this Map. Takes a key by reference
    /// and returns a reference to the corresponding data,
    /// or `None` if none exists.
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<R>
    where
        K: Borrow<Q>,
        Q: Eq;

    /// Returns a mutable reference to an object stored in
    /// this container based on the key given, or `None` if
    /// the key does not exist.
    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<Rm>
    where
        K: Borrow<Q>,
        Q: Eq;

    /// Adds a new item into this container with the associated key,
    /// and returns the previous value associated with that key, if it existed.
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Key-value map that also uses the `[`bracket`]` operators to access and modify
/// the internal data.
pub trait CopyDictionary<'a, K, V, R = &'a V, Rm = &'a mut V>:
    CopyMap<'a, K, V, V, R, Rm> + DynamicContainer<K> + Index<K, Output = V> + IndexMut<K, Output = V>
where
    K: Copy + Eq,
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
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

/// Key-value map that also uses the `[`bracket`]` operators to access and modify
/// the internal data.
pub trait Dictionary<'a, K, V, R = &'a V, Rm = &'a mut V>:
    Map<'a, K, V, V, R, Rm> + DynamicContainer<K> + Index<K, Output = V> + IndexMut<K, Output = V>
where
    K: Eq,
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
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

/// Statically-sized array stored in the heap.
pub trait Array<'a, V, R = &'a V, Rm = &'a mut V>:
    CopyMap<'a, usize, V, (usize, V), R, Rm> + Index<usize, Output = V> + IndexMut<usize, Output = V>
where
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
{
}

/// Dynamically changing array of values.
pub trait DynamicArray<'a, V, R = &'a V, Rm = &'a mut V>:
    CopyMap<'a, usize, V, V, R, Rm>
    + DynamicContainer<V>
    + Index<usize, Output = V>
    + IndexMut<usize, Output = V>
where
    V: 'a,
    R: Deref<Target = V>,
    Rm: DerefMut<Target = V>,
{
}
