use crate::{DeviceList, Device, Devices};
use core::mem;
use std::io;

pub struct IntoIter<'iter> {
    iter: Devices<'iter>,
    list_ptr: *const DeviceList<'iter>,
}

impl<'list> IntoIterator for DeviceList<'list> {
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
    type Item = io::Result<Device<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
