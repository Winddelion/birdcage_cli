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
    println!("-v --change-vault -> change default vault");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _verbose = false;
}
