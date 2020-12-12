mod day_one;
mod day_two;
mod day_three;
mod day_four;

#[macro_use] extern crate lazy_static;
use std::io;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = String::from("Usage: rust_advent[.exe] day:int puzzle:int fileName:String. Example: rust_advent.exe 1 2 1-2");
    println!("{:?}", args);

    let day: u8 = match args.get(1) {
        None => {panic!(usage)}
        Some(s) => {s.parse().expect(&*usage)}
    };

    if day < 1 || day > 24 {panic!(usage)}

    let puzzle: u8 = match args.get(2) {
        None => {panic!(usage)}
        Some(s) => {s.parse().expect(&*usage)}
    };

    if puzzle < 1  || puzzle > 2 {panic!(usage)}

    let f_name = match args.get(3) {
        None => {panic!(usage)}
        Some(s) => {s}
    };

    let base = env::current_dir().unwrap();
    let path= match base.parent(){
        None => {base.join("inputFiles").join(f_name)}
        Some(parent) => {parent.join("inputFiles").join(f_name)}
    };

    match day {
        1 => {
            if puzzle == 1 {
                day_one::puzzle_one::run(&*path)
            }
            else {
                day_one::puzzle_two::run(&*path)
            }
        }
        2 => {
            if puzzle == 1 {
                day_two::puzzle_one::run(&*path)
            }
            else {
                day_two::puzzle_two::run(&*path)
            }
        }
        3 => {
            if puzzle == 1 {
                day_three::puzzle_one::run(&*path)
            }
            else {
                day_three::puzzle_two::run(&*path)
            }
        }
        4 => {
            if puzzle == 1 {
                day_four::puzzle_one::run(&*path)
            }
            else {
                day_four::puzzle_two::run(&*path)
            }
        }
        _ => {panic!(usage)}
    }
}
