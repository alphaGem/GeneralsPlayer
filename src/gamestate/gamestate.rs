use std::mem::swap;
use crate::*;
use crate::colorize::Colorize;

macro_rules! CHECK {
    ($condition:expr) => {{
        if !$condition {
            return false
        }
    }}
}


pub const MAIN_UPGRADE_COST:[[i32;5];3] = [[0,20,40,0,0],[0,20,50,0,0],[0,10,20,0,0]];
pub const SUB_UPGRADE_COST:[[i32;5];3] = [[0,40,80,0,0],[0,40,100,0,0],[0,20,40,0,0]];
pub const MINE_UPGRADE_COST:[[i32;5];3] = [[0,10,25,35,0],[0,10,15,30,0],[0,0,0,0,0]];

pub fn get_promotion_cost(general: &General, attr: AttrType) -> i32 {
    match general.general_type {
        GeneralType::Main => {
            match attr {
                AttrType::Prod => MAIN_UPGRADE_COST[0][general.attr.prod as usize],
                AttrType::Def  => MAIN_UPGRADE_COST[1][general.attr.def as usize],
                AttrType::Spd  => MAIN_UPGRADE_COST[2][general.attr.spd as usize],
            }
        }
        GeneralType::Sub => {
            match attr {
                AttrType::Prod => SUB_UPGRADE_COST[0][general.attr.prod as usize],
                AttrType::Def  => SUB_UPGRADE_COST[1][general.attr.def as usize],
                AttrType::Spd  => SUB_UPGRADE_COST[2][general.attr.spd as usize],
            }
        }
        GeneralType::Mine => {
            match attr {
                AttrType::Prod => MINE_UPGRADE_COST[0][general.attr.prod as usize],
                AttrType::Def  => MINE_UPGRADE_COST[1][general.attr.def as usize],
                AttrType::Spd  => MINE_UPGRADE_COST[2][general.attr.spd as usize],
            }
        }
    }
}

pub fn get_tech_cost(tech: TechType, tree: TechTree) -> i32 {
    match tech {
        TechType::Motor => {
            if tree.motor == 1 {80}
            else if tree.motor == 2 {150}
            else {0}
        }
        TechType::Raft =>       if tree.raft==0         {100}   else {0},
        TechType::Track =>      if tree.track==0        {75}    else {0},
        TechType::Relativity => if tree.relativity==0   {250}   else {0},
    }
}

impl General {
    fn reduce_cd(&mut self) {
        let s = &mut self.skills;
        for skill in vec![&mut s.dash, &mut s.kill, &mut s.atk, &mut s.def, &mut s.magic] {
            if skill.cd > 0 {
                skill.cd -= 1;
            }
        }
        match self.attr.spd {
            1 => self.rest_shift = 1,
            2 => self.rest_shift = 2,
            3 => self.rest_shift = 4,
            _ => panic!()
        }
    }
}

impl GameState {
    /*
     * Call a general to fight for you!
     */
    pub fn call_general(&mut self, pos: Position) {
        if !self.check_call_general(pos) {panic!("You should check_call_general first");}
        self.our.coin -= 50;
        let id = GeneralId(self.generals.len() as u8);
        self.generals.push(
            General {
                skills: SkillSet::default(),
                attr: AttrSet::default(),
                pos: pos,
                attitude: Attitude::Friendly,
                general_type: GeneralType::Sub,
                alive: true,
                id: id,
                rest_shift: 1,
            }
        );
        self.cell[pos.x as usize][pos.y as usize] = id;
    }
    pub fn check_call_general(&self, pos: Position) -> bool {
        eprintln!("Check call {} {}", pos, self.owner[pos.x as usize][pos.y as usize]);
        CHECK!(self.our.coin >= 50);
        CHECK!(self.cell[pos.x as usize][pos.y as usize] == general::NOTHING);
        CHECK!(self.owner[pos.x as usize][pos.y as usize] == Attitude::Friendly);
        CHECK!(self.check_unfrozen(pos));
        return true;
    }

    pub fn get_main(&self, attitude: Attitude) -> Option<&General>{
        for g in &self.generals {
            if g.general_type == GeneralType::Main && g.attitude == attitude {
                return Some(&g);
            }
        }
        return None;
    }

    
    /*
     * General Promoting!
     */
    pub fn promote(&mut self, id: GeneralId, attr: AttrType) {
        if !self.check_promote(id, attr) {panic!("Invalid upgrade!")}
        let general = &mut self.generals[id.0 as usize];
        let cost = get_promotion_cost(general, attr);
        self.our.coin -= cost;
        match attr {
            AttrType::Prod => general.attr.prod += 1,
            AttrType::Def  => general.attr.def += 1,
            AttrType::Spd  => general.attr.spd += 1,
        }
    }
    pub fn check_promote(&self, id: GeneralId, attr: AttrType) -> bool {
        let general = &self.generals[id.0 as usize];
        CHECK!(general.attitude == Attitude::Friendly);
        CHECK!(general.alive);
        let cost = get_promotion_cost(general, attr);
        // reached max lvl
        CHECK!(cost!=0);
        // no enough coin
        CHECK!(self.our.coin>=cost);
        return true;
    }

