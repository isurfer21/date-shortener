extern crate chrono;
extern crate getopts;

use getopts::Options;
use chrono::prelude::*;
use std::env;

mod endec;

const VERSION: &str = "0.2.0";

const L10N_APPVER: &str = "
Date Shortener
Version [VER]
Licensed under MIT License
";

const L10N_APPINFO: &str = "
Date Shortener 
It is a tool to shorten (encode) the date and expand (decode) shortened date back to original date.
";

const L10N_APPCLIEX: &str = "
Examples: 
 $ ds -v 
 $ ds -t 
 $ ds -t -s 
 $ ds -e 15/08/19 
 $ ds -e 15/08/2019 -s 
 $ ds -d f8j 
 $ ds -d f8kj -s 
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optopt("e", "encode", "encode the provided date", "DD-MM-YYYY");
    opts.optopt("d", "decode", "decode the provided code", "DMY");

    opts.optflag("t", "today", "encode today's date");
    opts.optflag("s", "steps", "show with steps");
    opts.optflag("h", "help", "display the help menu");
    opts.optflag("v", "version", "display the application version");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        println!("{}", L10N_APPVER.replace("[VER]", VERSION));
        return;
    }

    let mut steps_flag = false;
    if matches.opt_present("s") {
        steps_flag = true;
    }
    
    if matches.opt_present("t") {
        let local: DateTime<Local> = Local::now();
        let dmy = endec::encode(local.day(), local.month(), local.year(), steps_flag);
        println!("{}", dmy);
    }

    let date = matches.opt_str("e");
    match date {
        Some(date) => {
                let tmp: Vec<&str> = date.split([',', '.', '-', '/'].as_ref()).collect();
                if tmp.len() == 3 {
                    let dmy = endec::encode(tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), tmp[2].parse().unwrap(), steps_flag);
                    println!("{}", dmy);
                } else {
                    println!("Invalid date: {:?}", date);
                }
        },
        None => do_nothing(),
    }

    let code = matches.opt_str("d");
    match code {
        Some(code) => {
            let dmy = endec::decode(code, steps_flag);
            println!("{}", dmy);
        },
        None => do_nothing(),
    }
}

fn do_nothing() {}

fn print_usage(program: &str, opts: Options) {
    println!("{}", L10N_APPINFO);
    let brief = format!("Usage: {} [flag] [options]", program);
    print!("{}", opts.usage(&brief));
    println!("{}", L10N_APPCLIEX);
}