use nrf52840::p0;
use nrf52840::gpiote;

use core::mem::transmute;


pub use gestalt_reference_api;
use gestalt_reference_api::generic::GenericEvent;

pub use  crate::non_gestalt::gpio::GpioInst;
use gestalt_reference_api::gpio::GpioInterrupt;
use crate::non_gestalt::gpio::GpioNoneGestalt;
use crate::non_gestalt::gpio;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const EVENT_SIZE: usize = 8;
const INTERRUPT_ADRESS: usize = 0x40006000;
const PSEL_MASK: u32 = 0b0000_0000_0000_0000_0001_1111_0000_0000;

#[derive(Copy, Clone, FromPrimitive)]
pub enum GpioFullEvents
{
    In0 = 0,
    In1 = 1,
    In2 = 2,
    In3 = 3,
    In4 = 4,
    In5 = 5,
    In6 = 6,
    In7 = 7,
    Port = 8
}

#[allow(dead_code)]
pub enum GpioDir
{
    IN  = 0,
    OUT = 1,
}

#[allow(dead_code)]

#[repr(u32)]
pub enum GpioState
{
    UP   =	1,
    DOWN =	0,
}

#[allow(dead_code)]
pub enum GpioPull
{
    UP   = 3,
    DOWN = 1,
    NONE = 0,
}


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum  GpioPin
{
    Gpio0	= 0,
    Gpio1	= 1,
    Gpio2	= 2,
    Gpio3	= 3,
    Gpio4	= 4,
    Gpio5	= 5,
    Gpio6	= 6,
    Gpio7	= 7,
    Gpio8	= 8,
    Gpio9	= 9,
    Gpio10	= 10,
    Gpio11	= 11,
    Gpio12	= 12,
    Gpio13	= 13,
    Gpio14	= 14,
    Gpio15	= 15,
    Gpio16	= 16,
    Gpio17	= 17,
    Gpio18	= 18,
    Gpio19	= 19,
    Gpio20	= 20,
    Gpio21	= 21,
    Gpio22	= 22,
    Gpio23	= 23,
    Gpio24	= 24,
    Gpio25	= 25,
    Gpio26	= 26,
    Gpio27	= 27,
    Gpio28	= 28,
    Gpio29	= 29,
    Gpio30	= 30,
    Gpio31	= 31,
}

pub struct Port
{
    pub base_pin:   *const p0::RegisterBlock,
    pub base_interrupt: *const gpiote::RegisterBlock,
}

pub const fn new	(p: gpio::GpioInst) -> Port
{
    let p = Port{ base_pin: ((p as u32) as  *const p0::RegisterBlock ), base_interrupt: INTERRUPT_ADRESS as *const gpiote::RegisterBlock};
    p
}



impl gestalt_reference_api::gpio::GenericGpio for Port
{
    type Port		= gpio::GpioInst;
    type Pin		= self::GpioPin;
    type Dir		= self::GpioDir;
    type Pull		= self::GpioPull;
    type State		= self::GpioState;
    type PortLength	= usize;

    fn set_state	(&self, pin: GpioPin, state: Self::State)
    {
        let st = state as u32;
        let p = 1 << (pin as u32);
        let r = self.base_pin as u32;
        let s = r as *const p0::RegisterBlock;

        unsafe
            {
                let v = ((p * st)) | ((*s).out.read().bits()  & !(p as u32));
                (*s).out.write(|w|w.bits(v));
            }
    }

    fn set_high		(&self, pin: GpioPin)
    {
        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe{(*s).outset.write(|w|w.bits(1<<(pin as u32)));}
    }

    fn set_low		(&self, pin: GpioPin)
    {
        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe{ (*s).outclr.write(|w|w.bits(1<<(pin as u32))); }

    }

    fn set_direction(&self, pin: GpioPin, dir: Self::Dir)
    {
        self.set_inp_buf(pin,crate::non_gestalt::gpio::GpioBufState::Connect);

        let st = dir as u32;
        let p = 1<<(pin as u32);
        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe
            {
                let v = (p * st) | ((*s).dir.read().bits()  & !(p as u32));
                (*s).dir.write(|w| w.bits(v));
            }
    }

    fn set_pull	(&self, pin: GpioPin, pull: Self::Pull)
    {
        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe
            {
                (*s).pin_cnf[pin as usize].write(|w| w.pull().bits(pull as u8));
            }
    }

