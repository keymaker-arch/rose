use lazy_static::lazy_static;
use uart_16550::SerialPort;
use spin::Mutex;

lazy_static! {
    pub static ref SERIAL_PORT0: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        Mutex::new(serial)
    };
}