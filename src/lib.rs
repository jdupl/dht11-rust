extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    read(14);
}

fn read(pin_num: u64) {
    let pin = Pin::new(pin_num);

    pin.with_exported(|| {
        try!(pin.set_direction(Direction::Out));
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
        Ok(())
    });
}
