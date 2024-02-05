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
                    vec.push(i.parse::<i32>().unwrap());
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

fn double_ai() {
    let (mut gs, real_our_seat) = map::read_init(true, Some(&"map.json".to_string()));
    loop {
        gs =sample_ai::sample_ai(gs);
        gs.print();
        if gs.turn == 300 {
            break;
        }
        if gs.get_main(Attitude::Friendly) == None {
            break;
        }
        if gs.get_main(Attitude::Hostile) == None {
            break;
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    player::play(true);
    // run_ai();
    // double_ai();
}