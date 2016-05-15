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
    // let pin = Pin::new(pin_num);
    // pin.with_exported(|| {
    //     pin.set_value(0).unwrap();
    //     sleep(Duration::from_millis(30)); // Sleep 30ms
    //     pin.set_value(1).unwrap();
    //     sleep(Duration::new(0, 30)); // Sleep 30us
    // })
    //
    // pin.with_exported(|| {
    //     try!(pin.set_direction(Direction::In));
    //
    //     let mut i = 0;
    //     while i < 100 {
    //         let val = try!(pin.get_value());
    //         println!("Pin State: {}",
    //         if val == 0 {
    //             "Low"
    //         } else {
    //             "High"
    //         });
    //         i += 1;
    //     }
    //     Ok(())
    // }).unwrap();
    let pin = Pin::new(pin_num);
    pin.with_exported(|| {
        pin.set_value(0).unwrap();
        sleep(Duration::from_millis(30)); // Sleep 30ms
        pin.set_value(1).unwrap();
        sleep(Duration::new(0, 30)); // Sleep 30us
        Ok(())
    }).unwrap();

    pin.with_exported(|| {
        try!(pin.set_direction(Direction::In));
        let mut prev_val: u8 = 255;
        loop {
            let val = try!(pin.get_value());
            if val != prev_val {
                println!("Pin State: {}",
                         if val == 0 {
                             "Low"
                         } else {
                             "High"
                         });
                prev_val = val;
            }
            sleep(Duration::from_millis(10));
        }
    }).unwrap();
    Ok(())
}