    /*
     * Tech Upgrading!
     */
    pub fn tech_advancement(&mut self, tt: TechType) {
        if !self.check_tech_advancement(tt) {panic!("Invalid Tech Upgrading!")}
        let cost = get_tech_cost(tt, self.our.tech_tree);
        self.our.coin -= cost;
        match tt {
            TechType::Motor => {
                self.our.tech_tree.motor += 1;
            }
            TechType::Raft => {
                self.our.tech_tree.raft += 1;
            }
            TechType::Track => {
                self.our.tech_tree.track += 1;
            }
            TechType::Relativity => {
                self.our.tech_tree.relativity += 1;
                self.our.sw = Some(SuperWeapon {
                    sw_type: SWType::Pending,
                    pos: Position{x:0,y:0},
                    duration: 0,
                    cd: 10,
                })
            }
        }
    }
    pub fn check_tech_advancement(&self, tt: TechType) -> bool {
        let cost = get_tech_cost(tt, self.our.tech_tree);
        CHECK!(cost > 0);
        CHECK!(self.our.coin >= cost);
        return true;
    }


    pub fn flip(&mut self) {
        self.active_player_seat = match self.active_player_seat {
            0 => 1,
            1 => 0,
            _ => panic!()
        };
        for i in 0..15 {
            for j in 0..16 {
                match self.owner[i][j] {
                    Attitude::Friendly => {
                        self.owner[i][j] = Attitude::Hostile;
                    }
                    Attitude::Hostile => {
                        self.owner[i][j] = Attitude::Friendly;
                    }
                    Attitude::Neutral => {}
                }
            }
        }
        for general in &mut self.generals {
            match general.attitude {
                Attitude::Friendly => {
                    general.attitude = Attitude::Hostile;
                }
                Attitude::Hostile => {
                    general.attitude = Attitude::Friendly;
                }
                Attitude::Neutral => {}
            }
        }
        swap(&mut self.our, &mut self.their);
        self.rest_march = match self.our.tech_tree.motor {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => panic!()
        };
        if self.active_player_seat == 0 && self.turn>0 {
            self.end_turn();
            
        }
    }

    pub fn attrition(troop: &mut i16, owner: &mut Attitude, cell: GeneralId, num: i16) {
        utils::reduce(troop, num);
        // Place become neutral if attrition killed all units on it
        if *troop == 0 && cell == general::NOTHING {
            *owner = Attitude::Neutral;
        }
    }

    fn radiation(troop: &mut [[i16; 16]; 15], owner: &mut[[Attitude;16];15], cell: &mut[[GeneralId;16];15], pos: Position) {
        let xmin = cmp::max(pos.x-1, 0);
        let xmax = cmp::min(pos.x+1, 15);
        let ymin = cmp::max(pos.y-1, 0);
        let ymax = cmp::min(pos.y+1, 15);
        for x in xmin..xmax+1 {
            for y in ymin..ymax+1 {
                Self::attrition(
                    &mut troop[x as usize][y as usize],
                    &mut owner[x as usize][y as usize],
                    cell[x as usize][y as usize],
                    3
                )
            }
        }
    }

