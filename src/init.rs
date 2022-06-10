use std::fs::File;

pub fn init() {
    let mut _file = File::create("all.json").unwrap();

    let _isLaptop = is_laptop::check();
    
    // add output of islaptop to a json file
    
}