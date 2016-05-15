extern crate sysfs_gpio;

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
        try!(pin.set_direction(Direction::Low));
        pin.set_value(1).unwrap();
        sleep(Duration::from_millis(30)); // Sleep 30ms
        pin.set_value(0).unwrap();
        sleep(Duration::new(0, 30)); // Sleep 30us
        Ok(())
    }).unwrap();

    pin.with_exported(|| {
        try!(pin.set_direction(Direction::In));
        for i in 1..101 {
            let val = try!(pin.get_value());
            println!("{:?}", val);
            sleep(Duration::new(0, 50));
        }
        Ok(())
    }).unwrap();
    Ok(())
}
