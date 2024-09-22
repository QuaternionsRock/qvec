use std::ops::DerefMut;

use crate::{copy::CopyQVec, drop::DropQVec, LeakyQVec};

pub(crate) trait CloneQVecType {
    type CloneQVecType: Clone + Default + DerefMut<Target = Self>;
}

impl<T: Clone, const C: usize> CloneQVecType for LeakyQVec<T, C> {
    default type CloneQVecType = DropQVec<T, C>;
}

impl<T: Copy, const C: usize> CloneQVecType for LeakyQVec<T, C> {
    type CloneQVecType = CopyQVec<T, C>;
}

pub(crate) trait QVecType {
    type QVecType: Default + DerefMut<Target = Self>;
}

impl<T, const C: usize> QVecType for LeakyQVec<T, C> {
    default type QVecType = DropQVec<T, C>;
}

impl<T: Clone, const C: usize> QVecType for LeakyQVec<T, C> {
    type QVecType = <Self as CloneQVecType>::CloneQVecType;
}
