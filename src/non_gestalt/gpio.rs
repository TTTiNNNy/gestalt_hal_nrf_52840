pub use crate::gestalt::gpio;
use crate::non_gestalt::gpio::gpio::GpioPin;

#[derive(Clone, Copy, PartialEq)]
pub enum GpioBufState
{
    Connect,
    Disconnect
}

pub trait GpioNoneGestalt
{
    fn set_inp_buf(&self, _:gpio::GpioPin, _:GpioBufState);
}

#[derive(Clone, Copy, PartialEq)]
pub enum GpioInst
{
    P0 = 0x5000_0000,
    P1 = 0x5000_0300,

}
