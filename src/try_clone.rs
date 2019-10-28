//! this module implements try clone for primitive rust types

use super::TryClone;
use alloc::collections::CollectionAllocErr;

macro_rules! impl_try_clone {
    ($($e: ty),*) => {
        $(impl TryClone for $e {
            #[inline(always)]
            fn try_clone(&self) -> Result<Self, CollectionAllocErr>
            where
                Self: core::marker::Sized,
            {
                Ok(*self)
            }
        }
        )*
    }
}

impl_try_clone!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, bool, [i8; 256]);

impl<T: TryClone> TryClone for Option<T> {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        Ok(match self {
            Some(t) => Some(t.try_clone()?),
            None => None,
        })
    }
}
// impl<T: Copy> TryClone for T {
//     fn try_clone(&self) -> Result<Self, CollectionAllocErr>
//     where
//         Self: core::marker::Sized,
//     {
//         Ok(*self)
//     }
// }
