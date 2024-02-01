use std::mem::swap;
use crate::*;

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

impl General {
    fn reduce_cd(&mut self) {
        let s = &mut self.skills;
        for skill in vec![&mut s.dash, &mut s.kill, &mut s.atk, &mut s.def, &mut s.magic] {
            if skill.cd > 0 {
                skill.cd -= 1;
            }
        }
        // TODO rest move
        /*match self.attr.spd {
            1 => self.rest_move = 1,
            2 => self.rest_move = 2,
            3 => self.rest_move = 3,
            _ => panic!()
        }*/
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
            }
        );
        self.cell[pos.x as usize][pos.y as usize] = id;
    }
    pub fn check_call_general(&self, pos: Position) -> bool {
        CHECK!(self.our.coin >= 50);
        CHECK!(self.cell[pos.x as usize][pos.y as usize] == general::NOTHING);
        return true;
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
            }
        }
    }
    pub fn check_tech_advancement(&self, tt: TechType) -> bool {
        // TODO
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
        swap(&mut self.our.coin, &mut self.their.coin);
        swap(&mut self.our.seat, &mut self.their.seat);
        swap(&mut self.our.sw, &mut self.their.sw);
        if self.active_player_seat == 0 && self.turn>0 {
            self.end_turn();
            
        }
    }

    fn end_turn(&mut self) {
        if self.turn % 10 == 0 {
            for i in 0..15 {
                for j in 0..15 {
                    match self.owner[i][j] {
                        Attitude::Friendly | Attitude::Hostile => {
                            self.troop[i][j] += 1;
                        }
                        Attitude::Neutral => {}
                    }
                }
            }
        }
        for general in &mut self.generals {
            match general.general_type {
                GeneralType::Main | GeneralType::Sub => {
                    match general.attitude {
                        Attitude::Friendly | Attitude::Hostile => {
                            let prod;
                            match general.attr.prod {
                                1 => prod=1,
                                2 => prod=2,
                                3 => prod=4,
                                _ => panic!("Wrong production level?"),
                            }
                            self.troop[general.pos.x as usize][general.pos.y as usize] += prod;
                            general.reduce_cd();
                        }
                        Attitude::Neutral => {}
                    }
                }
                GeneralType::Mine => {
                    match general.attitude {
                        Attitude::Friendly | Attitude::Hostile => {
                            let prod;
                            match general.attr.prod {
                                1 => prod=1,
                                2 => prod=2,
                                3 => prod=4,
                                4 => prod=6,
                                _ => panic!("Wrong production level?"),
                            }
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
        self.turn += 1;
    }

}