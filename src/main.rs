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
        let mut bits = vec![0];
        try!(pin.set_direction(Direction::Low));
        pin.set_value(0).unwrap();
        sleep(Duration::new(0, 20 * 1000000)); // Sleep 30ms

        pin.set_value(1).unwrap();
        sleep(Duration::new(0, 1));

        try!(pin.set_direction(Direction::In));
        let mut last = 1;
        for _ in 1..512 {
            let val = try!(pin.get_value());
            if val != last {
                last = val;
                bits.push(val);
            }
        }
        println!("{:?}", bits);
        Ok(())
    }).unwrap();
    Ok(())
}
