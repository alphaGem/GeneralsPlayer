use crate::*;

// GeneralType, GeneralId

#[derive(Clone, Copy, PartialEq)]
pub enum GeneralType { Main, Sub, Mine }


#[derive(Clone, Copy, PartialEq)]
pub struct GeneralId(pub u8);

// Skills: Dash, Kill, Atk, Def, Magic

#[derive(Clone, Copy, PartialEq)]
pub struct Skill {
    pub cd: i8,
    pub duration: i8,
}
impl Default for Skill {
    fn default() -> Self {
        Skill {
            cd: 0i8,
            duration: 0i8,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SkillType {
    Dash, Kill, Atk, Def, Magic,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct SkillSet {
    pub dash: Skill,
    pub kill: Skill,
    pub atk: Skill,
    pub def: Skill,
    pub magic: Skill,
}

// Attributes aka leveling up stuffs

#[derive(Clone, Copy, PartialEq)]
pub enum AttrType {
    Prod, Def, Spd,
}

#[derive(Clone,Copy, PartialEq)]
pub struct AttrSet {
    pub prod: i8,
    pub def: i8,
    pub spd: i8,
}
impl Default for AttrSet {
    fn default() -> Self {
        AttrSet {
            prod:1,
            def:1,
            spd:1,
        }
    }
}

// General

#[derive(Clone, PartialEq)]
pub struct General {
    pub skills: SkillSet,
    pub attr: AttrSet,
    pub pos: Position,
    pub attitude: Attitude,
    pub general_type: GeneralType,
    pub alive: bool,
    pub id: GeneralId,
}


impl General {
    pub fn in_range(&self, pos: Position) -> bool {
        utils::chebyshev_distance(self.pos, pos) <= 2
    }
}

pub const NOTHING:GeneralId = GeneralId(255);