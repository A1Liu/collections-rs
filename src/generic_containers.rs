use core::ops::{Deref, DerefMut};

/// Trait for a simple container.
pub trait Container<E> {
    /// Add an element to the container.
    fn add(&mut self, element: E);
    /// Get the size of the container.
    fn len(&self) -> usize;
}

/// Trait that represents dynamically growing containers.
pub trait DynamicContainer<E>: Container<E> {
    /// Reserves capacity for at least `additional` more elements to be
    /// inserted in the container. The collection may reserve more space to avoid
    /// frequent reallocations.
    fn reserve(&mut self, additional: usize);

    /// Shrinks the capacity of the container as much as possible.
    /// It will drop down as much as possible while maintaining the internal
    /// rules and possibly leaving some space in accordance with the resize policy.
    fn shrink_to_fit(&mut self);

    /// Returns the number of elements the container can hold without reallocating.
    fn capacity(&self) -> usize;
}
