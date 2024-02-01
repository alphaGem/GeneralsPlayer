use std;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::min;
use crate::utils;
use crate::*;


#[derive(Clone, Copy, PartialEq)]
pub enum Terrain {
    Plain,
    Sand,
    Swamp
}

// Map for the interesting game!
pub static mut MAP:[[Terrain;16];15] = [[Terrain::Plain;16];15];
// Distance when raft technology is unlocked
pub static mut DIST:[[[[i8;16];16];16];15] = [[[[15;16];16];16];15];

fn bfs(pos: Position) {
    let mut vis = [[false;15];15];
    let mut queue = vec![pos];
    let mut head = 0;
    let mut tail = 1;
    const DX:[i8;4] = [-1,0,1,0];
    const DY:[i8;4] = [0,-1,0,1];
    unsafe {DIST[pos.x as usize][pos.y as usize][pos.x as usize][pos.y as usize]=0;}
    while head < tail {
        let p = queue[head];
        head += 1;
        vis[p.x as usize][p.y as usize] = true;
        for d in 0..4 {
            let nx = p.x+DX[d];
            let ny = p.y+DY[d];
            if utils::chebyshev_distance(Position{x:nx,y:ny}, Position{x:7,y:7}) > 7 {continue;}
            if vis[nx as usize][ny as usize] {continue;}
            unsafe {
                if MAP[nx as usize][ny as usize]==Terrain::Swamp {continue;}
            }
            vis[nx as usize][ny as usize] = true;
            unsafe {
                let dist = &mut DIST[pos.x as usize][pos.y as usize];
                dist[nx as usize][ny as usize] = min(15,dist[p.x as usize][p.y as usize]+1);
            }
            queue.push(Position {x:nx,y:ny});
            tail += 1;
        }
    }
}

pub fn init_distance() {
    for x in 0..15 {
        for y in 0..15 {
            bfs(Position{x,y});
        }
    }
}

