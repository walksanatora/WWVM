use enum_dispatch::enum_dispatch;
use super::structs::Error;

pub trait ModuleOptions {}

pub trait CreateModule<T>: Module+Sized {
    /// generic creation function
    /// start is the first adress managed by this module
    /// alloc is a optional ammount of bytes allocated to this module
    /// options are extra options outside of those normally provided
    /// remaing is the ammout of free space after the start (alloc should never be more then remaing, that will cause a error)
    fn create(start: u16, alloc: Option<u16>, options: Option<T>,remaining: u16) -> Result<Self,Error> where T: ModuleOptions;
}

#[enum_dispatch]
pub trait Module {
    /// gets the number of size allocated to this module (should hopefully match the value passed into alloc)
    fn get_alloc_size(&self) -> u16;
    /// gets the value of a addr, should return `0` if it is outside of Module's juristiction
    fn get_value(&self, addr: u16) -> Result<u16,Error>;
    /// sets the value of a address
    fn set_value(&mut self, addr: u16, value: u16) -> Result<(),Error>;
    /// whether the output will change without the inputs changing
    fn is_unstable(&self, addr: u16) -> bool;
    /// is the adress readable?
    fn is_read(&self, addr: u16) -> bool;
    /// is the adress writable?
    fn is_write(&self, addr: u16) -> bool;
}