use nrf52840;
use nrf52840::{twi0, twim0};
use gestalt_reference_api::interface::GenericInterface;
use core::marker::PhantomData;
use crate::gestalt::gpio::GpioPin;


pub enum TwiInst
{
	Twi0   = 0x4000_3000,
	Twi1   = 0x4000_4000,
}

pub enum  Freq
{
	///100 kbps
	Freq100k	= 0x01980000,

	///250 kbps
	Freq250k	= 0x04000000,

	///400 kbps
	Freq400k	= 0x06400000,

}

pub struct Twi
{
	pub base: *const twim0::RegisterBlock,
	//phantom: PhantomData<&'a u8>,

}

impl gestalt_reference_api::interface::GenericInterface for Twi
{
	fn write(&mut self){unsafe {(*self.base).tasks_starttx.write(|w|{w.bits(1)})}}
	fn read(&mut self){unsafe {(*self.base).tasks_startrx.write(|w|{w.bits(1)})}}
}

impl gestalt_reference_api::generic::GenericEnable for Twi
{

	fn enable(&self, state: bool) {
		unsafe {(*self.base).enable.write(|w|{w.bits(state as u32 * 6)});}
	}

}

impl gestalt_reference_api::twi::GestaltTwi for Twi
{
	type SdaPin = GpioPin;
	type SclPin = GpioPin;
	type Freq   = Freq;
	type TxBuf  = u8;
	type RxBuf  = u8;

	fn set_sda(&self, pin: Self::SdaPin) {
		unsafe {(*self.base).psel.sda.write(|w|{w.bits(pin as u32)});}
	}

	fn set_scl(&self, pin: Self::SclPin) {
		unsafe {(*self.base).psel.scl.write(|w|{w.bits(pin as u32)});}
	}

	fn set_freq(&self, pin: Self::Freq) {
		unsafe {(*self.base).frequency.write(|w|{w.bits(pin as u32)});}
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

	fn set_addr(&self, addr: usize){unsafe{(*self.base).address.write(|w|w.bits(addr as u32))}}

	fn set_reg_addr(&self, _: usize) {
		unimplemented!()
	}


}

pub fn new(inst: self::TwiInst) -> Twi
{
	Twi { base: ((inst as u32) as *const twim0::RegisterBlock), }
}

