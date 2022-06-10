// Find important modules

// All distros based off these should also work

use std::env;

pub fn check_os() {
    println!("{}", env::consts::OS);
}