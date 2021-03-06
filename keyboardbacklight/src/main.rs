use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufWriter;



fn main() {
    let max_brightness_filepath = "/sys/class/leds/asus::kbd_backlight/max_brightness";
    let current_brighness_filepath = "/sys/class/leds/asus::kbd_backlight/brightness";
    let set_brightess_filepath = "/sys/class/leds/asus::kbd_backlight/brightness";


    let max_brightness: i16 = get_from_file(max_brightness_filepath);
    let current_brighness: i16 = get_from_file(current_brighness_filepath);
    let set_brightess: i16 = get_from_file(set_brightess_filepath);


    println!("Max Brightness {}", max_brightness);
    println!("Current Brightness {}", current_brighness);
    println!("Set Brightness {}", set_brightess);


    let cmd_args: Vec<String> = env::args().collect();
    let operation: &str = cmd_args[1].trim();
    let inc_value: i16 = cmd_args[2].trim().parse::<i16>().unwrap();

    let added_value = current_brighness+inc_value;
    let negated_value = current_brighness-inc_value;


    match operation.as_ref() {
        "inc" => increment(added_value, set_brightess_filepath),
        "dec" => decrement(negated_value, set_brightess_filepath),
          _   => println!("Wrong option, either give 'inc' or 'dec' full example 'exe inc 10' "),
    }
}


fn get_from_file(filepath: &str) -> i16 {

    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let count: i16 = contents.trim().parse::<i16>().unwrap();
    count
}


fn increment(change_value: i16, filepath: &str) {

    println!("came to increment here {}", change_value);
    if (0 > change_value) || (change_value > 3) {
        panic!("Value out of range" );
    }

    let mut file = File::create(filepath).unwrap();
    let change_value = change_value.to_string();

    file.write(change_value.as_bytes());
}



fn decrement(change_value: i16, filepath: &str) {


    println!("came to increment here {}", change_value);
    if (0 > change_value) || (change_value > 3) {
        panic!("Value out of range" );
    }

    let mut file = File::create(filepath).unwrap();
    let change_value = change_value.to_string();
    
    file.write(change_value.as_bytes());
}
