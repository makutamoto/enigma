extern crate enigma;
use enigma::{ Rotor, Enigma };

use std::io::Read;

fn main() {
    let rotors = vec!{
        Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", Some('V')),
        Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", Some('E')),
        Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", None),
    };
    let refrector = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
    let mut enigma = Enigma::new(rotors, refrector);
    let mut verbose = false;
    let mut option_type = "";
    for (i, arg) in std::env::args().enumerate() {
        if i == 0 { continue; }
        if option_type == "" {
            if &arg[0..1] == "-" {
                if &arg[1..2] == "-" {
                    match &arg[2..] {
                        "help" => show_help(),
                        "plugboard" => option_type = "plugboard",
                        "rotors" => option_type = "rotors",
                        "seed" => option_type = "seed",
                        "verbose" => verbose = true,
                        _ => {
                            println!("Invalid option: {}", arg);
                            std::process::exit(-1);
                        }
                    }
                } else {
                    for option in arg[1..].chars() {
                        match option {
                            'h' => show_help(),
                            'p' => option_type = "plugboard",
                            'r' => option_type = "rotors",
                            's' => option_type = "seed",
                            'v' => verbose = true,
                            _ => {
                                println!("Invalid option: {}", arg);
                                std::process::exit(-1);
                            }
                        }
                    }
                }
            } else {
                eprintln!("Invalid value: {}", arg);
                std::process::exit(-1);
            }
        } else {
            match option_type {
                "plugboard" => {
                    if arg.len() != 26 {
                        eprintln!("Number of plugboard characters must be 26.");
                        std::process::exit(-1);
                    }
                    enigma.plugboard = arg;
                },
                "rotors" => {
                    enigma.rotors.clear();
                    for rotor in arg.split(',') {
                        if rotor.len() != 26 {
                            println!("Number of rotor characters must be 26.");
                            std::process::exit(-1);
                        }
                        enigma.rotors.push(enigma::Rotor::new(rotor, None));
                    }
                },
                "seed" => {
                    for (i, character) in arg.char_indices() {
                        if i == enigma.rotor_offsets.len() {
                            eprintln!("Too many seed digits, it must be equal to or less than {}.", enigma.rotor_offsets.len());
                            std::process::exit(-1);
                        }
                        enigma.rotor_offsets[i] = character as i32 - 'A' as i32;
                    }
                },
                _ => (),
            }
            option_type = "";
        }
    }
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    data = enigma.encrypt(&data);
    if verbose {
        print!("{}", enigma.log);
        println!("\nResult: {}", data);
    } else {
        println!("{}", data);
    }
}

fn show_help() {
    println!("Enigma machine simulator

USAGE:
    echo [SOMETHING] | enigma [OPTIONS]
    -- Enigma reads standard input for its input.

OPTIONS:

    -v, --verbose Print internal sequence log
    -s, --seed    Specify a seed
    -h, --help    Prints this summary");
    std::process::exit(0);
}
