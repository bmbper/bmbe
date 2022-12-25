use sysfs_gpio::{Pin, Direction, Error};
use crate::{BmbeDriverErr, BmbeDriverErrKind};

pub enum BmbeGpioDirection {
    OutPut,
    InPut
}

pub struct BmbeGpio{
    pin: Pin,
    direction:Direction,
    pin_num:u64,
    value: u8
}

impl BmbeGpio {
    
    pub fn new_out(pin_num:u64)->Result<Self,BmbeDriverErr>{
        let mut pin  = Pin::new(pin_num);
        if !pin.is_exported(){
            pin.export()?;
        }
        pin.set_direction(Direction::Out)?;
        pin.set_value(0);
        Ok(BmbeGpio { pin, direction: Direction::Out, pin_num ,value:0u8})
    }
    
    pub fn new_in(pin_num:u64)->Result<Self,BmbeDriverErr>{
        let mut pin  = Pin::new(pin_num);
        if !pin.is_exported(){
            pin.export()?;
        }
        pin.set_direction(Direction::In)?;
        pin.set_value(0);
        Ok(BmbeGpio { pin, direction: Direction::Out, pin_num,value:0u8 })
    }

    pub fn set_high(&mut self){
        self.value = 1;
        self.pin.set_value(1);
    }

    pub fn set_low(&mut self){
        self.value = 0;
        self.pin.set_value(0);
    }

    pub fn get_value(&self)->u8{
        self.value.clone()
    }

    pub fn get_in_value(&self)->Result<u8,BmbeDriverErr>{
        let v = self.pin.get_value()?;
        Ok(v)
    }
}


pub struct BmbeGpioOut{
    pin: Pin,
    direction:Direction,
    pin_num:u64,
    value:u8
}

impl BmbeGpioOut {

    pub fn new(pin_num:u64)->Result<Self,BmbeDriverErr>{
        let mut pin  = Pin::new(pin_num);
        if !pin.is_exported(){
            pin.export()?;
        }
        pin.set_direction(Direction::Out)?;
        pin.set_value(0);
        Ok(BmbeGpioOut { pin, direction: Direction::Out, pin_num ,value:0u8})
    }

    pub fn set_high(&mut self){
        self.value = 1;
        self.pin.set_value(1);
    }

    pub fn set_low(&mut self){
        self.value = 0;
        self.pin.set_value(0);
    }

    pub fn get_value(&self)->u8{
        self.value.clone()
    }
}

pub struct BmbeGpioIn{
    pin: Pin,
    direction:Direction,
    pin_num:u64
}


impl BmbeGpioIn {
      
    pub fn new(pin_num:u64)->Result<Self,BmbeDriverErr>{
        let mut pin  = Pin::new(pin_num);
        if !pin.is_exported(){
            pin.export()?;
        }
        pin.set_direction(Direction::In)?;
        pin.set_value(0);
        Ok(BmbeGpioIn  { pin, direction: Direction::Out, pin_num})
    }

    pub fn get_in_value(&self)->Result<u8,BmbeDriverErr>{
        let v = self.pin.get_value()?;
        Ok(v)
    }
   
}


impl From<Error> for BmbeDriverErr {
    fn from(e: Error) -> Self {
        BmbeDriverErr::new(e.to_string(),BmbeDriverErrKind::Pin)
    }
}

