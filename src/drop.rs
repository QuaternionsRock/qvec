use std::ops::{Deref, DerefMut};

use crate::LeakyQVec;

#[derive(Clone)]
pub(crate) struct DropQVec<T, const C: usize>(LeakyQVec<T, C>);

impl<T, const C: usize> Default for DropQVec<T, C> {
    fn default() -> Self {
        Self(LeakyQVec::new())
    }
}

impl<T, const C: usize> Deref for DropQVec<T, C> {
    type Target = LeakyQVec<T, C>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const C: usize> DerefMut for DropQVec<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const C: usize> Drop for DropQVec<T, C> {
    fn drop(&mut self) {
        unsafe { self.0.drop() };
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn cloneable_clone() {
        let mut qv: DropQVec<_, 3> = Default::default();
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
        assert_eq!(**clone, ["a", "b", "c"]);

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
    fn drop() {
        struct IncOnDrop<'a>(&'a Cell<usize>);

        impl<'a> Drop for IncOnDrop<'a> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let cell = Cell::new(0);

        {
            let mut qv: DropQVec<_, 3> = DropQVec::default();
            qv.push(IncOnDrop(&cell));
            qv.push(IncOnDrop(&cell));
        }

        assert_eq!(cell.get(), 2);
    }
}
