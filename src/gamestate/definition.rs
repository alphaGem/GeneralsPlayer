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
    Nuclear, Boost, Teleport, Freeze,
}
#[derive(Clone, PartialEq)]
pub struct SuperWeapon {
    pub sw_type: SWType,
    pub pos: Position,
    pub duration: i8,
    pub cd: i8,
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
#[derive(PartialEq, Clone, Copy, Default)]
pub struct TechTree {
    pub motor: i8,
    pub raft: i8,
    pub track: i8,
    pub relativity: i8,
}


// Side & Game State
#[derive(Clone, PartialEq)]
pub struct Side {
    pub tech_tree: TechTree,
    pub coin: i32,
    pub sw: Option<SuperWeapon>,
    pub seat: i8,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub owner: [[Attitude; 16]; 15],
    pub troop: [[i16; 16]; 15],
    pub cell: [[GeneralId; 16]; 15], // id=255 means no general here
    pub generals: Vec<General>,
    pub our: Side,
    pub their: Side,
    pub active_player_seat: i8,
    pub turn: i16,
}
