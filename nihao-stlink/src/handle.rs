use crate::{Handle, Handles, HandleList};
use core::mem;
use std::io;

pub struct IntoIter<'iter> {
    iter: Handles<'iter>,
    list: mem::ManuallyDrop<HandleList<'iter>>,
}

impl<'list> IntoIterator for HandleList<'list> {
    type Item = <IntoIter<'list> as Iterator>::Item;
    type IntoIter = IntoIter<'list>;

    fn into_iter(self) -> IntoIter<'list> {
        let iter = self.iter();
        let list = mem::ManuallyDrop::new(self);
        IntoIter { iter, list }
    }
}

impl<'iter> Drop for IntoIter<'iter> {
    fn drop(&mut self) {
        // println!("Calling drop");
        unsafe { mem::ManuallyDrop::drop(&mut self.list) };
    }
}

impl<'iter> Iterator for IntoIter<'iter> {
    type Item = io::Result<Handle<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
