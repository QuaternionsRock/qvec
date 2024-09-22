#![allow(incomplete_features)]
#![feature(specialization)]

mod copy;
mod drop;
mod leaky;
mod maybe_uninit_ext;
mod traits;

use std::ops::{Deref, DerefMut};

pub use leaky::LeakyQVec;
use traits::QVecType;

#[derive(Clone, Copy)]
pub struct QVec<T, const C: usize>(<LeakyQVec<T, C> as QVecType>::QVecType);

impl<T, const C: usize> QVec<T, C> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        self.0.push_unchecked(value)
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        self.0.push_within_capacity(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, const C: usize> Default for QVec<T, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const C: usize> Deref for QVec<T, C> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const C: usize> DerefMut for QVec<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn clone() {
        let mut qv: QVec<_, 3> = QVec::new();
        assert_eq!(qv.capacity(), 3);
        assert_eq!(qv.len(), 0);

        qv.push("a".to_owned());
        assert_eq!(qv.len(), 1);
        assert_eq!(qv[0], "a");

        qv.push("b".to_owned());
        assert_eq!(qv.len(), 2);
        assert_eq!(qv[1], "b");

        qv.push("c".to_owned());
        assert_eq!(qv.len(), 3);
        assert_eq!(qv[2], "c");

        assert_eq!(qv.push_within_capacity("d".to_owned()), Err("d".to_owned()));
        assert_eq!(qv.len(), 3);

        let clone = qv.clone();
        assert_eq!(clone.len(), 3);
        assert_eq!(*clone, ["a", "b", "c"]);

        assert_eq!(qv.pop(), Some("c".to_owned()));
        assert_eq!(qv.len(), 2);

        assert_eq!(qv.pop(), Some("b".to_owned()));
        assert_eq!(qv.len(), 1);

        assert_eq!(qv.pop(), Some("a".to_owned()));
        assert_eq!(qv.len(), 0);

        assert_eq!(qv.pop(), None);
        assert_eq!(qv.len(), 0);
    }

    #[test]
    fn copy() {
        let mut qv: QVec<_, 3> = QVec::new();
        assert_eq!(qv.capacity(), 3);
        assert_eq!(qv.len(), 0);

        qv.push(1);
        assert_eq!(qv.len(), 1);
        assert_eq!(qv[0], 1);

        qv.push(2);
        assert_eq!(qv.len(), 2);
        assert_eq!(qv[1], 2);

        qv.push(3);
        assert_eq!(qv.len(), 3);
        assert_eq!(qv[2], 3);

        assert_eq!(qv.push_within_capacity(4), Err(4));
        assert_eq!(qv.len(), 3);

        let clone = qv.clone();
        assert_eq!(clone.len(), 3);
        assert_eq!(*clone, [1, 2, 3]);

        let copy = qv;
        assert_eq!(copy.len(), 3);
        assert_eq!(*copy, [1, 2, 3]);

        assert_eq!(qv.pop(), Some(3));
        assert_eq!(qv.len(), 2);

        assert_eq!(qv.pop(), Some(2));
        assert_eq!(qv.len(), 1);

        assert_eq!(qv.pop(), Some(1));
        assert_eq!(qv.len(), 0);

        assert_eq!(qv.pop(), None);
        assert_eq!(qv.len(), 0);
    }

    #[test]
    /// Test that a `QVec`` of `Copy` elements does not call `Clone::clone` on its elements when cloned.
    fn clone_copyable() {
        #[derive(Copy)]
        struct IncOnClone<'a>(&'a Cell<usize>);

        impl<'a> Clone for IncOnClone<'a> {
            fn clone(&self) -> Self {
                self.0.set(self.0.get() + 1);
                *self
            }
        }

        let cell = Cell::new(0);

        let mut qv: QVec<_, 3> = QVec::new();
        qv.push(IncOnClone(&cell));
        qv.push(IncOnClone(&cell));

        let _ = qv.clone();

        assert_eq!(cell.get(), 0);
    }

    #[test]
    /// Test that `QVec` drops its elements when it is dropped.
    fn drop() {
        struct IncOnDrop<'a>(&'a Cell<usize>);

        impl<'a> Drop for IncOnDrop<'a> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let cell = Cell::new(0);

        {
            let mut qv: QVec<_, 3> = QVec::new();
            qv.push(IncOnDrop(&cell));
            qv.push(IncOnDrop(&cell));

            assert_eq!(cell.get(), 0);
        }

        assert_eq!(cell.get(), 2);
    }
}
