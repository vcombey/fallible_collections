//! impl Fallible collections on allocation errors, quite as describe
//! in [RFC 2116](https://github.com/rust-lang/rfcs/blob/master/text/2116-alloc-me-maybe.md)
//! This was used in the turbofish OS hobby project to mitigate the
//! the lack of faillible allocation in rust.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "unstable", feature(try_reserve))]
#![cfg_attr(feature = "unstable", feature(specialization))]
#![cfg_attr(feature = "unstable", feature(allocator_api))]
#![cfg_attr(feature = "unstable", feature(dropck_eyepatch))]
#![cfg_attr(feature = "unstable", feature(ptr_internals))]
#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![cfg_attr(feature = "unstable", feature(maybe_uninit_ref))]
#![cfg_attr(feature = "unstable", feature(maybe_uninit_slice))]
#![cfg_attr(feature = "unstable", feature(maybe_uninit_extra))]
#![cfg_attr(feature = "unstable", feature(internal_uninit_const))]
extern crate alloc;

pub mod boxed;
pub use boxed::*;
#[macro_use]
pub mod vec;
pub use vec::*;
pub mod rc;
pub use rc::*;
pub mod arc;
pub use arc::*;
#[cfg(feature = "unstable")]
pub mod btree;
#[macro_use]
pub mod format;
pub mod try_clone;

#[cfg(feature = "unstable")]
use alloc::collections::TryReserveError;
#[cfg(not(feature = "unstable"))]
use hashbrown::CollectionAllocErr as TryReserveError;

/// trait for trying to clone an elem, return an error instead of
/// panic if allocation failed
/// # Examples
///
/// ```
/// use fallible_collections::TryClone;
/// let mut vec = vec![42, 100];
/// assert_eq!(vec.try_clone().unwrap(), vec)
/// ```
pub trait TryClone {
    /// try clone method, (Self must be size because of Result
    /// constraint)
    fn try_clone(&self) -> Result<Self, TryReserveError>
    where
        Self: core::marker::Sized;
}
