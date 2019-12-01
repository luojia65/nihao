use crate::backend;

pub type Error = <Host as backend::Host>::Error;
pub type Result<T> = core::result::Result<T, Error>;
pub type List = <Host as backend::Host>::List;
pub type IntoIter = <Host as backend::Host>::IntoIter;
pub type Device = <Host as backend::Host>::Device;
pub type Handle = <Host as backend::Host>::Handle;

#[cfg(any(windows, doc))]
pub mod windows;

#[cfg(any(windows))]
pub type Host = backend::dummy::DummyHost;

#[cfg(any(windows))]
pub(crate) fn new_default_host() -> Host {
    vec!["Test", "Test2", "Test3"].into()
}

/// Get an `Iterator` over all USB devices identified by your operating system.
/// 
/// Note that the return value for this iterator is a `Result`.
/// You may need to use a try operator `?` after the function call `devices()`
/// if you want to iterate everything in it by using `for` statements. 
/// That's because a `Result` is also an `Iterator`, and its `Item` is `Devices`
/// other than `Device` expected.
pub fn devices() -> Result<<Host as backend::Host>::List> {
    use backend::Host;
    new_default_host().available()
}
