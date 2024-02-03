use crate::*;
use crate::utils::manhattan_distance;

macro_rules! CHECK {
    ($condition:expr) => {{
        if !$condition {
            return false
        }
    }}
}

const DASH_COST:i32 = 20;
const KILL_COST:i32 = 15;
const ATK_COST:i32 = 30;
const DEF_COST:i32 = 30;
const MAGIC_COST:i32 = 30;
const DASH_CD: i8 = 5;
const KILL_CD: i8 = 10;
const ATK_CD: i8 = 10;
const DEF_CD: i8 = 10;
const MAGIC_CD: i8 = 10;

impl gamestate::GameState {
    /*
     * Get defence in a specific cell
     * Defence is affected by:
     * - general
     * - our general skill: def
     * - their general skill: magic
     * - superweapon
     */
    fn get_def(&self, pos: Position) -> f32 {
        let mut def = 1.0f32;
        // general effect
        let id = self.cell[pos.x as usize][pos.y as usize];
        if id != general::NOTHING {
            let general = &self.generals[id.0 as usize];
            match general.general_type {
                GeneralType::Main | GeneralType::Sub => {
                    match general.attr.def {
                        1 => {},
                        2 => def *= 2.0,
                        3 => def *= 3.0,
                        _ => {panic!()}
                    }
                }
                GeneralType::Mine => {
                    match general.attr.def {
                        1 => {},
                        2 => def *= 1.5,
                        3 => def *= 2.0,
                        4 => def *= 3.0,
                        _ => {panic!()}
                    }
                }
            }
        }
        // general skill effect
        let defender_attitude = self.owner[pos.x as usize][pos.y as usize];
        for general in &self.generals {
            if !general.in_range(pos) {continue;}
            if general.attitude == defender_attitude {
                if general.skills.def.duration > 0 {
                    def *= 1.5;
                }
            }
            else {
                if general.skills.magic.duration > 0 {
                    def *= 0.75;
                }
            }
        }
        // sw effect
        if let Some(sw) = &self.our.sw {
            if sw.sw_type == SWType::Boost && sw.in_range(pos) {
                def *= 3.0; // the sw is really strong!
            }
        }
        return def;
    }

    /*
     * Get attack in a specific cell
     * Attack is affect by:
     * - our general skill: atk
     * - their general skill: magic
     * - superweapon
     */
    fn get_atk(&self, pos: Position) -> f32 {
        let mut atk = 1.0f32;
        // general skill effect
        for general in &self.generals {
            if !general.in_range(pos) {continue;}
            let attacker_attitude = self.owner[pos.x as usize][pos.y as usize];
            for general in &self.generals {
                if general.attitude == attacker_attitude {
                    if general.skills.atk.duration > 0 {
                        atk *= 1.5;
                    }
                }
                else {
                    if general.skills.magic.duration > 0 {
                        atk *= 0.75;
                    }
                }
            }
        }
        // sw effect
        if let Some(sw) = &self.our.sw {
            if sw.sw_type == SWType::Boost && sw.in_range(pos) {
                atk *= 3.0; // the sw is really strong!
            }
        }
        return atk;
    }


    fn attack(&mut self, src_pos: Position, dst_pos: Position, num: i16) {
        if self.owner[src_pos.x as usize][src_pos.y as usize] == self.owner[dst_pos.x as usize][dst_pos.y as usize] {
            self.troop[dst_pos.x as usize][dst_pos.y as usize] += num;
            self.troop[src_pos.x as usize][src_pos.y as usize] -= num;
            return;
        }
        let atk = self.get_atk(src_pos);
        let def = self.get_def(dst_pos);
        let atk_troop = num as f32;
        let def_troop = self.troop[dst_pos.x as usize][dst_pos.y as usize] as f32;
        let force = atk*atk_troop-def*def_troop;
        self.troop[src_pos.x as usize][src_pos.y as usize] -= num;
        if force > 0.0 { // success
            let remaining = (force/atk).ceil() as i16;
            let attacker = self.owner[src_pos.x as usize][src_pos.y as usize];
            let gid = self.cell[dst_pos.x as usize][dst_pos.y as usize];
            self.troop[dst_pos.x as usize][dst_pos.y as usize] = remaining;
            self.owner[dst_pos.x as usize][dst_pos.y as usize] = attacker;
            if gid != general::NOTHING {
                self.generals[gid.0 as usize].attitude = attacker;
            }
        }
        else { // fail
            let remaining = (force/def).ceil() as i16;
            self.troop[dst_pos.x as usize][dst_pos.y as usize] = remaining;
        }
    }

