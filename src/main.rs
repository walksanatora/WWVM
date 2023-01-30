use std::collections::HashMap;
use serde_json::Value;

enum Error {
    /// for when the adress is not managed by this module
    OutOfAuthority,
    /// something tried to allocate more memory then is avaliable
    OutOfSpace
}

trait Module {
    /// gets the number of size allocated to this module (should hopefully match the value passed into alloc)
    fn get_alloc_size() -> u16;
    /// gets the value of a addr, should return `0` if it is outside of Module's juristiction
    fn get_value(addr: u16) -> Result<u16,Error>;
    /// sets the value of a address
    fn set_value(addr: u16, value: u16) -> Result<(),Error>;
    /// generic creation function
    /// start is the first adress managed by this module
    /// alloc is a optional ammount of bytes allocated to this module
    /// options are extra options outside of those normally provided
    /// remaing is the ammout of free space after the start (alloc should never be more then remaing, that will cause a error)
    fn create(start: u16, alloc: Option<u16>, options: Option<HashMap<String,Value>>,remaining: u16) -> Result<Self,Error> where Self: Sized;
    /// whether the output will change without the inputs changing
    fn is_unstable(addr: u16) -> bool;
    /// is the adress readable?
    fn is_read(addr: u16) -> bool;
    /// is the adress writable?
    fn is_write(addr: u16) -> bool;
}

struct Mux {
    start:u16,
    alloc: u16,
    addr: u16
}

enum IDs {
    Mux(Mux)
}


fn main() {
    println!("Hello, world!");
}
