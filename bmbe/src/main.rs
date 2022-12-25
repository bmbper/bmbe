mod car;

use car::BmbeCarWhell;
use std::{
    borrow::{Borrow, BorrowMut},
    sync::{Arc, RwLock},
    thread::{self, Thread},
    time::Duration,
};

use bmbe_driver::{BmbeDriverErr, BmbeGpioOut, BmbePwm};

fn main() -> Result<(), BmbeDriverErr> {
    let mut car_wheel = BmbeCarWhell::new()?;

    // car_wheel.go_go(100000, 0.5);
    // thread::sleep(Duration::from_secs(2));
    // car_wheel.stop();
    // thread::sleep(Duration::from_secs(3));
    car_wheel.go_test(100000, 0.5);
    thread::sleep(Duration::from_secs(2));
    // car_wheel.stop();
    // car_wheel.go_left(100000, 0.5);
    // thread::sleep(Duration::from_secs(2));
    // car_wheel.stop();
    // car_wheel.go_right(100000, 0.5);
    // thread::sleep(Duration::from_secs(2));
    //car_wheel.stop();
    car_wheel.stop();
    loop {}
}
