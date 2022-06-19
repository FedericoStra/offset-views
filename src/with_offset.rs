/// Functionality to add an offset to a value and convert it.
///
/// # Examples
///
/// ```rust
/// use offset_view::with_offset::WithOffset;
/// assert_eq!(3.with_offset(5), 8);
/// assert_eq!((3..8).with_offset(-3), 0..5);
/// ```
pub trait WithOffset<O> {
    type Output;
    /// Adds an offset to a value and tries to convert the result to a suitable output type.
    ///
    /// This method may panic if it is not possible to represent the result in the output type.
    fn with_offset(&self, offset: O) -> Self::Output;
}

impl WithOffset<isize> for isize {
    type Output = usize;
    fn with_offset(&self, offset: isize) -> usize {
        (self + offset)
            .try_into()
            .expect("cannot convert to `usize`")
    }
}

use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

impl<I> WithOffset<isize> for Range<I>
where
    I: WithOffset<isize, Output = usize>,
{
    type Output = Range<usize>;
    fn with_offset(&self, offset: isize) -> Range<usize> {
        self.start.with_offset(offset)..self.end.with_offset(offset)
    }
}

impl<I> WithOffset<isize> for RangeInclusive<I>
where
    I: WithOffset<isize, Output = usize>,
{
    type Output = RangeInclusive<usize>;
    fn with_offset(&self, offset: isize) -> RangeInclusive<usize> {
        self.start().with_offset(offset)..=self.end().with_offset(offset)
    }
}

impl<I> WithOffset<isize> for RangeFrom<I>
where
    I: WithOffset<isize, Output = usize>,
{
    type Output = RangeFrom<usize>;
    fn with_offset(&self, offset: isize) -> RangeFrom<usize> {
        self.start.with_offset(offset)..
    }
}

impl<I> WithOffset<isize> for RangeTo<I>
where
    I: WithOffset<isize, Output = usize>,
{
    type Output = RangeTo<usize>;
    fn with_offset(&self, offset: isize) -> RangeTo<usize> {
        ..self.end.with_offset(offset)
    }
}

impl<I> WithOffset<isize> for RangeToInclusive<I>
where
    I: WithOffset<isize, Output = usize>,
{
    type Output = RangeToInclusive<usize>;
    fn with_offset(&self, offset: isize) -> RangeToInclusive<usize> {
        ..=self.end.with_offset(offset)
    }
}

impl WithOffset<isize> for RangeFull {
    type Output = RangeFull;
    fn with_offset(&self, _offset: isize) -> RangeFull {
        RangeFull
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_panics;

    #[test]
    fn isize() {
        assert_eq!(0.with_offset(5), 5);
        assert_panics!(0.with_offset(-5), includes("cannot convert to `usize`"));
        assert_eq!(3.with_offset(3), 6);
        assert_eq!(3.with_offset(-3), 0);
        assert_panics!(3.with_offset(-5), includes("cannot convert to `usize`"));
    }

    #[test]
    fn range() {
        assert_eq!((0..10).with_offset(3), 3..13);
        assert_eq!((3..13).with_offset(-3), 0..10);
        assert_panics!(
            (3..13).with_offset(-5),
            includes("cannot convert to `usize`")
        );
    }

    #[test]
    fn range_inclusive() {
        assert_eq!((0..=10).with_offset(3), 3..=13);
        assert_eq!((3..=13).with_offset(-3), 0..=10);
        assert_panics!(
            (3..=13).with_offset(-5),
            includes("cannot convert to `usize`")
        );
    }

    #[test]
    fn range_from() {
        assert_eq!((0..).with_offset(3), 3..);
        assert_eq!((3..).with_offset(-3), 0..);
        assert_panics!((3..).with_offset(-5), includes("cannot convert to `usize`"));
    }

    #[test]
    fn range_to() {
        assert_eq!((..10).with_offset(3), ..13);
        assert_eq!((..13).with_offset(-3), ..10);
        assert_eq!((..13).with_offset(-5), (..8));
    }

    #[test]
    fn range_to_inclusive() {
        assert_eq!((..=10).with_offset(3), ..=13);
        assert_eq!((..=13).with_offset(-3), ..=10);
        assert_eq!((..=13).with_offset(-5), (..=8));
    }

    #[test]
    fn range_full() {
        assert_eq!(RangeFull.with_offset(5), RangeFull);
        assert_eq!(RangeFull.with_offset(-5), RangeFull);
    }
}
