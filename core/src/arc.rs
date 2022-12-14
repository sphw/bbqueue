use core::sync::atomic::{AtomicUsize, AtomicBool};

use alloc::sync::Arc;

use crate::{BufStorage, Storage, StoragePointer};

impl<'a, const N: usize> Storage for Arc<BufStorage<N>> {
    type Pointer = Self;

    fn get(&self) -> &Self::Pointer {
        self
    }

    fn get_mut(&mut self) -> &mut Self::Pointer {
        self
    }
}

impl<'a, const N: usize> StoragePointer for Arc<BufStorage<N>> {
    const CAPACITY: usize = N;

    fn get_buf_ptr(&self) -> *mut u8 {
        unsafe { (*self.buf.get()).as_mut_ptr() }
    }

    unsafe fn zero_buffer(&self) {
        unsafe { (*self.buf.get()).as_mut_ptr().write_bytes(0, N) };
    }

    #[inline]
    fn write(&self) -> &AtomicUsize {
        &self.write
    }

    #[inline]
    fn read(&self) -> &AtomicUsize {
        &self.read
    }

    #[inline]
    fn last(&self) -> &AtomicUsize {
        &self.last
    }

    #[inline]
    fn reserve(&self) -> &AtomicUsize {
        &self.reserve
    }

    #[inline]
    fn read_in_progress(&self) -> &AtomicBool {
        &self.read_in_progress
    }

    #[inline]
    fn write_in_progress(&self) -> &AtomicBool {
        &self.write_in_progress
    }

    #[inline]
    fn prod_con_count(&self) -> &AtomicUsize {
        &self.prod_con_count
    }

    fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(self, other)
    }
}