use std::{
    mem::{forget, MaybeUninit},
    ops::{Deref, DerefMut},
};

use crate::maybe_uninit_ext;

#[derive(Copy)]
pub struct LeakyQVec<T, const C: usize> {
    buf: [MaybeUninit<T>; C],
    len: usize,
}

impl<T, const C: usize> LeakyQVec<T, C> {
    pub const fn new() -> Self {
        Self {
            buf: maybe_uninit_ext::uninit_array(),
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        C
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        *unsafe { self.buf.get_unchecked_mut(self.len) } = MaybeUninit::new(value);
        self.len = unsafe { self.len.unchecked_add(1) };
    }

    pub const fn push(&mut self, value: T) {
        self.buf[self.len] = MaybeUninit::new(value);
        self.len = unsafe { self.len.unchecked_add(1) };
    }

    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        match self.buf.get_mut(self.len) {
            Some(ele) => {
                *ele = MaybeUninit::new(value);
                self.len = unsafe { self.len.unchecked_add(1) };
                Ok(())
            }
            None => Err(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.len.checked_sub(1).map(|len| {
            self.len = len;
            unsafe { self.buf.get_unchecked(len).assume_init_read() }
        })
    }

    pub fn clear(&mut self) {
        self.len = 0;
        unsafe { self.drop() };
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn drop(&mut self) {
        maybe_uninit_ext::slice_assume_init_drop(self.buf.get_unchecked_mut(..self.len));
    }
}

impl<T: Clone, const C: usize> Clone for LeakyQVec<T, C> {
    default fn clone(&self) -> Self {
        let mut new = Self::new();
        let guard = Guard(&mut new);
        for ele in self.iter() {
            unsafe { guard.0.push_unchecked(ele.clone()) };
        }
        forget(guard);
        new
    }
}

impl<T: Copy, const C: usize> Clone for LeakyQVec<T, C> {
    fn clone(&self) -> Self {
        let mut new = Self {
            buf: maybe_uninit_ext::uninit_array(),
            len: self.len,
        };
        unsafe {
            new.buf
                .as_mut_ptr()
                .copy_from_nonoverlapping(self.buf.as_ptr(), self.len)
        };
        new
    }
}

impl<T, const C: usize> Default for LeakyQVec<T, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const C: usize> Deref for LeakyQVec<T, C> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { maybe_uninit_ext::slice_assume_init_ref(self.buf.get_unchecked(..self.len)) }
    }
}

impl<T, const C: usize> DerefMut for LeakyQVec<T, C> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { maybe_uninit_ext::slice_assume_init_mut(self.buf.get_unchecked_mut(..self.len)) }
    }
}

struct Guard<'a, T, const C: usize>(&'a mut LeakyQVec<T, C>);

impl<'a, T, const C: usize> Drop for Guard<'a, T, C> {
    fn drop(&mut self) {
        unsafe { self.0.drop() };
    }
}
