use std::env;
use std::fs;

fn print_help() {
    println!("birdcage password manager");
    println!("Usage:");
    println!("Use 'birdcage [HANDLER] | [OPTIONS]");
    println!("List of possible options");
    println!("-l or --list -> list all available handlers");
    println!("-a -> add a new password");
    println!("-d -> delete password");
    println!("-rh -> rename the handler");
    println!("-rp -> change password");
}

fn request_password() {
    println!("please enter current vault's master pasword: ")
}

fn lorem_ipsum() {
    println!("Something")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _verbose = false;

    let mut arg_check = 1;
    while args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
            }
            "-l" | "--list" => {
                lorem_ipsum();
            }
            "-a" | "--add" => {
                lorem_ipsum();
            }
            "-d" | "--delete" => {
                lorem_ipsum();
            }
            "-rh" => {
                lorem_ipsum();
            }
            "-rp" => {
                lorem_ipsum();
            }
            _ => {
                eprintln!("Error. Failed to parse arguments");
            }
        }
    }
}
