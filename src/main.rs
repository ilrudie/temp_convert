fn main() {
    let (temp, unit) = get_and_parse_input();
    if unit == 'f' {
        println!("Temp in c is {}", f_to_c(temp));
    } else if unit == 'c' {
        println!("Temp in f is {}", c_to_f(temp));
    }
}

// (32°F − 32) × 5/9 = 0°C
fn f_to_c(f: f64) -> f64 {
    (f - 32.) * 5. / 9. // this is an expression ending a block so it returns
}

// (0°C x 9/5) + 32 = 32°F
fn c_to_f(c: f64) -> f64 {
    c * 9. / 5. + 32. // this is an expression ending a block so it returns
}

// loop until you are able to get and parse input from stdin
fn get_and_parse_input() -> (f64, char) {
    // loop until we can successfully read the input
    loop {
        // create mutable variable to read the input; initialize it to a new string
        let mut input = String::new();

        // try to read the input from stdin
        match std::io::stdin().read_line(&mut input) {
            Ok(_n) => {},
            Err(_err) => {
                println!("Error reading input");
                continue;
            },
        }

        // trim the trailing whitespace and shadow the result
        let input = input.trim().to_string();

        // get the unit out of the input; clone the string so its reference isn't borrowed
        let unit = match find_unit(input.clone()) {
            Some(c) => c,
            None => {
                println!("Unable to parse a unit from {}", input);
                continue;
            },
        };
        
        // The temp portion of the input will be just numeric characters; remove alphabetic (hopefully the unit)
        let temp = input.trim_end_matches(char::is_alphabetic).to_string();

        // try to parse temp and collect the f64 value parsed if OK or print the error if there is one
        let temp: f64 = match temp.parse() {
            Ok(f) => f,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        // break with shadowed temp, unit from the loop
        // no semi colon so this is expression becomes the return
        break (temp, unit)
    }
}

// will attempt to find the unit character on the provided input
fn find_unit(s: String) -> Option<char> {
    // remove all numberic characters from the beginning of the string and convert to lowercase for easier matching.
    let unit = s.trim().trim_start_matches(char::is_numeric).to_ascii_lowercase();
    
    // all logical branches end with an expression making this if block a guaranteed return
    if unit == "f" {
        // return some f
        std::option::Option::Some('f')
    } else if unit == "c" {
        // return some c
        std::option::Option::Some('c')
    } else {
        // return none
        std::option::Option::None
    }
}