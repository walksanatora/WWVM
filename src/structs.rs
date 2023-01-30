use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use super::traits::*;

#[derive(Serialize,Deserialize)]
pub struct MuxOptions{
    addr_size: Option<u16>
}

impl ModuleOptions for MuxOptions {}

pub enum Error {
    /// for when the adress is not managed by this module
    OutOfScope,
    /// something tried to allocate more memory then is avaliable
    OutOfSpace,
    /// Generic error, Aka: too specific error
    Generic(String)
}

#[enum_dispatch(Module)]
pub enum IDs {
    Mux(Mux),
    Rom(Rom)
}


/// a MultiPlexer module
/// this module can hold mutiple other lowers
/// defaults:
///     alloc: 32
///     addr_size: 1
pub struct Mux {
    start:u16,
    alloc: u16,
    addr_size: u16,
    sub_modules: Vec<Vec<IDs>>
}

impl CreateModule<MuxOptions> for Mux {
    fn create(start: u16, alloc: Option<u16>, options: Option<MuxOptions>,remaining: u16) -> Result<Self,Error> {
        let mut sub_modules:Vec<Vec<IDs>> = vec![];
        let addr_size = if let Some(values) = options {
            if let Some(value) = values.addr_size {value}
            else {1}
        } else {1};
        

        let alloc = alloc.unwrap_or(32);
        if alloc > remaining {
            return Err(
                Error::OutOfSpace
            )
        }

        if addr_size > alloc {
            return Err(
                Error::Generic("addr size greater then allocated size".into())
            )
        }

        

        Ok(Mux {
            start,
            alloc,
            addr_size,
            sub_modules
        })
    }
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

pub struct Rom {
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