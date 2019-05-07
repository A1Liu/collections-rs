/// Trait for a simple container.
pub trait Container<E> {
    fn add(element: E);
    fn len() -> usize;
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

/// Trait for objects that can be iterated over.
pub trait Iterable<'a, E, ERef = &'a E, EMutRef = &'a mut E>: IntoIterator
where
    Self: 'a,
    &'a Self: IntoIterator<Item = ERef>,
    &'a mut Self: IntoIterator<Item = EMutRef>,
{
}
