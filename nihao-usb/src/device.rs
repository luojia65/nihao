use crate::{DeviceList, Device, Devices};
use core::mem;
use std::io;

pub struct IntoIter<'iter> {
    iter: Devices<'iter>,
    list: mem::ManuallyDrop<DeviceList<'iter>>,
}

impl<'list> IntoIterator for DeviceList<'list> {
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
    type Item = io::Result<Device<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
