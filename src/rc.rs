//! Implement a Fallible Rc
use super::FallibleBox;
use crate::TryReserveError;
#[cfg(not(feature = "unstable"))]
use alloc::boxed::Box;
use alloc::rc::Rc;

/// trait to implement Fallible Rc
#[cfg_attr(
    not(feature = "unstable"),
    deprecated(
        since = "0.4.9",
        note = "⚠️️️this function is not completely fallible, it can panic!, see [issue](https://github.com/vcombey/fallible_collections/issues/13). help wanted"
    )
)]
pub trait FallibleRc<T> {
    /// try creating a new Rc, returning a Result<Box<T>,
    /// TryReserveError> if allocation failed
    fn try_new(t: T) -> Result<Self, TryReserveError>
    where
        Self: Sized;
}

#[allow(deprecated)]
impl<T> FallibleRc<T> for Rc<T> {
    fn try_new(t: T) -> Result<Self, TryReserveError> {
        #[cfg(not(feature = "unstable"))]
        {
            let b = <Box<T> as FallibleBox<T>>::try_new(t)?;
            // bug: from() will reallocate and possibly abort
            Ok(Rc::from(b))
        }
        #[cfg(feature = "unstable")]
        {
            use alloc::alloc::Layout;
            use alloc::collections::TryReserveErrorKind;
            Rc::try_new(t).map_err(|_e| {
                TryReserveErrorKind::AllocError {
                    layout: Layout::new::<Rc<T>>(), // TryReserveErrorKind is not well designed, since it uses a made-up layout, not error from the allocator
                    non_exhaustive: (),
                }
                .into()
            })
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn fallible_rc() {
        use std::rc::Rc;

        let mut x = Rc::new(3);
        *Rc::get_mut(&mut x).unwrap() = 4;
        assert_eq!(*x, 4);

        let _y = Rc::clone(&x);
        assert!(Rc::get_mut(&mut x).is_none());
    }
}
