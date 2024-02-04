use crate::*;
use std::fmt;

// Attitude
#[derive(PartialEq, Clone, Copy)]
pub enum Attitude {
    Friendly, Neutral, Hostile,
}
impl fmt::Display for Attitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attitude::Friendly => write!(f, "[Attitude::Friendly]"),
            Attitude::Hostile => write!(f, "[Attitude::Hostile]"),
            Attitude::Neutral => write!(f, "[Attitude::Neutral]")
        }
        
    }
}

// Super Weapons
#[derive(PartialEq, Clone, Copy)]
pub enum SWType {
    Nuclear, Boost, Teleport, Freeze, Pending,
}
#[derive(Clone, PartialEq)]
pub struct SuperWeapon {
    pub sw_type: SWType,
    pub pos: Position,
    pub duration: u8,
    pub cd: u8,
}
impl fmt::Display for SuperWeapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sw_type {
            SWType::Nuclear => write!(f, "[Nuclear] {} {} {}",self.pos,self.duration,self.cd),
            SWType::Boost => write!(f, "[Boost] {} {} {}",self.pos,self.duration,self.cd),
            SWType::Teleport => write!(f, "[Tele] {} {} {}",self.pos,self.duration,self.cd),
            SWType::Freeze => write!(f, "[Freeze] {} {} {}",self.pos,self.duration,self.cd),
            SWType::Pending => write!(f, "[Pending] {} {} {}",self.pos,self.duration,self.cd)
        }
        
    }
}
impl SuperWeapon {
    pub fn in_range(&self, pos: Position) -> bool {
        return utils::chebyshev_distance(self.pos, pos) <= 1;
    }
}

// Technology
#[derive(PartialEq, Clone, Copy)]
pub enum TechType {
    Motor, Raft, Track, Relativity,
}
#[derive(PartialEq, Clone, Copy)]
pub struct TechTree {
    pub motor: i8,
    pub raft: i8,
    pub track: i8,
    pub relativity: i8,
}
impl Default for TechTree {
    fn default() -> Self {
        TechTree {
            motor:1,
            raft:0,
            track:0,
            relativity:0,
        }
    }
}


// Side & Game State
#[derive(Clone, PartialEq)]
pub struct Side {
    pub tech_tree: TechTree,
    pub coin: i32,
    pub sw: Option<SuperWeapon>,
    pub seat: i8,
}

#[derive(Clone)]
pub struct GameState {
    pub owner: [[Attitude; 16]; 15],
    pub troop: [[i16; 16]; 15],
    pub cell: [[GeneralId; 16]; 15], // id=255 means no general here
    pub generals: Vec<General>,
    pub our: Side,
    pub their: Side,
    pub active_player_seat: i8,
    pub turn: i16,
    pub rest_march: i8,
}
