extern crate json;
use std::cmp::max;
use std::fmt;
use crate::map;

pub fn json_to_vec(j:&json::JsonValue) -> Vec::<i32> {
    let mut v = vec![];
    for i in j.members() {
        v.push(i.as_i32().unwrap());
    }
    return v;
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
pub struct Direction(pub u8);
impl Direction{
    pub fn to_delta_pos(&self) -> Position {
        match self.0 {
            1 => Position{x:-1,y:0},
            2 => Position{x:1,y:0},
            3 => Position{x:0,y:-1},
            4 => Position{x:0,y:1},
            _ => panic!("Invalid Direction"),
        }
    }
}

pub fn manhattan_distance(pos1: Position, pos2: Position) -> i8 {
    return (pos1.x-pos2.x).abs()+(pos1.y-pos2.y).abs();
}

// used in general skill, superweapon range.
pub fn chebyshev_distance(pos1: Position, pos2: Position) -> i8 {
    return max((pos1.x-pos2.x).abs(),(pos1.y-pos2.y).abs());
}

pub fn reduce<T: Copy + Ord + std::ops::Sub<Output=T> + std::ops::Add<Output=T> >(x: &mut T, num: T) {
    *x = std::cmp::max(*x, num)-num;
}

static mut SEED:i64=523;
const MOD:i64=998244353;

pub fn srand() {
    unsafe {
        for i in 0..15 {
            for j in 0..15 {
                match map::MAP[i][j] {
                    map::Terrain::Plain => {SEED = (SEED * 1048583 + 2333347) % MOD;}
                    map::Terrain::Sand => {SEED = (SEED * 2333347 + 1048583) % MOD;}
                    map::Terrain::Swamp => {SEED = (SEED * 6666679 + 1048583) % MOD;}
                }
            }
        }
    }
}

/// returns a random number in [0,n), n<1e6
pub fn rand(n:i32) -> i32 {
    unsafe {
        SEED = (SEED * 19260817 + 6666679) % MOD;
        return (SEED as i32)%n;
    }
}