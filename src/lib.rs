pub mod with_offset;
pub use with_offset::WithOffset;

use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetSlice<'a, T> {
    parent: &'a [T],
    offset: isize,
}

impl<'a, T> OffsetSlice<'a, T> {
    pub fn new(parent: &'a [T], offset: isize) -> OffsetSlice<'a, T> {
        OffsetSlice { parent, offset }
    }
}

// impl<'a, T> Index<isize> for OffsetSlice<'a, T> {
//     type Output = T;
//     fn index(&self, index: isize) -> &Self::Output {
//         let index: usize = (index + self.offset)
//             .try_into()
//             .expect("cannot convert to `usize`");
//         self.parent.index(index)
//     }
// }

impl<'a, T, I> Index<I> for OffsetSlice<'a, T>
where
    I: WithOffset<isize>,
    I::Output: SliceIndex<[T]>,
{
    type Output = <I::Output as SliceIndex<[T]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        let index = index.with_offset(self.offset);
        self.parent.index(index)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OffsetSliceMut<'a, T> {
    parent: &'a mut [T],
    offset: isize,
}

impl<'a, T> OffsetSliceMut<'a, T> {
    pub fn new(parent: &'a mut [T], offset: isize) -> OffsetSliceMut<'a, T> {
        OffsetSliceMut { parent, offset }
    }
}

impl<'a, T, I> Index<I> for OffsetSliceMut<'a, T>
where
    I: WithOffset<isize>,
    I::Output: SliceIndex<[T]>,
{
    type Output = <I::Output as SliceIndex<[T]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        let index = index.with_offset(self.offset);
        self.parent.index(index)
    }
}

impl<'a, T, I> IndexMut<I> for OffsetSliceMut<'a, T>
where
    I: WithOffset<isize>,
    I::Output: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.with_offset(self.offset);
        self.parent.index_mut(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OffsetView<P, O> {
    parent: P,
    offset: O,
}

impl<P, O> OffsetView<P, O> {
    fn new(parent: P, offset: O) -> OffsetView<P, O> {
        OffsetView { parent, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_panics;

    #[test]
    fn offset_slice_works() {
        let a = [0, 1, 2, 3, 4, 5];
        let os = OffsetSlice::new(&a, -3isize);
        for i in 3..=8 {
            assert_eq!(os[i], i - 3);
        }
        assert_panics!(os[2], includes("cannot convert to `usize`"));
        assert_panics!(os[9], includes("index out of bounds"));
    }

    #[test]
    fn offset_slice_mut_works() {
        let mut a = [0, 1, 2, 3, 4, 5];
        let mut os = OffsetSliceMut::new(&mut a, -3isize);
        for i in 3..=8 {
            assert_eq!(os[i], i - 3);
            os[i] = 0;
        }
        assert_panics!(os[2], includes("cannot convert to `usize`"));
        assert_panics!(os[9], includes("index out of bounds"));
        for i in 3..=8 {
            assert_eq!(os[i], 0);
        }
    }

    #[test]
    fn can_create_from_string() {
        let parent = String::from("Hello!");
        let view = OffsetView::new(&parent, -2);
        assert_eq!(view.parent, &parent);
        assert_eq!(view.offset, -2);
    }

    #[test]
    fn can_create_from_array() {
        let parent = [0, 1, 2, 3];
        let view = OffsetView::new(&parent, -2);
        assert_eq!(view.parent, &parent);
        assert_eq!(view.offset, -2);
    }
}
