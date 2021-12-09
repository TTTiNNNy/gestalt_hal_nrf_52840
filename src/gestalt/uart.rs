use nrf52840;

use nrf52840::uarte0;

use gestalt_reference_api;

use gestalt_reference_api::interface::GenericInterface;
use gestalt_reference_api::uart::{FullGenericUartStatus, GenericUartStatus};

use crate::gestalt::gpio::GpioPin;

use gestalt_reference_api::generic::{GenericEvent, GenericStatus};
use gestalt_reference_api::generic::GenericEnable;


#[allow(dead_code)]
#[derive(Debug)]
pub enum UartInst
{
    Uart0		= 0x4000_2000,
    Uart1		= 0x4002_8000,
}

#[derive(Copy, Clone)]
pub enum UartFullEvents
{
    Cts,
    Ncts,
    Rxdy, //Data received in RXD (but potentially not yet transferred to Data RAM)
    Endrx,
    Txdy,
    Endtx,
    Err,
    Rxto,
    RxStarted,
    TxStarted,
    TxStopped
}

pub enum  Baudrate
{
    ///1200 baud (actual rate: 1205)
    Baud1200	= 0x0004F000,

    ///2400 baud (actual rate: 2396)
    Baud2400	= 0x0009D000,
    ///4800 baud (actual rate: 4808)
    Baud4800	= 0x0013B000,
    ///9600 baud (actual rate: 9598)
    Baud9600	= 0x00275000,
    ///14400 baud (actual rate: 14401)
    Baud14400	= 0x003AF000,
    ///19200 baud (actual rate: 19208)
    Baud19200	= 0x004EA000,
    ///28800 baud (actual rate: 28777)
    Baud28800	= 0x0075C000,
    ///31250 baud
    Baud31250	= 0x00800000,
    ///38400 baud (actual rate: 38369)
    Baud38400	= 0x009D0000,
    ///56000 baud (actual rate: 55944)
    Baud56000	= 0x00E50000,
    ///57600 baud (actual rate: 57554)
    Baud57600	= 0x00EB0000,
    ///76800 baud (actual rate: 76923)
    Baud76800	= 0x013A9000,
    ///115200 baud (actual rate: 115108)
    Baud115200	= 0x01D60000,
    ///230400 baud (actual rate: 231884)
    Baud230400	= 0x03B00000,
    ///250000 baud
    Baud250000	= 0x04000000,
    ///460800 baud (actual rate: 457143)
    Baud460800	= 0x07400000,
    ///921600 baud (actual rate: 941176)
    Baud921600	= 0x0F000000,
    ///1 Mega baud
    Baud1M		= 0x10000000,
}

pub struct Uart
{
    pub base: *const uarte0::RegisterBlock,
}

impl gestalt_reference_api::interface::GenericInterface for Uart
{
    fn write(&mut self){unsafe {(*self.base).tasks_starttx.write(|w|{w.bits(1)})}}
    fn read(&mut self){unsafe {(*self.base).tasks_startrx.write(|w|{w.bits(1)})}}
}

impl GenericEnable for Uart
{
    fn enable(&self, state: bool)
    {
        unsafe {(*self.base).enable.write(|w|{w.bits(state as u32*8)});}
    }
}

pub const fn new (inst: UartInst) -> Uart
{
    let addr = inst as u32;
    let mut p = addr as  *const uarte0::RegisterBlock;

    Uart{base : p}
}

impl gestalt_reference_api::uart::GenericUart for Uart
{
    type TxPin	= GpioPin;
    type RxPin	= GpioPin;
    type Baud	= Baudrate;
    type TxBuf	= usize;
    type RxBuf	= usize;

    fn set_rx       (&self, pin: Self::RxPin)
    {
        unsafe {(*self.base).psel.rxd.write(|w|{w.bits(pin as u32)});}
    }

    fn set_tx       (&self, pin: Self::TxPin)
    {
        unsafe {(*self.base).psel.txd.write(|w|{w.bits(pin as u32)});}
    }

