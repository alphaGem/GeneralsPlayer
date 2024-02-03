extern crate json;
use std::cmp::max;

pub fn json_to_vec(j:&json::JsonValue) -> Vec::<usize> {
    let mut v = vec![];
    for i in j.members() {
        v.push(i.as_usize().unwrap());
    }
    return v;
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
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