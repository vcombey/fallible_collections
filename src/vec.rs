//! Implement Fallible Vec
use super::TryClone;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;

#[macro_export]
/// macro trying to create a vec, return a
/// Result<Vec<T>,CollectionAllocErr>
macro_rules! try_vec {
   ($elem:expr; $n:expr) => (
        $crate::vec::try_from_elem($elem, $n)
    );
    ($($x:expr),*) => (
        match <alloc::boxed::Box<_> as $crate::boxed::FallibleBox<_>>::try_new([$($x),*]) {
            Err(e) => Err(e),
            Ok(b) => Ok(<[_]>::into_vec(b)),
        }
    );
    ($($x:expr,)*) => ($crate::try_vec![$($x),*])
}

/// trait implementing all fallible methods on vec
pub trait FallibleVec<T> {
    /// see reserve
    fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr>;
    /// see push
    fn try_push(&mut self, elem: T) -> Result<(), CollectionAllocErr>;
    /// try push and give back ownership in case of error
    fn try_push_give_back(&mut self, elem: T) -> Result<(), (T, CollectionAllocErr)>;
    /// see with capacity, (Self must be sized by the constraint of Result)
    fn try_with_capacity(capacity: usize) -> Result<Self, CollectionAllocErr>
    where
        Self: core::marker::Sized;
    /// see insert
    fn try_insert(&mut self, index: usize, element: T) -> Result<(), (T, CollectionAllocErr)>;
    /// see append
    fn try_append(&mut self, other: &mut Self) -> Result<(), CollectionAllocErr>;
    /// see resize, only works when the `value` implements Copy, otherwise, look at try_resize_no_clone
    fn try_resize(&mut self, new_len: usize, value: T) -> Result<(), CollectionAllocErr>
    where
        T: Copy + Clone;
    /// resize the vec by trying to clone the value repeatingly
    fn try_resize_no_copy(&mut self, new_len: usize, value: T) -> Result<(), CollectionAllocErr>
    where
        T: TryClone;
    /// see resize, only works when the `value` implements Copy, otherwise, look at try_extend_from_slice_no_copy
    fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), CollectionAllocErr>
    where
        T: Copy + Clone;
    /// extend the vec by trying to clone the value in `other`
    fn try_extend_from_slice_no_copy(&mut self, other: &[T]) -> Result<(), CollectionAllocErr>
    where
        T: TryClone;
}

impl<T> FallibleVec<T> for Vec<T> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
        self.try_reserve(additional)
    }
    fn try_push(&mut self, elem: T) -> Result<(), CollectionAllocErr> {
        self.try_reserve(1)?;
        Ok(self.push(elem))
    }
    fn try_push_give_back(&mut self, elem: T) -> Result<(), (T, CollectionAllocErr)> {
        if let Err(e) = self.try_reserve(1) {
            return Err((elem, e));
        }
        Ok(self.push(elem))
    }
    fn try_with_capacity(capacity: usize) -> Result<Self, CollectionAllocErr>
    where
        Self: core::marker::Sized,
    {
        let mut n = Self::new();
        n.try_reserve(capacity)?;
        Ok(n)
    }

    fn try_insert(&mut self, index: usize, element: T) -> Result<(), (T, CollectionAllocErr)> {
        if let Err(e) = self.try_reserve(1) {
            return Err((element, e));
        }
        Ok(self.insert(index, element))
    }
    fn try_append(&mut self, other: &mut Self) -> Result<(), CollectionAllocErr> {
        self.try_reserve(other.len())?;
        Ok(self.append(other))
    }
    fn try_resize(&mut self, new_len: usize, value: T) -> Result<(), CollectionAllocErr>
    where
        T: Copy + Clone,
    {
        let len = self.len();
        if new_len > len {
            self.try_reserve(new_len - len)?;
        }
        Ok(self.resize(new_len, value))
    }
    fn try_resize_no_copy(&mut self, new_len: usize, value: T) -> Result<(), CollectionAllocErr>
    where
        T: TryClone,
    {
        let len = self.len();

        if new_len > len {
            self.try_extend_with(new_len - len, TryExtendElement(value))
        } else {
            Ok(self.truncate(new_len))
        }
    }
    fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), CollectionAllocErr>
    where
        T: Copy + Clone,
    {
        self.try_reserve(other.len())?;
        Ok(self.extend_from_slice(other))
    }
    fn try_extend_from_slice_no_copy(&mut self, other: &[T]) -> Result<(), CollectionAllocErr>
    where
        T: TryClone,
    {
        let mut len = self.len();
        self.try_reserve(other.len())?;
        let mut iterator = other.iter();
        while let Some(element) = iterator.next() {
            unsafe {
                core::ptr::write(self.get_unchecked_mut(len), element.try_clone()?);
                // NB can't overflow since we would have had to alloc the address space
                len += 1;
                self.set_len(len);
            }
        }
        Ok(())
    }
}

trait ExtendWith<T> {
    fn next(&mut self) -> Result<T, CollectionAllocErr>;
    fn last(self) -> T;
}

