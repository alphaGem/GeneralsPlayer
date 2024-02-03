extern crate json;
mod sample_ai;
use main::*;
use std::io::{self, BufRead};
use std::env;    

fn run_ai() {
    let (mut gs, real_our_seat) = map::read_init(true, Some(&"map.json".to_string()));
    loop {
        if gs.active_player_seat == real_our_seat {
            gs =sample_ai::sample_ai(gs);
            // gs.print();
        }
        else {
            loop {
                let line = io::stdin().lock().lines().next().unwrap().unwrap();
                let ints = line.split(' ');
                let mut vec = vec![];
                for i in ints {
                    vec.push(i.parse::<usize>().unwrap());
                }
                let op = Op::from(vec);
                gs = operation::apply_op(&gs, op);
                // gs.print();
                match op {
                    Op::End => {break;}
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    player::play(true);
    // run_ai();
}