use crate::*;

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
            let general = self.generals.get(id).unwrap();
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
            if !general.alive {continue;}
            if general.attitude == defender_attitude {
                if general.skills.def.cd > 0 {
                    def *= 1.5;
                }
            }
            else {
                if general.skills.magic.cd > 0 {
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
        // eprintln!("def {}", def);
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
            if !general.alive {continue;}
            let attacker_attitude = self.owner[pos.x as usize][pos.y as usize];
            if general.attitude == attacker_attitude {
                if general.skills.atk.cd > 0 {
                    atk *= 1.5;
                }
            }
            else {
                if general.skills.magic.cd > 0 {
                    atk *= 0.75;
                }
            }
        }
        // sw effect
        if let Some(sw) = &self.our.sw {
            if sw.sw_type == SWType::Boost && sw.in_range(pos) {
                atk *= 3.0; // the sw is really strong!
            }
        }
        // eprintln!("atk {}", atk);
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
        // eprintln!("atk {} * atkt {} - def {} * deft {} = {}", atk, atk_troop, def, def_troop, force);
        self.troop[src_pos.x as usize][src_pos.y as usize] -= num;
        if force > 0.0 { // success
            let remaining = (force/atk).ceil() as i16;
            let attacker = self.owner[src_pos.x as usize][src_pos.y as usize];
            let gid = self.cell[dst_pos.x as usize][dst_pos.y as usize];
            self.troop[dst_pos.x as usize][dst_pos.y as usize] = remaining;
            self.owner[dst_pos.x as usize][dst_pos.y as usize] = attacker;
            if gid != general::NOTHING {
                self.generals.get_mut(gid).unwrap().attitude = attacker;
            }
        }
        else { // fail
            let remaining = (-force/def).ceil() as i16;
            self.troop[dst_pos.x as usize][dst_pos.y as usize] = remaining;
        }
    }

    fn can_conquer(&self, src_pos: Position, dst_pos: Position) -> bool {
        if self.owner[dst_pos.x as usize][dst_pos.y as usize]==Attitude::Friendly {return true;}
        let atk = self.get_atk(src_pos);
        let def = self.get_def(dst_pos);
        let atk_troop = self.troop[src_pos.x as usize][src_pos.y as usize] as f32;
        let def_troop = self.troop[dst_pos.x as usize][dst_pos.y as usize] as f32;
        let force = atk*(atk_troop-1.0)-def*def_troop;
        return force > 0.0;
    }

    pub fn check_unfrozen(&self, pos: Position) -> bool {
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
        self.attack(src_pos, dst_pos, std::cmp::min(num, self.troop[src_pos.x as usize][src_pos.y as usize]-1));
        self.rest_march -= 1;
    }
    pub fn check_march(&self, src_pos: Position, dst_pos: Position, num: i16) -> bool {
        CHECK!(self.rest_march>0);
        CHECK!(self.owner[src_pos.x as usize][src_pos.y as usize]==Attitude::Friendly);
        CHECK!(num>0);
        CHECK!(self.troop[src_pos.x as usize][src_pos.y as usize]>=2);
        CHECK!(self.check_movability(src_pos, dst_pos));
        CHECK!(utils::manhattan_distance(src_pos, dst_pos)<=1);
        return true;
    }

    fn get_general_distance(&self, src: Position, dst: Position, limit: i8) -> i32 {
        let mut dis = [[255i32;16];15];
        let mut queue = vec![src];
        let mut head = 0;
        let mut tail = 1;
        const DX:[i8;4] = [-1,0,1,0];
        const DY:[i8;4] = [0,-1,0,1];
        dis[src.x as usize][src.y as usize] = 0;
        while head < tail {
            let p = queue[head];
            head += 1;
            for d in 0..4 {
                let nx = p.x+DX[d];
                let ny = p.y+DY[d];
                if utils::chebyshev_distance(Position{x:nx,y:ny}, Position{x:7,y:7}) > 7 { 
                    continue;
                }
                if self.our.tech_tree.raft == 0 {
                    unsafe {
                        if map::MAP[nx as usize][ny as usize]==Terrain::Swamp {
                            continue;
                        }
                    }
                }
                if self.owner[nx as usize][ny as usize] != Attitude::Friendly {
                    continue;
                }
                if self.cell[nx as usize][ny as usize] != general::NOTHING {
                    continue;
                }
                if (Position{x:nx, y:ny})==dst {
                    return dis[p.x as usize][p.y as usize]+1;
                }
                if dis[nx as usize][ny as usize] <= dis[p.x as usize][p.y as usize]+1 {
                    continue;
                }
                dis[nx as usize][ny as usize] = dis[p.x as usize][p.y as usize]+1;
                if dis[nx as usize][ny as usize] < limit as i32 {
                    queue.push(Position {x:nx,y:ny});
                    tail += 1;
                }
            }
        }
        return (limit+1) as i32;
    }

    // general move
    pub fn shift(&mut self, general_id: GeneralId, dst_pos: Position) {
        if !self.check_shift(general_id, dst_pos) {panic!("invalid shift!");}
        let general = self.generals.get(general_id).unwrap();
        let pos = general.pos;
        let rest_shift = general.rest_shift;
        let d = self.get_general_distance(pos, dst_pos, rest_shift);
        let general = self.generals.get_mut(general_id).unwrap();
        general.rest_shift -= d as i8;
        general.pos = dst_pos;
        self.cell[pos.x as usize][pos.y as usize] = general::NOTHING;
        self.cell[dst_pos.x as usize][dst_pos.y as usize] = general_id;
    }


    pub fn check_shift(&self, general_id: GeneralId, dst_pos: Position) -> bool {
        CHECK!(self.cell[dst_pos.x as usize][dst_pos.y as usize]==general::NOTHING);
        CHECK!(self.owner[dst_pos.x as usize][dst_pos.y as usize]==Attitude::Friendly);
        let general = self.generals.get(general_id).unwrap();
        CHECK!(self.check_movability(general.pos, dst_pos));
        CHECK!(general.general_type != GeneralType::Mine);
        let distance = self.get_general_distance(general.pos, dst_pos, general.rest_shift);
        // eprintln!("dis check {} {} {} {}", general.pos, dst_pos, distance, general.rest_shift);
        CHECK!(distance <= general.rest_shift as i32);
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
        self.our.coin -= DASH_COST;
        let general = self.generals.get_mut(general_id).unwrap();
        let pos = general.pos;
        self.attack(pos, dst_pos, self.troop[pos.x as usize][pos.y as usize]-1);
        let general = self.generals.get_mut(general_id).unwrap();
        general.pos = dst_pos;
        general.skills.dash.cd = DASH_CD;
        self.cell[pos.x as usize][pos.y as usize]=general::NOTHING;
        self.cell[dst_pos.x as usize][dst_pos.y as usize]=general_id;
    }

    // dash additional checklist
    // - no general at destination
    // - can conquer
    pub fn check_dash(&self, general_id: GeneralId, dst_pos: Position) -> bool{
        let general = self.generals.get(general_id).unwrap();
        CHECK!(self.common_check(general, SkillType::Dash, Some(dst_pos)));
        CHECK!(self.check_movability(general.pos, dst_pos));
        CHECK!(self.cell[dst_pos.x as usize][dst_pos.y as usize] == general::NOTHING);
        CHECK!(self.troop[general.pos.x as usize][general.pos.y as usize] >= 2);
        CHECK!(self.can_conquer(general.pos, dst_pos));
        return true;
    }

    /*
     * Kill Skill
     */
    pub fn kill(&mut self, general_id: GeneralId, dst_pos: Position) {
        if !self.check_kill(general_id, dst_pos) {panic!("Invalid kill");}
        let general = self.generals.get_mut(general_id).unwrap();
        self.our.coin -= KILL_COST;
        general.skills.kill.cd = KILL_CD;
        Self::attrition(
            &mut self.troop[dst_pos.x as usize][dst_pos.y as usize],
            &mut self.owner[dst_pos.x as usize][dst_pos.y as usize],
            self.cell[dst_pos.x as usize][dst_pos.y as usize],
            20
        );
    }

    pub fn check_kill(&self, general_id: GeneralId, dst_pos: Position) -> bool {
        let general = self.generals.get(general_id).unwrap();
        CHECK!(self.common_check(general, SkillType::Kill, Some(dst_pos)));
        return true;
    }

    /*
     * Atk/Def/Magic Skill
     */
    pub fn buff(&mut self, general_id: GeneralId, skill_type: SkillType) {
        if !self.check_buff(general_id, skill_type) {panic!("Invalid buff");}
        let general = self.generals.get_mut(general_id).unwrap();
        match skill_type {
            SkillType::Atk => {self.our.coin -= ATK_COST; general.skills.atk.cd = ATK_CD;}
            SkillType::Def => {self.our.coin -= DEF_COST; general.skills.def.cd = DEF_CD;}
            SkillType::Magic => {self.our.coin -= MAGIC_COST; general.skills.magic.cd = MAGIC_CD;}
            _ => {panic!("Invalid buff");}
        }
    }
    pub fn check_buff(&self, general_id: GeneralId, skill_type: SkillType) -> bool{
        let general = self.generals.get(general_id).unwrap();
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
                        self.generals.get_mut(gid).unwrap().alive = false;
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
    pub fn check_teleport(&self, pos: Position, src_pos: Position) -> bool {
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
    pub fn check_freeze(&self, _pos: Position) -> bool {
        CHECK!(self.our.sw == None);
        CHECK!(self.our.tech_tree.relativity>0);
        return true;
    }
}