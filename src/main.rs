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
        try!(pin.set_direction(Direction::Low));
        pin.set_value(0).unwrap();
        let t1 = time::now();
        sleep(Duration::new(0, 20 * 1000000)); // Sleep 30ms

        pin.set_value(1).unwrap();
        let t2 = time::now();
        sleep(Duration::new(0, 1));
        let t3 = time::now();

        try!(pin.set_direction(Direction::In));
        let t4 = time::now();
        let mut last = 1;
        for _ in 1..512 {
            let val = try!(pin.get_value());
            if val != last {
                last = val;
                println!("{}", val);
            }
        }
        println!("{} {} {}", t2 - t1, t3 - t2, t4 - t3);
        Ok(())
    }).unwrap();
    Ok(())
}
