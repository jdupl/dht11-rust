extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    match read(14) {
        Ok(()) => println!("ok"),
        Err(err) => println!("Error: {}", err),
    }
}

fn read(pin_num: u64) -> sysfs_gpio::Result<()> {
    let pin = Pin::new(pin_num);

    pin.with_exported(|| {
        let mut prev_val: u8 = 255;
        loop {
            pin.set_value(0).unwrap();
            sleep(Duration::from_millis(30));
            pin.set_value(1).unwrap();
            sleep(Duration::from_millis(20 / 1000));
            try!(pin.set_direction(Direction::In));

            let mut i = 0;
            while i < 100 {
                let val = try!(pin.get_value());
                println!("Pin State: {}",
                if val == 0 {
                    "Low"
                } else {
                    "High"
                });
                i += 1;
            }
            sleep(Duration::from_millis(1000));
        }
    })
}
