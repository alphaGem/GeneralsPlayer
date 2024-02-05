/*
 * ### Table of Content
 * 
 * Basic Calculation
 * - Position{x:i8,y:i8}, with manhattan_diatance and chebyshev_distance
 * - Direction(u8): 1234 in cartesian => udlr
 * 
 * General Related
 * - GeneralType{Main,Sub,Mine}
 * - GeneralId(u8)
 * - SkillType{Dash,Kill,Atk,Def,Magic}
 * - SkillSet{dash,kill,atk,def,magic}
 * - Skill{cd,duration}
 * - AttrType{Prod,Def,Spd}
 * - AttrSet
 * - General{...}
 * - NOTHING:GeneralId = GeneralId(255)
 * 
 * Super Weapons and Upgrades
 * - SWType{Nuclear,Boost,Teleport,Freeze}
 * - SuperWeapon{...}
 * - TechType{Motor,Raft,Track,Relativity}
 * - Terrain{Plain,Sand,Swamp}
 * - Side{coin,tech,...}
 * 
 * GameState
 * - Attitude{Friendly,Hostile,Neutral}
 * - GameState{...}
 * 
 * Map
 * - MAP, DIST
 */

// `Position`, `Direction` and distances

// Math & json utils
pub mod utils;
pub use utils::{
    Position,
    Direction
};

// General
pub mod general;
pub use general::{
    GeneralId,
    GeneralType,
    Skill,
    SkillType,
    SkillSet,
    AttrType,
    AttrSet,
    General,
};

// GameState
pub mod gamestate;
pub use gamestate::{
    SWType,
    SuperWeapon,
    TechType,
    TechTree,
    GeneralStack,
    Side,
    Attitude,
    GameState,
};

pub mod skills;
pub mod operation;
pub use operation::Op;
pub mod map;
pub use map::Terrain;
pub mod colorize;

pub mod player;

pub use std;
pub use std::cmp;