    fn end_turn(&mut self) {
        for i in 0..15 {
            for j in 0..15 {
                if self.turn % 10 == 0 {
                    match self.owner[i][j] {
                        Attitude::Friendly | Attitude::Hostile => {
                            self.troop[i][j] += 1;
                        }
                        Attitude::Neutral => {}
                    }
                }
                unsafe {
                    let track_not_unlocked = match self.owner[i][j] {
                        Attitude::Friendly => self.our.tech_tree.track==0,
                        Attitude::Hostile => self.their.tech_tree.track==0,
                        Attitude::Neutral => false,
                    };
                    if track_not_unlocked && map::MAP[i][j] == Terrain::Sand {
                        Self::attrition(
                            &mut self.troop[i][j],
                            &mut self.owner[i][j],
                            self.cell[i][j],
                            1
                        );
                        // eprintln!("attrition {} {} {}",i,j,self.owner[i][j]);
                    }
                }
            }
        }
        for general in &mut self.generals {
            match general.general_type {
                GeneralType::Main | GeneralType::Sub => {
                    match general.attitude {
                        Attitude::Friendly | Attitude::Hostile => {
                            let prod = match general.attr.prod {
                                1 => 1, 2 => 2, 3 => 4,
                                _ => panic!("Wrong production level?"),
                            };
                            self.troop[general.pos.x as usize][general.pos.y as usize] += prod;
                            general.reduce_cd();
                        }
                        Attitude::Neutral => {}
                    }
                }
                GeneralType::Mine => {
                    match general.attitude {
                        Attitude::Friendly | Attitude::Hostile => {
                            let prod = match general.attr.prod {
                                1 => 1, 2 => 2, 3 => 4, 4 => 6,
                                _ => panic!("Wrong production level?"),
                            };
                            if general.attitude == Attitude::Friendly {
                                self.our.coin += prod;
                            }
                            else {
                                self.their.coin += prod;
                            }
                        }
                        Attitude::Neutral => {}
                    }
                }
            }
        }
        if let Some(sw) = &mut self.our.sw {
            if sw.sw_type == SWType::Nuclear {
                Self::radiation(&mut self.troop, &mut self.owner, &mut self.cell, sw.pos);
            }
            utils::reduce(&mut sw.duration, 1);
            if sw.duration == 0 {
                sw.sw_type = SWType::Pending;
                sw.pos = Position{x:0,y:0};
            }
            sw.cd -= 1;
            if sw.cd == 0 {
                self.our.sw = None;
            }
        }
        if let Some(sw) = &mut self.their.sw {
            if sw.sw_type == SWType::Nuclear {
                Self::radiation(&mut self.troop, &mut self.owner, &mut self.cell, sw.pos);
            }
            utils::reduce(&mut sw.duration, 1);
            if sw.duration == 0 {
                sw.sw_type = SWType::Pending;
                sw.pos = Position{x:0,y:0};
            }
            sw.cd -= 1;
            if sw.cd == 0 {
                self.their.sw = None;
            }
        }
        self.turn += 1;
    }

    fn detailed_check(&self, gs:&GameState) -> bool {
        let mut ret=true;
        if !(self.cell == gs.cell && self.troop==gs.troop && self.owner==gs.owner) {
            eprintln!("Cell {} | Troop {} | Owner {}", self.cell == gs.cell,self.troop==gs.troop, self.owner==gs.owner);
            ret=false;
        }
        if self.our!=gs.our {
            eprintln!("Our side unmatch");
            ret=false;
        }
        if self.their != gs.their {
            eprintln!("Their side unmatch");
            ret=false;
        }
        let n=self.generals.len();
        let m = gs.generals.len();
        if n!=m {
            eprintln!("Generals length unmatch!");
        }
        for i in 0..n {
            let gg = &gs.generals[i];
            // because replay file doesn't have the rest_shift info, we need to do tricky things here
            let ng = General {
                skills: gg.skills,
                attr: gg.attr,
                pos: gg.pos,
                attitude: gg.attitude,
                general_type: gg.general_type,
                alive: gg.alive,
                id: gg.id,
                rest_shift: self.generals[i].rest_shift,
            };
            if self.generals[i] != ng {
                eprintln!("General diff {} attitude {} {} dashcd {} {} killcd {} {}, buff cd {}/{} {}/{} {}/{}",
                    i, 
                    self.generals[i].attitude, gs.generals[i].attitude,
                    self.generals[i].skills.dash.cd, gs.generals[i].skills.dash.cd,
                    self.generals[i].skills.kill.cd, gs.generals[i].skills.kill.cd,
                    self.generals[i].skills.atk.cd, gs.generals[i].skills.atk.cd,
                    self.generals[i].skills.def.cd, gs.generals[i].skills.def.cd,
                    self.generals[i].skills.magic.cd, gs.generals[i].skills.magic.cd,
                );
                ret=false;
            }
        }
        if self.active_player_seat!=gs.active_player_seat {
            eprintln!("Active player unmatch {} {}", self.active_player_seat, gs.active_player_seat);
            ret=false;
        }
        if self.turn != gs.turn {
            eprintln!("Turn unmatch {} {}", self.turn, gs.turn);
        }
        return ret;
    }

    pub fn check_with_replay_file(&self, gs: &GameState) {
        match self.detailed_check(gs) {
            true => {

            }
            false => {
                gs.print();
                self.print();
                eprintln!("{}", "Error: Simulated unmatch with Replay File".bold().red());
                panic!();
            }
        }
    }

}