use std::mem::MaybeUninit;

pub(crate) const fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    [const { MaybeUninit::uninit() }; N]
}

pub(crate) unsafe fn slice_assume_init_drop<T>(slice: &mut [MaybeUninit<T>]) {
    // SAFETY: the caller must guarantee that `self` is initialized and
    // satisfies all invariants of `T`.
    // Dropping the value in place is safe if that is the case.
    unsafe { (slice as *mut [MaybeUninit<T>] as *mut [T]).drop_in_place() };
}

pub(crate) const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    // SAFETY: casting `slice` to a `*const [T]` is safe since the caller guarantees that
    // `slice` is initialized, and `MaybeUninit` is guaranteed to have the same layout as `T`.
    // The pointer obtained is valid since it refers to memory owned by `slice` which is a
    // reference and thus guaranteed to be valid for reads.
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}

pub(crate) const unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
}
