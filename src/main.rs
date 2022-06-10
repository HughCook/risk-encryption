#![allow(non_snake_case)]
// Dead code is just making it easier for development
#![allow(dead_code)]

mod os_specific;
mod encryption;
mod ui;
// mod init;

fn main() {
    os_specific::check_os();
}