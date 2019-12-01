// use crate::backend;

// use crate::sys::windows;

// pub struct Host;

// impl backend::Host for Host {
    
//     type Device: 'lt;    

//     type List: iter::IntoIterator<
//         Item = Result<Self::Device, Self::Error>,
//         IntoIter = Self::IntoIter,
//     >;

//     type Iter: iter::Iterator<
//         Item = Result<&'lt Self::Device, Self::Error>,
//     >;

//     type IntoIter: iter::Iterator<
//         Item = Result<Self::Device, Self::Error>,
//     >;
    
//     type Error;

//     fn available(&self) -> Result<Self::List, Self::Error> {

//     }

//     type Handle;

//     fn open(&self, device: Self::Device) -> Result<Self::Handle, Self::Error> {

//     }
// }
