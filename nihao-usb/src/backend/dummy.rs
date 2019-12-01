use crate::backend;

pub struct DummyHost {
    vec: Vec<&'static str>,
}

impl<T: Into<Vec<&'static str>>> From<T> for DummyHost {
    fn from(t: T) -> DummyHost {
        DummyHost { vec: t.into() }
    }
}

#[derive(Debug)]
pub struct DummyError;

impl backend::Host for DummyHost {
    type Device = DummyDevice;    
    type List = DummyList;
    type IntoIter = DummyIntoIter;
    type Error = DummyError;

    fn available(&self) -> Result<Self::List, Self::Error> {
        // typically here is a system call
        let vec = self.vec.iter().map(|id| DummyDevice { id }).collect();
        Ok(DummyList { vec })
    }

    type Handle = DummyHandle;

    fn open(&self, _device: Self::Device) -> Result<Self::Handle, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct DummyDevice {
    id: &'static str
}

impl DummyDevice {
    pub fn open(&self) -> Result<DummyHandle, DummyError> {
        println!("Opening {:?}...", self.id);
        Ok(DummyHandle { id: self.id })
    }
}

#[derive(Debug)]
pub struct DummyHandle {
    id: &'static str
}

impl DummyHandle {
    pub fn speed(&self) -> crate::Speed {
        crate::Speed::Low
    } 

    pub fn device_descriptor(&self) -> Result<crate::DeviceDescriptor, DummyError> {
        Ok(crate::DeviceDescriptor {
            length: 66,
            descriptor_type: 66,
            bcd_usb: 66,
            device_class: 66,
            device_sub_class: 66,
            device_protocol: 66,
            max_packet_size_0: 66,
            id_vendor: 66,
            id_product: 66,
            bcd_device: 66,
            manufacturer: 66,
            product: 66,
            serial_number: 66,
            num_configurations: 66
        })
    }
}

#[derive(Debug)]
pub struct DummyList {
    vec: Vec<DummyDevice>,
}

impl<'lt> IntoIterator for DummyList {
    type IntoIter = DummyIntoIter;
    type Item = Result<DummyDevice, DummyError>;
    fn into_iter(self) -> Self::IntoIter {
        DummyIntoIter { inner: self.vec.into_iter() }
    }
}

#[derive(Debug)]
pub struct DummyIntoIter {
    inner: std::vec::IntoIter<DummyDevice>
}

impl Iterator for DummyIntoIter {
    type Item = Result<DummyDevice, DummyError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| Ok(s))
    }
}
