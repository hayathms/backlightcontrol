use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufWriter;


fn main() {

    let max_brightness_filepath = "/sys/class/backlight/intel_backlight/max_brightness";
    let current_brighness_filepath = "/sys/class/backlight/intel_backlight/actual_brightness";
    let set_brightess_filepath = "/sys/class/backlight/intel_backlight/brightness";


    //let max_brightness_filepath = "/home/hayathms/GitWorld/backlightcontrol/displaybackligt/max_brightness";
    //let current_brighness_filepath = "/home/hayathms/GitWorld/backlightcontrol/displaybackligt/brightness";
    //let set_brightess_filepath = "/home/hayathms/GitWorld/backlightcontrol/displaybackligt/brightness";


    let max_brightness: i16 = get_from_file(max_brightness_filepath);
    let current_brighness: i16 = get_from_file(current_brighness_filepath);
    let set_brightess: i16 = get_from_file(set_brightess_filepath);


    println!("Max Brightness {}", max_brightness);
    println!("Current Brightness {}", current_brighness);
    println!("Set Brightness {}", set_brightess);


    let cmd_args: Vec<String> = env::args().collect();
    let operation: &str = cmd_args[1].trim();
    let inc_value: i16 = cmd_args[2].trim().parse::<i16>().unwrap();

    

    let added_value = multiples_of_input(current_brighness+inc_value, inc_value);
    let negated_value = multiples_of_input(current_brighness-inc_value, inc_value);


    match operation.as_ref() {
        "inc" => increment(added_value, set_brightess_filepath),
        "dec" => decrement(negated_value, set_brightess_filepath),
          _   => println!("Wrong option, either give 'inc' or 'dec' full example 'exe inc 10' "),
    }
}


fn multiples_of_input(disp_value: i16, inc_value: i16) -> i16 {

    let fract_value = disp_value/inc_value;
    fract_value*inc_value
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
    if (0 > change_value) || (change_value > 600) {
        panic!("Value out of range" );
    }

    let mut file = File::create(filepath).unwrap();
    let change_value = change_value.to_string();

    file.write(change_value.as_bytes());
}


fn decrement(change_value: i16, filepath: &str) {

    println!("came to increment here {}", change_value);
    if (0 > change_value) || (change_value > 600) {
        panic!("Value out of range" );
    }

    let mut file = File::create(filepath).unwrap();
    let change_value = change_value.to_string();
    
    file.write(change_value.as_bytes());
}

