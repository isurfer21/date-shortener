extern crate chrono;
extern crate getopts;

use getopts::Options;
use chrono::prelude::*;
use std::env;

const BASE36_CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optopt("d", "date", "encode the provided date", "DD-MM-YYYY");
    opts.optopt("c", "code", "decode the provided code", "DMY");

    opts.optflag("t", "today", "encode today's date");
    opts.optflag("e", "explain", "explain with steps");
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
        println!("Date Shortener \nVersion {} \nLicensed under MIT License", VERSION);
        return;
    }

    let mut explain_flag = false;
    if matches.opt_present("e") {
        explain_flag = true;
    }
    
    if matches.opt_present("t") {
        let local: DateTime<Local> = Local::now();
            let dmy = to_date36(local.day(), local.month(), local.year(), explain_flag);
            println!("{}", dmy);
    }

    let date = matches.opt_str("d");
    match date {
        Some(date) => {
                let tmp: Vec<&str> = date.split([',', '.', '-', '/'].as_ref()).collect();
                if tmp.len() == 3 {
                    let dmy = to_date36(tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), tmp[2].parse().unwrap(), explain_flag);
                    println!("{}", dmy);
                } else {
                    println!("Invalid date: {:?}", date);
                }
        },
        None => do_nothing(),
    }

    let code = matches.opt_str("c");
    match code {
        Some(code) => {
            let dmy = to_date_str(code, explain_flag);
                    println!("{}", dmy);
        },
        None => do_nothing(),
    }
}

fn do_nothing() {}

fn print_usage(program: &str, opts: Options) {
    println!("Date Shortener is a tool to shorten (encode) the date and expand (decode) shortened date back to original date. \n");
    let brief = format!("Usage: {} [flag] [options]", program);
    print!("{}", opts.usage(&brief));
    println!("\nExamples: \n $ ds -v \n $ ds -t \n $ ds -t -e \n $ ds -d 15/08/2019 \n $ ds -d 15/08/2019 -e \n $ ds -c f8j \n $ ds -c f8j -e \n");
}

fn to_date36(day:u32, month:u32, long_year:i32, explain:bool) -> String {
    let symbols: Vec<char> = BASE36_CHARSET.chars().collect();
    let short_year = long_year % 100;
    let d36 = symbols[(day % 36) as usize];
    let m36 = symbols[(month % 36) as usize];
    let y36 = symbols[(short_year % 36) as usize];
    let dmy = format!("{}{}{}", d36, m36, y36).to_ascii_lowercase();
    let dmy_explained = format!("{}-{}-{} -> {}.{}.{} -> {}.{}.{} -> {}", day, month, long_year, day, month, short_year, d36, m36, y36, dmy);
    return if explain { dmy_explained } else { dmy };
}

fn to_date_str(date36:String, explain: bool) -> String {
    let date36_uc = date36.to_ascii_uppercase();
    let date_sym: Vec<char> = date36_uc.chars().collect();
    let day = BASE36_CHARSET.find(date_sym[0]).unwrap();
    let month = BASE36_CHARSET.find(date_sym[1]).unwrap();
    let short_year = BASE36_CHARSET.find(date_sym[2]).unwrap();
    let long_year = short_year as i32 + 2000;
    let dmy = format!("{}-{}-{}", day, month, long_year);
    let dmy_explained = format!("{} -> {}.{}.{} -> {}.{}.{} -> {}", date36, date_sym[0], date_sym[1], date_sym[2], day, month, short_year, dmy);
    return if explain { dmy_explained } else { dmy };
}