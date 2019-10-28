//! Implement Fallible Box
use super::TryClone;
use alloc::boxed::Box;
use alloc::collections::CollectionAllocErr;
use core::alloc::Layout;
use core::borrow::Borrow;
use core::mem::{align_of, size_of};

/// trait to implement Fallible Box
pub trait FallibleBox<T> {
    /// try creating a new box, returning a Result<Box<T>,
    /// CollectionAllocErr> if allocation failed
    fn try_new(t: T) -> Result<Self, CollectionAllocErr>
    where
        Self: Sized;
}

impl<T> FallibleBox<T> for Box<T> {
    fn try_new(t: T) -> Result<Self, CollectionAllocErr> {
        let mut g = alloc::alloc::Global;
        let ptr = unsafe {
            core::alloc::Alloc::alloc(
                &mut g,
                Layout::from_size_align(size_of::<T>(), align_of::<T>()).unwrap(),
            )?
        }
        .as_ptr() as *mut T;
        unsafe {
            core::ptr::write(ptr, t);
            Ok(Box::from_raw(ptr))
        }
    }
}

impl<T: TryClone> TryClone for Box<T> {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        Self::try_new(Borrow::<T>::borrow(self).try_clone()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boxed() {
        let mut v = Box::try_new(5).unwrap();
        assert_eq!(*v, 5);
        *v = 3;
        assert_eq!(*v, 3);
    }
}
