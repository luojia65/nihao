use crate::{Handle, Handles, HandleList};
use core::mem;
use std::io;

pub struct IntoIter<'iter> {
    iter: Handles<'iter>,
    list_ptr: *const HandleList<'iter>,
}

impl<'list> IntoIterator for HandleList<'list> {
    type Item = <IntoIter<'list> as Iterator>::Item;
    type IntoIter = IntoIter<'list>;

    fn into_iter(self) -> IntoIter<'list> {
        let iter = self.iter();
        let list_ptr = &self as *const _;
        mem::forget(self);
        IntoIter { iter, list_ptr }
    }
}

impl<'iter> Drop for IntoIter<'iter> {
    fn drop(&mut self) {
        let list = unsafe { &*self.list_ptr };
        mem::drop(list);
    }
}

impl<'iter> Iterator for IntoIter<'iter> {
    type Item = io::Result<Handle<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
