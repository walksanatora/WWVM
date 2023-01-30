use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use serde_json::Value;

enum Error {
    /// for when the adress is not managed by this module
    OutOfAuthority,
    /// something tried to allocate more memory then is avaliable
    OutOfSpace
}

trait CreateModule {
        /// generic creation function
    /// start is the first adress managed by this module
    /// alloc is a optional ammount of bytes allocated to this module
    /// options are extra options outside of those normally provided
    /// remaing is the ammout of free space after the start (alloc should never be more then remaing, that will cause a error)
    fn create(start: u16, alloc: Option<u16>, options: Option<HashMap<String,Value>>,remaining: u16) -> Result<Self,Error> where Self: Sized + Module;
}

#[enum_dispatch]
trait Module {
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

struct Mux {
    start:u16,
    alloc: u16,
    addr_size: u16,
    sub_modules: Vec<Vec<IDs>>
}

impl Module for Mux {
    fn get_alloc_size(&self) -> u16 {
        todo!()
    }

    fn get_value(&self,addr:u16) -> Result<u16,Error> {
        todo!()
    }

    fn set_value(&mut self,addr:u16,value:u16) -> Result<(),Error> {
        todo!()
    }

    fn is_unstable(&self,addr:u16) -> bool {
        todo!()
    }

    fn is_read(&self,addr:u16) -> bool {
        todo!()
    }

    fn is_write(&self,addr:u16) -> bool {
        todo!()
    }
}

struct Rom {
    start: u16,
    alloc: u16,
    values: Vec<u16>
}

impl Module for Rom {
    fn get_alloc_size(&self) -> u16 {
        todo!()
    }

    fn get_value(&self,addr:u16) -> Result<u16,Error> {
        todo!()
    }

    fn set_value(&mut self,addr:u16,value:u16) -> Result<(),Error> {
        todo!()
    }

    fn is_unstable(&self,addr:u16) -> bool {
        todo!()
    }

    fn is_read(&self,_addr:u16) -> bool {
        true
    }

    fn is_write(&self,_addr:u16) -> bool {
        false
    }
}

#[enum_dispatch(Module)]
enum IDs {
    Mux(Mux),
    Rom(Rom)
}

fn decodeJsonToVM(data: Vec<Value>) -> Vec<IDs> {
    vec![IDs::Rom(Rom {start: 0,alloc: 1,values: vec![2,3,4,5,6,7,8,9,10]})]
}

fn main() {
    println!("Hello, world!");
}
