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
        let t1 = time::now();
        try!(pin.set_direction(Direction::Low));
        pin.set_value(1).unwrap();
        sleep(Duration::new(0, 30 * 1000)); // Sleep 30ms

        let t2 = time::now();
        pin.set_value(0).unwrap();
        sleep(Duration::new(0, 30)); // Sleep 30us

        let t3 = time::now();
        try!(pin.set_direction(Direction::In));
        let t4 = time::now();
        for _ in 1..101 {
            let t5 = time::now();
            let val = try!(pin.get_value());
            let t6 = time::now();
            println!("{:?} took {}", val, t6 - t5);
        }
        println!("{} {} {}", t2 - t1, t3 - t2, t4 - t3);
        Ok(())
    }).unwrap();
    Ok(())
}
