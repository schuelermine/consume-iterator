/// Consume an iterator.
///
/// This function takes any implementation of [`IntoIterator`],
/// which includes iterators themselves.
///
/// # Example
///
/// The iterator is always fully consumed.
///
/// ```
/// # use consume_iterator::consume;
/// let mut range = 0..=10;
/// consume(&mut range);
/// assert!(range.is_empty());
/// ```
pub fn consume(iter: impl IntoIterator) {
    for _ in iter {}
}

/// Convenience trait to allow using [`consume`] as a method.
/// This trait is implemented for every [`Iterator`].
pub trait ConsumeIterator: Iterator {
    /// Consume an iterator.
    ///
    /// # Example
    ///
    /// The iterator is always fully consumed.
    ///
    /// ```
    /// # use consume_iterator::ConsumeIterator;
    /// let mut range = 0..=10;
    /// range.by_ref().consume();
    /// assert!(range.is_empty());
    /// ```
    fn consume(self)
    where
        Self: Sized,
    {
        consume(self);
    }
}

impl<T: Iterator> ConsumeIterator for T {}
