// RLS by Alexander Abraham, a.k.a. "The Black Unicorn"
// a.k.a. "angeldustduke.eth".
// Licensed under the MIT license.

use std::fs;
use std::env;
use std::path::Path;

// A function list files
// inside a given directory.
fn list_files(dir: String) {
    // We check if the path we want to know about
    // even exists.
    if Path::new(&dir).exists() {
        // We get a list of all available paths.
        let paths = fs::read_dir(&dir).unwrap();
        // We iterate through them and display them.
        for path in paths {
            println!("{}", path.unwrap().path().display());
        }
    }
    // If not, throw an error!
    else {
        println!("Path not found!");
    }
}

// Main entry-point for the Rust compiler.
fn main() {
    // We store the command-line arguments in a vector.
    let args: Vec<String> = env::args().collect();
    // We ascertain how many args were handed down.
    let arg_len = args.len();
    // If two arguments were handed to us,
    // we take the second as the main argument.
    if arg_len == 2 {
        // We get the relevant directory
        // from the [args] vector.
        let subject = args.get(1).unwrap();
        // We run the [list_files] function
        // on it.
        list_files(subject.to_string());
    }
    // If none are handed, we run the
    // [list_files] function in the
    // current directory.
    else if arg_len == 1{
        list_files(".".to_string());
    }
    // If anything else happens, we
    // print out this warning.
    else {
        println!("Wrong usage!");
    }
}