    fn set_pull_up(&self, pin: GpioPin)
    {

        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().pullup())}
    }

    fn set_pull_down(&self, pin: GpioPin)
    {

        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().pulldown())}
    }

    fn set_pull_none(&self, pin: GpioPin)
    {

        let r = {self.base_pin as u32};
        let s = r as *const p0::RegisterBlock;

        unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().disabled())}
    }

    fn set_port	(&self){}

    fn toggle (&self, pin: GpioPin)
    {
        let p = 1<<(pin as u32);
        let r = self.base_pin as u32;
        let s = r as *const p0::RegisterBlock;

        unsafe
            {
                let state	= (*s).in_.read().bits();
                let mask = state & p;
                let port = (p ^ mask) | (state & (!p));
                (*s).out.write(| w|{w.bits(port)})
            }

    }

    fn get(&self, pin: Self::Pin) -> Self::State
    {
        let p = 1<<(pin as u32);
        let r = self.base_pin as u32;
        let s = r as *const p0::RegisterBlock;
        let state;
        unsafe{state = (*s).in_.read().bits() & p;}
        let state = state >> (pin as u32);
        let num;
        unsafe {
            num = transmute::<u32, Self::State>(state);
        };
        num
    }

    fn get_port(&self) -> Self::PortLength {
        0
    }
}

impl GpioNoneGestalt for Port
{
    fn set_inp_buf(&self, pin: GpioPin, state: crate::non_gestalt::gpio::GpioBufState)
    {
        let r = self.base_pin as u32;
        let s = r as *const p0::RegisterBlock;
        let state_as_bool;
        unsafe
            {

                if let crate::non_gestalt::gpio::GpioBufState::Connect = state{state_as_bool=false}
                else{state_as_bool=true}

                (*s).pin_cnf[pin as usize].write(|w| w.input().bit(state_as_bool));
//				let adr =(0x50000000 + 0x704) as *mut u32;
//				*adr=0;
            }
    }
}

impl GenericEvent for Port {
    type Event = GpioFullEvents;

    fn is_event_active(&self, status: Self::Event) -> bool {
        unsafe {
            match status {
                GpioFullEvents::In0 => { (*self.base_interrupt).events_in[0].read().bits() == 1 }
                GpioFullEvents::In1 => { (*self.base_interrupt).events_in[1].read().bits() == 1 }
                GpioFullEvents::In2 => { (*self.base_interrupt).events_in[2].read().bits() == 1 }
                GpioFullEvents::In3 => { (*self.base_interrupt).events_in[3].read().bits() == 1 }
                GpioFullEvents::In4 => { (*self.base_interrupt).events_in[4].read().bits() == 1 }
                GpioFullEvents::In5 => { (*self.base_interrupt).events_in[5].read().bits() == 1 }
                GpioFullEvents::In6 => { (*self.base_interrupt).events_in[6].read().bits() == 1 }
                GpioFullEvents::In7 => { (*self.base_interrupt).events_in[7].read().bits() == 1 }
                GpioFullEvents::Port => { false }
            }
        }
    }

    fn flush_event(&self, status: Self::Event)
    {
        unsafe
            {
                match status
                {
                    GpioFullEvents::In0 => { (*self.base_interrupt).events_in[0].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In1 => { (*self.base_interrupt).events_in[1].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In2 => { (*self.base_interrupt).events_in[2].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In3 => { (*self.base_interrupt).events_in[3].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In4 => { (*self.base_interrupt).events_in[4].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In5 => { (*self.base_interrupt).events_in[5].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In6 => { (*self.base_interrupt).events_in[6].write(|w| { w.bits(0) }); }
                    GpioFullEvents::In7 => { (*self.base_interrupt).events_in[7].write(|w| { w.bits(0) }); }
                    GpioFullEvents::Port => { (*self.base_interrupt).events_port.write(|w| { w.bits(0) }); }
                }
            }
    }
}

impl Port
{
    pub fn witch_interrupt_number_active(&self) -> Option<usize> {
        unsafe
            {
                for (i, el) in  (*self.base_interrupt).events_in.iter().enumerate()
                {
                    if el.read().bits() == 1 { return  Some(i) }
                }
            };
        None
    }

    pub fn flush_all_events(&self)
    {
       unsafe {(0..=EVENT_SIZE).map(|i| (*self.base_interrupt).events_in[i].write(|w| { w.bits(0) }));}
    }
}