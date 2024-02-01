use crate::*;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn nxt(lines: &Vec<String>, j: &mut usize, gs: gamestate::GameState, display_limit: i16, do_check: bool) -> gamestate::GameState{
    let parsed_old = json::parse(&lines[*j-1]).unwrap();
    let parsed = json::parse(&lines[*j]).unwrap();
    *j += 1;
    let action = operation::Op::from(utils::json_to_vec(&parsed["Action"]));
    let check_gs;
    if parsed_old["Player"].as_i32().unwrap() == parsed["Player"].as_i32().unwrap() || parsed["Player"].as_i32().unwrap()!=1 {
        check_gs = operation::apply_op(&gs, action);
    }
    else{
        check_gs = operation::apply_op(&operation::apply_op(&gs, operation::Op::End), action);
    }
    let new_gs = map::update_state(gs, parsed);
    if new_gs.turn >= display_limit {new_gs.print()};
    if check_gs != new_gs {
        new_gs.print();
        check_gs.print();
        eprintln!("Error: Unmatch!");
        eprintln!("Generals {}", new_gs.generals==check_gs.generals);
        let n=new_gs.generals.len();
        for i in 0..n {
            if new_gs.generals[i] != check_gs.generals[i] {
                eprintln!("diff {} att {} {} dashcd {} {}", i, new_gs.generals[i].attitude, check_gs.generals[i].attitude,
                    new_gs.generals[i].skills.dash.cd, check_gs.generals[i].skills.dash.cd
                );
            }
        }
        eprintln!("ActivePlayer {} {}", new_gs.active_player_seat,check_gs.active_player_seat);
        eprintln!("Our Side {}", new_gs.our==check_gs.our);
        eprintln!("Our seat {} {}", new_gs.our.seat, check_gs.our.seat);
        eprintln!("Our coin {} {}", new_gs.our.coin, check_gs.our.coin);
        eprintln!("Their coin {} {}", new_gs.their.coin, check_gs.their.coin);
        eprintln!("Their Side {}", new_gs.their==check_gs.their);
        eprintln!("Cell {} | Troop {} | Owner {}", new_gs.cell == check_gs.cell,new_gs.troop==check_gs.troop, new_gs.owner==check_gs.owner);
        
    }
    new_gs
}

pub fn play(do_check: bool) {
    // let filename = io::stdin().lock().lines().next().unwrap().unwrap();
    let filename = "map.json".to_string();
    let init_gs = map::read_init(true, Some(&filename));
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