    fn can_conquer(&self, src_pos: Position, dst_pos: Position) -> bool {
        let atk = self.get_atk(src_pos);
        let def = self.get_def(dst_pos);
        let atk_troop = self.troop[src_pos.x as usize][src_pos.y as usize] as f32;
        let def_troop = self.troop[dst_pos.x as usize][dst_pos.y as usize] as f32;
        let force = atk*(atk_troop-1.0)-def*def_troop;
        return force > 0.0;
    }

    fn check_unfrozen(&self, pos: Position) -> bool {
        if let Some(sw) = &self.our.sw {
            if sw.sw_type == SWType::Teleport && sw.pos == pos {
                return false;
            }
            if sw.sw_type == SWType::Freeze && sw.in_range(pos){
                return false;
            }
        }
        if let Some(sw) = &self.their.sw {
            if sw.sw_type == SWType::Freeze && sw.in_range(pos){
                return false;
            }
        }
        return true;
    }

    fn check_movability(&self, src_pos: Position, dst_pos: Position) -> bool {
        CHECK!(self.check_unfrozen(src_pos));
        unsafe {
            if map::MAP[dst_pos.x as usize][dst_pos.y as usize] == Terrain::Swamp {
                CHECK!(self.our.tech_tree.raft==1);
            }
        }
        return true;
    }

    // normal attack / movement
    pub fn march(&mut self, src_pos: Position, dst_pos: Position, num: i16) {
        if !self.check_march(src_pos, dst_pos, num) {panic!("Invalid march!")}
        self.attack(src_pos, dst_pos, num);
    }
    pub fn check_march(&self, src_pos: Position, dst_pos: Position, num: i16) -> bool {
        CHECK!(self.owner[src_pos.x as usize][src_pos.y as usize]==Attitude::Friendly);
        CHECK!(self.check_movability(src_pos, dst_pos));
        CHECK!(num>0);
        CHECK!(self.troop[src_pos.x as usize][src_pos.y as usize]-num>=1);
        CHECK!(utils::manhattan_distance(src_pos, dst_pos)<=1);
        return true;
    }

    // general move
    pub fn shift(&mut self, general_id: GeneralId, dst_pos: Position) {
        if !self.check_shift(general_id, dst_pos) {panic!("invalid shift!");}
        let general = &mut self.generals[general_id.0 as usize];
        let pos = general.pos;
        general.pos = dst_pos;
        self.cell[pos.x as usize][pos.y as usize] = general::NOTHING;
        self.cell[dst_pos.x as usize][dst_pos.y as usize] = general_id;
    }

    pub fn check_shift(&self, general_id: GeneralId, dst_pos: Position) -> bool {
        let general = &self.generals[general_id.0 as usize];
        CHECK!(self.check_movability(general.pos, dst_pos));
        CHECK!(general.general_type != GeneralType::Mine);
        let distance;
        if self.our.tech_tree.raft == 1 {
            distance = manhattan_distance(general.pos, dst_pos);
        }
        else {
            unsafe {
                distance = map::DIST[general.pos.x as usize][general.pos.y as usize][dst_pos.x as usize][dst_pos.y as usize];
            }
        }
        eprintln!("dis check {} {}", distance, general.rest_shift);
        CHECK!(distance > 0);
        CHECK!(distance <= general.rest_shift);
        return true;
    }

