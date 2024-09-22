use std::ops::{Deref, DerefMut};

use crate::LeakyQVec;

#[derive(Clone, Copy)]
pub(crate) struct CopyQVec<T: Copy, const C: usize>(LeakyQVec<T, C>);

impl<T: Copy, const C: usize> Default for CopyQVec<T, C> {
    fn default() -> Self {
        Self(LeakyQVec::new())
    }
}

impl<T: Copy, const C: usize> Deref for CopyQVec<T, C> {
    type Target = LeakyQVec<T, C>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const C: usize> DerefMut for CopyQVec<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
