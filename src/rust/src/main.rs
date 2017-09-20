mod loader;

use std::env;
use std::fs::File;
use std::process;
use std::io::prelude::*;

fn abort_if_err<T, E: std::fmt::Display>(r: Result<T, E>, err_str: &str) -> T {
    match r {
        Ok(t) => t,
        Err(error) => {
            eprintln!("{}: {}", err_str, error);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Invalid number of arguments");
        process::exit(1);
    }

    let infile = &args[1];
    let mut passwd = "";
    if args.len() == 3 {
        passwd = &args[2];
    }

    println!("infile {} passwd {}", infile, passwd);

    let mut file = abort_if_err(File::open(infile), "Failed to open file");

    let mut pemstr = String::new();
    abort_if_err(file.read_to_string(&mut pemstr), "Failed to read file");

    let (priv_key, pub_key) =
        abort_if_err(loader::load_pem_key(&pemstr, pemstr.len(), passwd), "Failed to load key");

    println!("Private Key: {}", priv_key);
    println!("Public  Key: {}", pub_key);
}
