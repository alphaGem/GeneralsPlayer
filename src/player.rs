use crate::*;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn nxt(lines: &Vec<String>, j: &mut usize, gs: gamestate::GameState, display_limit: i16, do_check: bool) -> gamestate::GameState{
    let parsed_old = json::parse(&lines[*j-1]).unwrap();
    let parsed = json::parse(&lines[*j]).unwrap();
    *j += 1;
    let action = operation::Op::from(utils::json_to_vec(&parsed["Action"]));
    let new_gs = map::update_state_by_json(&gs, &parsed, false);
    if new_gs.turn >= display_limit {new_gs.print()};
    if do_check {
        let check_gs = match (parsed_old["Player"].as_i32().unwrap(),parsed["Player"].as_i32().unwrap()) {
            (-1,-1) => operation::apply_op(&operation::apply_op(&gs, operation::Op::End), operation::Op::End),
            (0,0)|(1,1)|(1,-1)|(-1,0) => operation::apply_op(&gs, action),
            (0,1)|(0,-1)|(-1,1) => operation::apply_op(&operation::apply_op(&gs, operation::Op::End), action),
            _ => panic!()
        };
        check_gs.check_with_replay_file(&new_gs);
        return check_gs;
    }
    new_gs
}

pub fn play(do_check: bool) {
    // let filename = io::stdin().lock().lines().next().unwrap().unwrap();
    let filename = "map.json".to_string();
    let (init_gs,_) = map::read_init(true, Some(&filename));
    init_gs.print();
    let file = File::open(Path::new(&filename)).unwrap();
    let br = BufReader::new(file);
    let mut lines=vec![];
    for line in br.lines() {
        lines.push(line.unwrap());
    }
    let mut gs = init_gs.clone();
    let mut j=1;
    loop {
        let l = io::stdin().lock().lines().next().unwrap().unwrap();
        let mut v:Vec<char> = l.chars().collect();
        v.push('\n');
        match v[0] {
            '\n' => {
                gs = nxt(&lines, &mut j, gs, -1, do_check);
            },
            'j' => {
                let pair = l.split_once(' ');
                if let Some((_, second)) = pair {
                    let x = second.parse::<i16>().unwrap();
                    gs = init_gs.clone();
                    j = 1;
                    while gs.turn < x {
                        gs = nxt(&lines, &mut j, gs, x, do_check);
                    }
                }
            }
            _ => {
            }
        }
    }
}