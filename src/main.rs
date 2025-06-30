use std::io;

fn main() {
    loop {
        println!("Welcome to the control-flow demonstration!");
        println!("If you would like to convert Temperature, type 'T' without quotes.");
        println!("If you want to generate a Fibonacci number, type 'F' without quotes.");
        println!(
            "If you want to print the lyrics of 'The Twelve Days of Christmas', type 'C' without quotes."
        );
        println!("If you want to quit, type 'Q' without quotes.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failure to read line.");

        let user_input: char = match user_input.trim().parse() {
            Ok(chr) => chr,
            Err(_) => continue,
        };

        println!(
            "{} {}",
            user_input.to_ascii_lowercase(),
            user_input.to_ascii_lowercase() == 'q'
        );
        match user_input.to_ascii_lowercase() {
            't' => temperature_converter(),
            'f' => fibonnaci_generator(),
            'c' => music_lyrics(),
            'q' => break,
            _ => continue,
        }
    }
}

fn music_lyrics() {
    let days = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth", "Eleventh", "Twelfth",
    ];

    println!("The Twelve Days of Christmas");

    for day in 1..=12 {
        println!(
            "On the {} day of Christmas my True Love gave to Me:",
            days[day - 1]
        );
        for verse in (1..=day).rev() {
            match verse {
                1 => println!(
                    "{} partrdge in a pear tree!",
                    if day > 1 { "and a" } else { "A" }
                ),
                2 => println!("Two turtle doves,"),
                3 => println!("Three French hens,"),
                4 => println!("Four calling birds,"),
                5 => println!("Five Golden Riiiiiings!"),
                6 => println!("Six geese a-laying,"),
                7 => println!("Seven swans a-swimming,"),
                8 => println!("Eight maids a-milking,"),
                9 => println!("Nine ladies dancing,"),
                10 => println!("Ten lords a-leaping,"),
                11 => println!("Eleven pipers piping,"),
                12 => println!("Twelve drummers drumming,"),
                _ => continue,
            }
        }
    }
}

fn fibonnaci_generator() {
    fn iterative_fibb(n: u128) -> u128 {
        let mut a = 0;
        let mut b = 1;
        let mut c;
        if n == 0 {
            return a;
        }
        for _m in 2..=n {
            c = a + b;
            a = b;
            b = c;
        }
        return b;
    }

    fn recursive_fibb(n: u128) -> u128 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }
        recursive_fibb(n - 1) + recursive_fibb(n - 2)
    }

    loop {
        println!(
            "Enter the number of iterations of the Fibonnaci sequence you'd like to calculate."
        );
        println!("Or enter 'Q' to quit.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let temp_input = user_input.clone();

        let user_input: u128 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if user_input < 1 {
            let temp_input: char = match temp_input.trim().parse() {
                Ok(chr) => chr,
                Err(_) => continue,
            };

            if temp_input.to_ascii_lowercase() == 'q' {
                break;
            }
        }

        println!(
            "The Fibonnaci sequence up to n={user_input} is {}",
            iterative_fibb(user_input)
        );
    }
}

fn temperature_converter() {
    fn fahrenheit_converter() {
        loop {
            println!("Enter the degrees in Celsius to have the value calculated in Fahrenheit!");
            println!("Alternatively, enter 'Q' to quit.");

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            let temp_input = user_input.trim();

            let user_input: f64 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => -999.0,
            };

            if user_input < -460.0 {
                let temp_input: char = match temp_input.trim().parse() {
                    Ok(chr) => chr,
                    Err(_) => continue,
                };

                if temp_input.to_ascii_lowercase() == 'q' {
                    break;
                }
            }

            let result = (user_input * 1.8) + 32.0;
            println!("{temp_input}°C is {result}");
        }
    }

    fn celsius_converter() {
        loop {
            println!("Enter the degrees in Fahrenheit to have the value calculated in Celsius!");
            println!("Alternatively, enter 'Q' to quit.");

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            let temp_input = user_input.clone();

            let user_input: f64 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => -999.0,
            };

            if user_input < -274.0 {
                let temp_input: char = match temp_input.trim().parse() {
                    Ok(chr) => chr,
                    Err(_) => continue,
                };

                println!("{}", temp_input.to_ascii_lowercase());
                if temp_input.to_ascii_lowercase() == 'q' {
                    break;
                }
            }

            let result = (user_input - 32.0) / 1.8;
            println!("{temp_input}°F is {result}");
        }
    }

    loop {
        println!("Welcome to the temperature converter!");
        println!("Please enter whether you want to convert to Fahrenheit 'F' or Celsius 'C'");
        println!("Otherwise, enter 'Q' to quit.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failure to read line.");

        let user_input: char = match user_input.trim().parse() {
            Ok(chr) => chr,
            Err(_) => continue,
        };

        match user_input.to_ascii_lowercase() {
            'f' => fahrenheit_converter(),
            'c' => celsius_converter(),
            'q' => break,
            _ => continue,
        }
    }
}
