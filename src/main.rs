extern crate sysfs_gpio;
extern crate time;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    match read(14) {
        Ok(()) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }
}

fn read(pin_num: u64) -> sysfs_gpio::Result<()> {
    let pin = Pin::new(pin_num);
    pin.with_exported(|| {
        let mut bits = vec![0; 512];
        try!(pin.set_direction(Direction::Low));
        pin.set_value(0).unwrap();
        sleep(Duration::new(0, 20 * 1000000)); // Sleep 30ms

        pin.set_value(1).unwrap();

        try!(pin.set_direction(Direction::In));
        for i in 1..512 {
            let val = pin.get_value();
            bits[i] = val;
        }
        println!("{:?}", bits);
        Ok(())
    }).unwrap();
    Ok(())
}