struct TryExtendElement<T: TryClone>(T);
impl<T: TryClone> ExtendWith<T> for TryExtendElement<T> {
    fn next(&mut self) -> Result<T, CollectionAllocErr> {
        self.0.try_clone()
    }
    fn last(self) -> T {
        self.0
    }
}

trait TryExtend<T> {
    fn try_extend_with<E: ExtendWith<T>>(
        &mut self,
        n: usize,
        value: E,
    ) -> Result<(), CollectionAllocErr>;
}

impl<T> TryExtend<T> for Vec<T> {
    /// Extend the vector by `n` values, using the given generator.
    fn try_extend_with<E: ExtendWith<T>>(
        &mut self,
        n: usize,
        mut value: E,
    ) -> Result<(), CollectionAllocErr> {
        self.try_reserve(n)?;

        unsafe {
            let mut ptr = self.as_mut_ptr().add(self.len());

            let mut local_len = self.len();
            // Write all elements except the last one
            for _ in 1..n {
                core::ptr::write(ptr, value.next()?);
                ptr = ptr.offset(1);
                // Increment the length in every step in case next() panics
                local_len += 1;
                self.set_len(local_len);
            }

            if n > 0 {
                // We can write the last element directly without cloning needlessly
                core::ptr::write(ptr, value.last());
                local_len += 1;
                self.set_len(local_len);
            }

            // len set by scope guard
        }
        Ok(())
    }
}

trait Truncate {
    fn truncate(&mut self, len: usize);
}

impl<T> Truncate for Vec<T> {
    fn truncate(&mut self, len: usize) {
        let current_len = self.len();
        unsafe {
            let mut ptr = self.as_mut_ptr().add(current_len);
            // Set the final length at the end, keeping in mind that
            // dropping an element might panic. Works around a missed
            // optimization, as seen in the following issue:
            // https://github.com/rust-lang/rust/issues/51802
            let mut local_len = self.len();

            // drop any extra elements
            for _ in len..current_len {
                ptr = ptr.offset(-1);
                core::ptr::drop_in_place(ptr);
                local_len -= 1;
                self.set_len(local_len);
            }
        }
    }
}

/// try creating a vec from an `elem` cloned `n` times, see std::from_elem
pub fn try_from_elem<T: TryClone>(elem: T, n: usize) -> Result<Vec<T>, CollectionAllocErr> {
    <T as SpecFromElem>::try_from_elem(elem, n)
}

// Specialization trait used for Vec::from_elem
trait SpecFromElem: Sized {
    fn try_from_elem(elem: Self, n: usize) -> Result<Vec<Self>, CollectionAllocErr>;
}

impl<T: TryClone> SpecFromElem for T {
    default fn try_from_elem(elem: Self, n: usize) -> Result<Vec<T>, CollectionAllocErr> {
        let mut v = Vec::new();
        v.try_resize_no_copy(n, elem)?;
        Ok(v)
    }
}

impl SpecFromElem for u8 {
    #[inline]
    fn try_from_elem(elem: u8, n: usize) -> Result<Vec<u8>, CollectionAllocErr> {
        unsafe {
            let mut v = Vec::try_with_capacity(n)?;
            core::ptr::write_bytes(v.as_mut_ptr(), elem, n);
            v.set_len(n);
            Ok(v)
        }
    }
}

impl<T: TryClone> TryClone for Vec<T> {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr>
    where
        Self: core::marker::Sized,
    {
        let mut v = Vec::new();
        v.try_extend_from_slice_no_copy(self)?;
        Ok(v)
    }
}

pub trait TryFromIterator<I>: Sized {
    fn try_from_iterator<T: IntoIterator<Item = I>>(
        iterator: T,
    ) -> Result<Self, CollectionAllocErr>;
}

impl<I> TryFromIterator<I> for Vec<I> {
    fn try_from_iterator<T: IntoIterator<Item = I>>(iterator: T) -> Result<Self, CollectionAllocErr>
    where
        T: IntoIterator<Item = I>,
    {
        let mut new = Self::new();
        for i in iterator {
            new.try_push(i)?;
        }
        Ok(new)
    }
}

pub trait TryCollect<I> {
    fn try_collect<C: TryFromIterator<I>>(self) -> Result<C, CollectionAllocErr>;
}

impl<I, T> TryCollect<I> for T
where
    T: IntoIterator<Item = I>,
{
    fn try_collect<C: TryFromIterator<I>>(self) -> Result<C, CollectionAllocErr> {
        C::try_from_iterator(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec() {
        // let v: Vec<u8> = from_elem(1, 10);
        let v: Vec<Vec<u8>> = try_vec![try_vec![42; 10].unwrap(); 100].unwrap();
        println!("{:?}", v);
        let v2 = try_vec![0, 1, 2];
        println!("{:?}", v2);
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn try_clone_vec() {
        // let v: Vec<u8> = from_elem(1, 10);
        let v = vec![42; 100];
        assert_eq!(v.try_clone().unwrap(), v);
    }

    // #[test]
    // fn try_out_of_mem() {
    //     let v = try_vec![42_u8; 1000000000];
    //     assert_eq!(v.try_clone().unwrap(), v);
    // }
}