pub fn update_state(old_state: &GameState, parsed: &json::JsonValue) -> GameState {
    let os = match parsed["Player"].as_i32().unwrap() {
        0 => 0,
        1 => 1,
        -1 => 0,
        _ => panic!()
    };
    let ts = 1-os;
    let mut owner = old_state.owner;
    if old_state.our.seat != os {
        for i in 0..15 {
            for j in 0..16 {
                owner[i][j] = match owner[i][j] {
                    Attitude::Friendly => Attitude::Hostile,
                    Attitude::Hostile => Attitude::Friendly,
                    Attitude::Neutral => Attitude::Neutral,
                    
                }
            }
        }
    }
    let mut troop = old_state.troop;
    let mut cell = [[general::NOTHING;16];15];
    for cell in parsed["Cells"].members(){
        let iter = &mut cell.members();
        let pos = utils::json_to_vec(iter.next().unwrap());
        let plr = iter.next().unwrap().as_i32().unwrap();
        let trp = iter.next().unwrap().as_i32().unwrap();
        match plr {
            -1 => {}
            0|1 => {
                if plr as i8==os
                    {owner[pos[0]][pos[1]] = Attitude::Friendly;}
                else
                    {owner[pos[0]][pos[1]] = Attitude::Hostile;}
            }
            _ => {panic!();}
        }
        troop[pos[0]][pos[1]] = trp as i16;
    }
    let mut generals = vec![];
    for general in parsed["Generals"].members() {
        let pos = utils::json_to_vec(&general["Position"]);
        let plr = general["Player"].as_i32().unwrap();
        let att;
        match plr{
            -1 => {att=Attitude::Neutral}
            0|1 => {
                if plr as i8==os
                    {att = Attitude::Friendly;}
                else
                    {att = Attitude::Hostile;}
            }
            _ => {panic!();}
        }
        let gtype;
        match general["Type"].as_i32().unwrap() {
            1 => gtype=GeneralType::Main,
            2 => gtype=GeneralType::Sub,
            3 => gtype=GeneralType::Mine,
            _ => {panic!()}
        }
        let lvls = utils::json_to_vec(&general["Level"]);
        let cd = &mut general["Skill_cd"].members();
        let dur = &mut general["Skill_rest"].members();
        let skills = SkillSet {
            dash: Skill {cd: cd.next().unwrap().as_i8().unwrap(), duration: 0},
            kill: Skill {cd: cd.next().unwrap().as_i8().unwrap(), duration: 0},
            atk: Skill {cd: cd.next().unwrap().as_i8().unwrap(), duration: dur.next().unwrap().as_i8().unwrap()},
            def: Skill {cd: cd.next().unwrap().as_i8().unwrap(), duration: dur.next().unwrap().as_i8().unwrap()},
            magic: Skill {cd: cd.next().unwrap().as_i8().unwrap(), duration: dur.next().unwrap().as_i8().unwrap()},
        };
        let id = GeneralId(general["Id"].as_u8().unwrap());
        generals.push(General {
            skills,
            attr: AttrSet {prod: lvls[0] as i8, def: lvls[1] as i8, spd: lvls[2] as i8},
            pos: Position {x:pos[0] as i8, y:pos[1] as i8},
            attitude: att,
            general_type: gtype,
            alive: general["Alive"].as_i32().unwrap()!=0,
            id,
        });
        cell[pos[0]][pos[1]] = GeneralId(general["Id"].as_u8().unwrap());
    }
    let our_tech_tree = TechTree::default();
    let their_tech_tree = TechTree::default();
    
    generals.sort_by(|a,b| a.id.0.cmp(&b.id.0));
    let r = match parsed["Player"].as_i8().unwrap() {
        -1 => parsed["Round"].as_i16().unwrap()+1,
        _ => parsed["Round"].as_i16().unwrap(),
    };
    return GameState {
        owner: owner,
        troop: troop,
        cell: cell,
        generals: generals,
        our: Side {
            coin: parsed["Coins"][os as usize].as_i32().unwrap(),
            sw: None,
            tech_tree: our_tech_tree,
            seat: os,
        },
        their: Side {
            coin: parsed["Coins"][ts as usize].as_i32().unwrap(),
            sw: None,
            tech_tree: their_tech_tree,
            seat: ts,
        },
        active_player_seat: os,
        turn: r,
    }
}

pub fn read_init(is_player_mode: bool, filename: Option<&String>) -> GameState {
    let line;
    if is_player_mode {
        let file = File::open(Path::new(filename.unwrap())).unwrap();
        let br = BufReader::new(file);
        line = br.lines().next().unwrap().unwrap();
    }
    else {
        line = io::stdin().lock().lines().next().unwrap().unwrap();
    }
    
    let parsed = json::parse(&line).unwrap();
    let ctstr = parsed["Cell_type"].as_str().unwrap().as_bytes();
    for i in 0..15 {
        for j in 0..15 {
            unsafe {
                match ctstr[i*15+j] {
                    48 => MAP[i][j] = Terrain::Plain,
                    49 => MAP[i][j] = Terrain::Sand,
                    50 => MAP[i][j] = Terrain::Swamp,
                    _ => {panic!()}
                }
            }
        }
    }
    init_distance();
    let our_seat;
    let their_seat;
    if is_player_mode {
        our_seat = 0;
        their_seat = 1;
    }
    else {
        our_seat = parsed["Player"].as_i8().unwrap();
        their_seat = 1-our_seat;
    }
    return update_state(
        &GameState {
            owner: [[Attitude::Neutral;16];15],
            troop: [[0 as i16;16];15],
            cell: [[general::NOTHING;16];15],
            generals: vec![],
            our: Side {
                coin: 0,
                sw: None,
                tech_tree: TechTree::default(),
                seat: our_seat,
            },
            their: Side {
                coin: 0,
                sw: None,
                tech_tree: TechTree::default(),
                seat: their_seat,
            },
            active_player_seat: 0,
            turn: 0,
        },
        &parsed
    );
}