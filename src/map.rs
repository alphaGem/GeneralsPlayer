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
                dist[nx as usize][ny as usize] = min(63,dist[p.x as usize][p.y as usize]+1);
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


/// Warning: the GameState returned by this function has `rest_shift=0` for all generals
/// and `rest_march=2` for the game state, because they are not specified in the replay 
/// file.
pub fn update_state_by_json(old_state: &GameState, parsed: &json::JsonValue, force_os_zero: bool) -> GameState {
    let os;
    if force_os_zero {
        os = 0;
    }
    else {
        os = match parsed["Player"].as_i32().unwrap() {
            0 => 0,
            1 => 1,
            -1 => 0,
            _ => panic!()
        };
    }
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
            -1 => {
                owner[pos[0] as usize][pos[1] as usize] = Attitude::Neutral;
            }
            0|1 => {
                if plr as i8==os
                    {owner[pos[0] as usize][pos[1] as usize] = Attitude::Friendly;}
                else
                    {owner[pos[0] as usize][pos[1] as usize] = Attitude::Hostile;}
            }
            _ => {panic!();}
        }
        troop[pos[0] as usize][pos[1] as usize] = trp as i16;
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
            dash: Skill {cd: cd.next().unwrap().as_i8().unwrap()},
            kill: Skill {cd: cd.next().unwrap().as_i8().unwrap()},
            atk: Skill {cd: cd.next().unwrap().as_i8().unwrap()},
            def: Skill {cd: cd.next().unwrap().as_i8().unwrap()},
            magic: Skill {cd: cd.next().unwrap().as_i8().unwrap()},
        };
        assert!(dur.next().unwrap().as_i8().unwrap()==skills.atk.cd);
        assert!(dur.next().unwrap().as_i8().unwrap()==skills.def.cd);
        assert!(dur.next().unwrap().as_i8().unwrap()==skills.magic.cd);
        let id = GeneralId(general["Id"].as_u8().unwrap());
        generals.push(General {
            skills,
            attr: AttrSet {prod: lvls[0] as i8, def: lvls[1] as i8, spd: lvls[2] as i8},
            pos: Position {x:pos[0] as i8, y:pos[1] as i8},
            attitude: att,
            general_type: gtype,
            alive: general["Alive"].as_i32().unwrap()!=0,
            id,
            rest_shift: 0,
        });
        cell[pos[0] as usize][pos[1] as usize] = id;
    }
    let mut parsed_tech = parsed["Tech_level"].members();
    
    let o = utils::json_to_vec(parsed_tech.next().unwrap());
    let t = utils::json_to_vec(parsed_tech.next().unwrap());
    let tts=vec![
    TechTree {
        motor: o[0] as i8,
        raft: o[1] as i8,
        track: o[2] as i8,
        relativity: o[3] as i8,
    }, TechTree {
        motor: t[0] as i8,
        raft: t[1] as i8,
        track: t[2] as i8,
        relativity: t[3] as i8,
    }];
    
    generals.sort_by(|a,b| a.id.0.cmp(&b.id.0));
    let r;
    if force_os_zero {
        r=1;
    }
    else {
        r = match parsed["Player"].as_i8().unwrap() {
            -1 => parsed["Round"].as_i16().unwrap()+1,
            _ => parsed["Round"].as_i16().unwrap(),
        };
    }
    let cds = utils::json_to_vec(&parsed["Weapon_cds"]);
    let mut oswt = SWType::Pending;
    let mut oswp = vec![0,0];
    let mut oswd = 0;
    let mut tswt = SWType::Pending;
    let mut tswp = vec![0,0];
    let mut tswd = 0;
    //"Weapons": [{"Type": 2, "Player": 0, "Position": [6, 7], "Rest": 5}]
    for sw in parsed["Weapons"].members() {
        if sw["Rest"].as_u8().unwrap()==0 {
            continue;
        }
        let swt = match sw["Type"].as_i32().unwrap() {
            1 => {SWType::Nuclear},
            2 => {SWType::Boost},
            3 => {SWType::Teleport},
            4 => {SWType::Freeze},
            _ => panic!(),
        };
        if sw["Player"].as_i8().unwrap() == os {
            oswt = swt;
            oswp = utils::json_to_vec(&sw["Position"]);
            oswd = sw["Rest"].as_u8().unwrap();
        }
        if sw["Player"].as_i8().unwrap() == ts {
            tswt = swt;
            tswp = utils::json_to_vec(&sw["Position"]);
            tswd = sw["Rest"].as_u8().unwrap();
        }
    }
    let osw = match cds[os as usize] {
        -1|0 => None,
        _ => {
            Some(SuperWeapon {
                sw_type: oswt,
                pos: Position{x:oswp[0] as i8,y:oswp[1] as i8},
                duration: oswd,
                cd: cds[os as usize] as u8,
            })
        }
    };
    let tsw = match cds[ts as usize] {
        -1|0 => None,
        _ => {
            Some(SuperWeapon {
                sw_type: tswt,
                pos: Position{x:tswp[0] as i8,y:tswp[1] as i8},
                duration: tswd,
                cd: cds[ts as usize] as u8,
            })
        }
    };
    eprintln!("cds {} {} dur {} {}", cds[os as usize], cds[ts as usize], oswd, tswd);
    return GameState {
        owner: owner,
        troop: troop,
        cell: cell,
        generals: generals,
        our: Side {
            coin: parsed["Coins"][os as usize].as_i32().unwrap(),
            sw: osw,
            tech_tree: tts[os as usize],
            seat: os,
        },
        their: Side {
            coin: parsed["Coins"][ts as usize].as_i32().unwrap(),
            sw: tsw,
            tech_tree: tts[ts as usize],
            seat: ts,
        },
        active_player_seat: os,
        turn: r,
        rest_march: 2,
    }
}

pub fn read_init(is_player_mode: bool, filename: Option<&String>) -> (GameState, i8) {
    let mut line = String::new();
    if is_player_mode {
        if let Ok(file) = File::open(Path::new(filename.unwrap())) {
            let br = BufReader::new(file);
            line = br.lines().next().unwrap().unwrap();
        }
        else {
            io::stdin().read_line(&mut line).unwrap();
        }
    }
    else {
        io::stdin().read_line(&mut line).unwrap();
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
    utils::srand();
    let mut our_seat;
    let their_seat;
    our_seat = parsed["Player"].as_i8().unwrap();
    if our_seat == -1 {
        our_seat = 0;
    }
    their_seat = 1-our_seat;
    return (update_state_by_json(
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
            rest_march: 2,
        },
        &parsed,
        true
    ),our_seat);
}