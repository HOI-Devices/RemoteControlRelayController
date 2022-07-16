use gpio::{GpioIn, GpioOut,GpioValue};
use client::Client;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::thread::sleep;

pub struct GpioHandler{
    activation_pin:u16
}

impl GpioHandler{

    pub fn new(input:u16)->Self{
        Self{
            activation_pin:input
        }
    }

    pub fn turn_relay_off(&mut self){
        let mut gpio_pin = gpio::sysfs::SysFsGpioOutput::open(self.activation_pin).unwrap();
        gpio_pin.set_value(true).unwrap();
    }

    pub fn trigger_relay_for_x_milliseconds(&mut self, duration:u64){
        let mut gpio_pin = gpio::sysfs::SysFsGpioOutput::open(self.activation_pin).unwrap();
        gpio_pin.set_value(false).unwrap();
        sleep(Duration::from_millis(duration));
        gpio_pin.set_value(true).unwrap();
    }

}