    fn common_check(&self, general: &General, skill_type: SkillType, maybe_dst_pos: Option<Position>) -> bool {
        // General should be friendly
        CHECK!(general.attitude == Attitude::Friendly);
        CHECK!(general.general_type != GeneralType::Mine);
        CHECK!(self.check_unfrozen(general.pos));
        CHECK!(general.alive);
        if let Some(dst_pos) = maybe_dst_pos {
            CHECK!(general.in_range(dst_pos));
        }
        match skill_type {
            SkillType::Dash => {
                CHECK!(general.skills.dash.cd==0);
                CHECK!(self.our.coin >= DASH_COST);
            }
            SkillType::Kill => {
                CHECK!(general.skills.kill.cd==0);
                CHECK!(self.our.coin >= KILL_COST);
            }
            SkillType::Atk => {
                CHECK!(general.skills.atk.cd==0);
                CHECK!(self.our.coin >= ATK_COST);
            }
            SkillType::Def => {
                CHECK!(general.skills.def.cd==0);
                CHECK!(self.our.coin >= DEF_COST);
            }
            SkillType::Magic => {
                CHECK!(general.skills.magic.cd==0);
                CHECK!(self.our.coin >= MAGIC_COST);
            }
        }
        return true;
    }

    /*
     * Dash Skill
     */
    pub fn dash(&mut self, general_id: GeneralId, dst_pos: Position) {
        if !self.check_dash(general_id, dst_pos) {panic!("Invalid dash");}
        let general = &mut self.generals[general_id.0 as usize];
        self.our.coin -= DASH_COST;
        general.skills.dash.cd = DASH_CD;
        let pos = general.pos;
        self.attack(pos, dst_pos, self.troop[pos.x as usize][pos.y as usize]-1);
        let general = &mut self.generals[general_id.0 as usize];
        general.pos = dst_pos;
        general.skills.dash.cd = DASH_CD;
        self.cell[pos.x as usize][pos.y as usize]=general::NOTHING;
        self.cell[dst_pos.x as usize][dst_pos.y as usize]=general_id;
    }

    // dash additional checklist
    // - no general at destination
    // - can conquer
    pub fn check_dash(&self, general_id: GeneralId, dst_pos: Position) -> bool{
        let general = &self.generals[general_id.0 as usize];
        CHECK!(self.common_check(general, SkillType::Dash, Some(dst_pos)));
        CHECK!(self.check_movability(general.pos, dst_pos));
        CHECK!(self.cell[dst_pos.x as usize][dst_pos.y as usize] == general::NOTHING);
        CHECK!(self.can_conquer(general.pos, dst_pos));
        return true;
    }

    /*
     * Kill Skill
     */
    pub fn kill(&mut self, general_id: GeneralId, dst_pos: Position) {
        if !self.check_kill(general_id, dst_pos) {panic!("Invalid kill");}
        let general = &mut self.generals[general_id.0 as usize];
        self.our.coin -= KILL_COST;
        general.skills.kill.cd = KILL_CD;
    }

    pub fn check_kill(&self, general_id: GeneralId, dst_pos: Position) -> bool {
        let general = &self.generals[general_id.0 as usize];
        CHECK!(self.common_check(general, SkillType::Kill, Some(dst_pos)));
        return true;
    }

    /*
     * Atk/Def/Magic Skill
     */
    pub fn buff(&mut self, general_id: GeneralId, skill_type: SkillType) {
        if !self.check_buff(general_id, skill_type) {panic!("Invalid buff");}
        let general = &mut self.generals[general_id.0 as usize];
        match skill_type {
            SkillType::Atk => {self.our.coin -= ATK_COST; general.skills.atk.cd = ATK_CD;}
            SkillType::Def => {self.our.coin -= DEF_COST; general.skills.def.cd = DEF_CD;}
            SkillType::Magic => {self.our.coin -= MAGIC_COST; general.skills.magic.cd = MAGIC_CD;}
            _ => {panic!("Invalid buff");}
        }
    }
    pub fn check_buff(&self, general_id: GeneralId, skill_type: SkillType) -> bool{
        let general = &self.generals[general_id.0 as usize];
        CHECK!(self.common_check(general, skill_type, None));
        return true;
    }

