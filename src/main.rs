extern crate serial;

use std::env;
use std::io;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap();
    }
}

fn interact<T: serial::SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(1000)));
    let characters = "0123456789ABCDEF";

    loop {
        for i in characters.chars() {
            for (cnt, j) in characters.chars().enumerate() {
                sleep(Duration::from_millis(100));
                let pref = (1..cnt).map(|_| "000000").collect::<String>();
                let suf = (cnt+2..15).map(|_| "000000").collect::<String>();
                let msg = format!("Y{pref}{i}{j}{i}{j}{i}{j}{suf}Z", pref=pref, suf=suf, i=i, j=j).to_string();
                let buf: Vec<u8> = msg.into_bytes();
                print!("{:?}", buf);

                try!(port.write(&buf[..]));
            }
        }
    }

    Ok(())
}
