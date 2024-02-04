use crate::*;
use std::io;
use std::fmt;
use std::io::Write;
use crate::colorize::Colorize;


#[derive(Clone, Copy)]
pub enum Op {
    March(Position, Direction, i16),
    Shift(GeneralId, Position),
    Promote(GeneralId, AttrType),
    Skill(GeneralId, SkillType, Option<Position>),
    Tech(TechType),
    SuperWeapon(SWType, Position, Option<Position>),
    Call(Position),
    End,
}

impl From<Vec<i32>> for Op {
    fn from(v: Vec<i32>) -> Op {
        match v[0] {
            1 => {
                Op::March(
                    Position{x:v[1] as i8, y:v[2] as i8},
                    Direction(v[3] as u8),
                    v[4] as i16,
                )
            },
            2 => {
                Op::Shift(
                    GeneralId(v[1] as u8),
                    Position{x: v[2] as i8, y: v[3] as i8},
                )
            },
            3 => {
                let attr = match v[2] {
                    1 => AttrType::Prod,
                    2 => AttrType::Def,
                    3 => AttrType::Spd,
                    _ => panic!(),
                };
                Op::Promote(
                    GeneralId(v[1] as u8),
                    attr
                )
            }
            4 => {
                let st = match v[2] {
                    1 => SkillType::Dash,
                    2 => SkillType::Kill,
                    3 => SkillType::Atk,
                    4 => SkillType::Def,
                    5 => SkillType::Magic,
                    _ => panic!(),
                };
                match v[2] {
                    3|4|5 => Op::Skill(
                        GeneralId(v[1] as u8),
                        st,
                        None
                    ),
                    1|2 => Op::Skill(
                        GeneralId(v[1] as u8),
                        st,
                        Some(Position{x:v[3] as i8,y:v[4] as i8}),
                    ),
                    _ => panic!(),
                }
            }
            5 => {
                let tt = match v[1] {
                    1 => TechType::Motor,
                    2 => TechType::Raft,
                    3 => TechType::Track,
                    4 => TechType::Relativity,
                    _ => panic!(),
                };
                Op::Tech(tt)
            }
            6 => {
                let st = match v[1] {
                    1 => SWType::Nuclear,
                    2 => SWType::Boost,
                    3 => SWType::Teleport,
                    4 => SWType::Freeze,
                    _ => panic!()
                };
                match v[1] {
                    1|2|4 => Op::SuperWeapon(
                        st, 
                        Position{x:v[2] as i8, y:v[3] as i8},
                        None,
                    ),
                    3 => Op::SuperWeapon(
                        st, 
                        Position{x:v[2] as i8, y:v[3] as i8},
                        Some(Position{x:v[4] as i8, y:v[5] as i8}),
                    ),
                    _ => panic!()
                }
            },
            7 => {
                Op::Call(Position{x:v[1] as i8, y: v[2] as i8})
            }
            8 => {
                Op::End
            }
            9 => {
                eprintln!("{}", "Game over!".bold().blue());
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::March(pos, dir, num) => {
                write!(f,"1 {} {} {} {}", pos.x, pos.y, dir.0, num)
            },
            Op::Shift(id, pos) => {
                write!(f,"2 {} {} {}", id.0, pos.x, pos.y)
            }
            Op::Promote(id, t) => {
                let t_num = match t {
                    AttrType::Prod => 1,
                    AttrType::Def => 2,
                    AttrType::Spd => 3,
                };
                write!(f,"3 {} {}", id.0, t_num)
            }
            Op::Skill(id, t, maybe_pos) => {
                let t_num = match t {
                    SkillType::Dash => 1,
                    SkillType::Kill => 2,
                    SkillType::Atk => 3,
                    SkillType::Def => 4,
                    SkillType::Magic => 5,
                };
                if let Some(pos) = maybe_pos {
                    write!(f,"4 {} {} {} {}", id.0, t_num, pos.x, pos.y)
                }
                else {
                    write!(f,"4 {} {}", id.0, t_num)
                }
            }
            Op::Tech(t) => {
                let t_num = match t {
                    TechType::Motor => 1,
                    TechType::Raft => 2,
                    TechType::Track => 3,
                    TechType::Relativity => 4,
                };
                write!(f,"5 {}", t_num)
            }
            Op::SuperWeapon(t, dst_pos, maybe_src_pos) => {
                let t_num = match t {
                    SWType::Nuclear => 1,
                    SWType::Boost => 2,
                    SWType::Teleport => 3,
                    SWType::Freeze => 4,
                    SWType::Pending => panic!(),
                };
                if let Some(src_pos) = maybe_src_pos {
                    write!(f,"6 {} {} {} {} {}", t_num, dst_pos.x, dst_pos.y, src_pos.x, src_pos.y)
                }
                else {
                    write!(f,"6 {} {} {}", t_num, dst_pos.x, dst_pos.y)
                }
            }
            Op::Call(pos) => {
                write!(f,"7 {} {}", pos.x, pos.y)
            }
            Op::End => {
                write!(f,"8")
            }
        }
    }
}

pub fn send_op(ops: Vec::<Op>) {
    let mut output = String::new();
    for op in ops {
        output = format!("{}{}\n", output, op);
    }
    let n = output.len() as i32;
    let be_bytes = n.to_be_bytes();
    eprintln!("[{}]\n{}", n, output);
    match io::stderr().flush() {
        Ok(()) => {}
        _ => {panic!("Failed to write to stderr?")}
    };
    io::stdout().write_all(&be_bytes).unwrap();
    print!("{}",output);
    match io::stdout().flush() {
        Ok(()) => {}
        _ => {panic!("Failed to write to stdout?")}
    };
}

pub fn apply_op(old_gs: &GameState, op: Op) -> GameState {
    let mut gs = old_gs.clone();
    match op {
        Op::March(pos, dir, num) => {
            let dp = dir.to_delta_pos();
            let dst = Position {x:pos.x+dp.x, y:pos.y+dp.y};
            gs.march(pos, dst, num);
        },
        Op::Shift(id, pos) => {
            gs.shift(id, pos);
        }
        Op::Promote(id, t) => {
            gs.promote(id, t);
        }
        Op::Skill(id, t, maybe_pos) => {
            match t {
                SkillType::Dash => {
                    gs.dash(id, maybe_pos.unwrap());
                }
                SkillType::Kill => {
                    gs.kill(id, maybe_pos.unwrap());
                }
                SkillType::Atk | SkillType::Def | SkillType::Magic => {
                    gs.buff(id, t);
                }
            };
        }
        Op::Tech(tt) => {
            gs.tech_advancement(tt);
        }
        Op::SuperWeapon(t, dst_pos, maybe_src_pos) => {
            match t {
                SWType::Nuclear => gs.nuclear(dst_pos),
                SWType::Boost => gs.boost(dst_pos),
                SWType::Teleport => {
                    let src_pos = maybe_src_pos.unwrap();
                    gs.teleport(dst_pos, src_pos);
                }
                SWType::Freeze => gs.freeze(dst_pos),
                SWType::Pending => panic!(),
            }
        }
        Op::Call(pos) => {
            gs.call_general(pos);
        }
        Op::End => {
            gs.flip();
        }
    }
    return gs;
}