    /*
     * Nuclear Bomb
     */
    pub fn nuclear(&mut self, pos: Position) {
        assert!(self.check_nuclear(pos));
        let xmin = cmp::max(pos.x-1, 0);
        let xmax = cmp::min(pos.x+1, 15);
        let ymin = cmp::max(pos.y-1, 0);
        let ymax = cmp::min(pos.y+1, 15);
        let omp = self.get_main(Attitude::Friendly).unwrap().pos;
        let tmp = self.get_main(Attitude::Hostile).unwrap().pos;
        for x in xmin..xmax+1 {
            for y in ymin..ymax+1 {
                let p = Position{x,y};
                if p == omp || p == tmp {
                    self.troop[x as usize][y as usize] /= 2;
                }
                else {
                    self.troop[x as usize][y as usize] = 0;
                    self.owner[x as usize][y as usize] = Attitude::Neutral;
                    let gid = self.cell[x as usize][y as usize];
                    if gid != general::NOTHING {
                        self.generals[gid.0 as usize].alive = false;
                        self.cell[x as usize][y as usize] = general::NOTHING;
                    }
                }
            }
        }
        self.our.sw = Some(SuperWeapon {
            sw_type: SWType::Nuclear,
            pos,
            duration: 5,
            cd: 50,
        });
    }
    pub fn check_nuclear(&self, _pos: Position) -> bool {
        CHECK!(self.our.sw == None);
        CHECK!(self.our.tech_tree.relativity>0);
        return true;
    }

    /*
     * Boost
     */
    pub fn boost(&mut self, pos: Position) {
        assert!(self.check_boost(pos));
        self.our.sw = Some(SuperWeapon {
            sw_type: SWType::Boost, 
            pos,
            duration: 5,
            cd: 50
        });
    }
    pub fn check_boost(&self, _pos: Position) -> bool {
        CHECK!(self.our.sw == None);
        CHECK!(self.our.tech_tree.relativity>0);
        return true;
    }

    /*
     * Teleport
     */
    pub fn teleport(&mut self, pos: Position, src_pos: Position) {
        assert!(self.check_teleport(pos, src_pos));
        let num = self.troop[src_pos.x as usize][src_pos.y as usize]-1;
        self.troop[pos.x as usize][pos.y as usize] = num;
        self.owner[pos.x as usize][pos.y as usize] = Attitude::Friendly;
        self.our.sw = Some(SuperWeapon {
            sw_type: SWType::Teleport,
            pos,
            duration: 2,
            cd: 50,
        });
    }
    pub fn check_teleport(&mut self, pos: Position, src_pos: Position) -> bool {
        CHECK!(self.our.sw == None);
        CHECK!(self.our.tech_tree.relativity>0);
        CHECK!(self.owner[src_pos.x as usize][src_pos.y as usize]==Attitude::Friendly);
        CHECK!(self.cell[pos.x as usize][pos.y as usize]==general::NOTHING);
        CHECK!(self.troop[src_pos.x as usize][src_pos.y as usize]>1);
        return true;
    }

    /*
     * Freeze
     */
    pub fn freeze(&mut self, pos: Position) {
        assert!(self.check_freeze(pos));
        self.our.sw = Some(SuperWeapon {
            sw_type: SWType::Freeze,
            pos,
            duration: 10,
            cd: 50,
        })
    }
    pub fn check_freeze(&mut self, _pos: Position) -> bool {
        CHECK!(self.our.sw == None);
        CHECK!(self.our.tech_tree.relativity>0);
        return true;
    }
}