    fn set_baud     (&self, baud: Self::Baud)
    {
        unsafe {(*self.base).baudrate.write(|w|{w.bits(baud as u32)})};
    }

    fn set_tx_buf   (&self, arr: &[Self::TxBuf])
    {
        unsafe
        {
            (*self.base).txd.maxcnt.write	(|w| {w.bits(arr.len() as u32)});
            (*self.base).txd.ptr.write		(|w| {w.bits(arr.as_ptr() as u32)});
        }
    }

    fn set_rx_buf   (&self, arr: &[Self::RxBuf])
    {
        unsafe
        {
            (*self.base).rxd.maxcnt.write	(|w| {w.bits(arr.len() as u32)});
            (*self.base).rxd.ptr.write		(|w| {w.bits(arr.as_ptr() as u32)});
        }
    }

}


impl Uart
{
    pub fn is_writed(&self) -> bool { unsafe {(*self.base).events_endrx.read().bits() == 1} }
    pub fn is_readed(&self) -> bool { unsafe {(*self.base).events_endtx.read().bits() == 1} }
    pub fn flush_write_event(&self)
    {        unsafe {(*self.base).events_endtx.write(|w| {w.bits(0)}); }}
    pub fn flush_read_event(&self)  { unsafe {(*self.base).events_endrx.write(|w| {w.bits(0)}); }}

}

impl GenericEvent for Uart {
    type Event = UartFullEvents;

    fn is_event_active(&self, status: Self::Event) -> bool
    {
        unsafe
            {
                    match status
                    {
                        UartFullEvents::Cts =>       {(*self.base).events_cts.read().bits() == 1}
                        UartFullEvents::Ncts =>      {(*self.base).events_ncts.read().bits() == 1}
                        UartFullEvents::Rxdy =>      {(*self.base).events_rxdrdy.read().bits() == 1}
                        UartFullEvents::Txdy =>      {(*self.base).events_txdrdy.read().bits() == 1}
                        UartFullEvents::Err =>       {(*self.base).events_error.read().bits() == 1}
                        UartFullEvents::Rxto =>      {(*self.base).events_rxto.read().bits() == 1}
                        UartFullEvents::RxStarted => {(*self.base).events_rxstarted.read().bits() == 1}
                        UartFullEvents::TxStarted => {(*self.base).events_txstarted.read().bits() == 1}
                        UartFullEvents::TxStopped => {(*self.base).events_txstopped.read().bits() == 1}
                        UartFullEvents::Endrx =>     {(*self.base).events_endrx.read().bits() == 1}
                        UartFullEvents::Endtx =>     {(*self.base).events_endtx.read().bits() == 1}
                    }
            }
    }


    fn flush_event(&self, status: Self::Event)
    {
        unsafe
            {
                match status
                {
                    UartFullEvents::Cts =>      { (*self.base).events_cts.write(|w| { w.bits(0) }); }
                    UartFullEvents::Ncts =>     { (*self.base).events_ncts.write(|w| { w.bits(0) }); }
                    UartFullEvents::Rxdy =>     { (*self.base).events_rxdrdy.write(|w| { w.bits(0) }); }
                    UartFullEvents::Endrx =>    { (*self.base).events_endrx.write(|w| { w.bits(0) }); }
                    UartFullEvents::Txdy =>     { (*self.base).events_txdrdy.write(|w| { w.bits(0) }); }
                    UartFullEvents::Endtx =>    { (*self.base).events_endtx.write(|w| { w.bits(0) }); }
                    UartFullEvents::Err =>      { (*self.base).events_error.write(|w| { w.bits(0) }); }
                    UartFullEvents::Rxto =>     { (*self.base).events_rxto.write(|w| { w.bits(0) }); }
                    UartFullEvents::RxStarted =>{ (*self.base).events_rxstarted.write(|w| { w.bits(0) }); }
                    UartFullEvents::TxStarted =>{ (*self.base).events_txstarted.write(|w| { w.bits(0) }); }
                    UartFullEvents::TxStopped =>{ (*self.base).events_txstopped.write(|w| { w.bits(0) }); }
                }
            }
    }

}

