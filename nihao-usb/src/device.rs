use crate::{DeviceList, Device, Devices};
use core::mem;
use std::io;

pub struct IntoIter {
    iter: Devices,
    list: mem::ManuallyDrop<DeviceList>,
}

impl IntoIterator for DeviceList {
    type Item = <IntoIter as Iterator>::Item;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        let iter = self.iter();
        let list = mem::ManuallyDrop::new(self);
        IntoIter { iter, list }
    }
}

impl<'iter> Drop for IntoIter {
    fn drop(&mut self) {
        // println!("Calling drop");
        unsafe { mem::ManuallyDrop::drop(&mut self.list) };
    }
}

impl<'iter> Iterator for IntoIter {
    type Item = io::Result<Device>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
