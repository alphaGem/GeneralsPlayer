use crate::*;

fn get_map_distance(pos1: Position, pos2: Position, our: &Side) -> i8 {
    let distance;
    if our.tech_tree.raft == 1 {
        distance = utils::manhattan_distance(pos1, pos2);
    }
    else {
        unsafe {
            distance = map::DIST[pos1.x as usize][pos1.y as usize][pos2.x as usize][pos2.y as usize];
        }
    }
    return distance;
}

fn get_next_move(gs: &GameState) -> Op {
    if gs.get_main(Attitude::Friendly) == None {
        eprintln!("Game Over!");
        return Op::End;
    }
    if gs.get_main(Attitude::Hostile) == None {
        eprintln!("Game Over!");
        return Op::End;
    }
    // Try Prod Upgrade
    for g in &gs.generals {
        if gs.check_promote(g.id, AttrType::Prod) {
            return Op::Promote(g.id, AttrType::Prod);
        }
    }
    for g in &gs.generals {
        if gs.check_promote(g.id, AttrType::Spd) {
            return Op::Promote(g.id, AttrType::Spd);
        }
    }
    // Try Tech Upgrade
    for tt in vec![TechType::Raft, TechType::Motor, TechType::Track, TechType::Relativity] {

        if gs.check_tech_advancement(tt) {
            return Op::Tech(tt);
        }
    }
    // Try Def Upgrade
    if gs.our.tech_tree.raft > 0
    {
        for g in &gs.generals {
            if g.attr.def >= g.id.0 as i8 % 3 {
                continue;
            }
            if gs.check_promote(g.id, AttrType::Prod) {
                return Op::Promote(g.id, AttrType::Prod);
            }
        }
    }
    let mut major_pos = Position{x:0, y:0};
    for i in 0..15 {
        for j in 0..15 {
            if gs.owner[i][j]==Attitude::Friendly {
                if gs.troop[i][j] > gs.troop[major_pos.x as usize][major_pos.y as usize] {
                    major_pos = Position{x: i as i8, y: j as i8};
                }
            }
        }
    }
    // Try move general
    let g = gs.get_main(Attitude::Friendly).unwrap();
    if g.pos != major_pos {
        if gs.check_shift(g.id, major_pos) {
            return Op::Shift(g.id, major_pos);
        }
        let mut best_d = 255;
        let mut best_dist = 127;
        let mut best_np = g.pos;
        for d in vec![1,2,3,4] {
            let dp = Direction(d).to_delta_pos();
            let np = Position{x:g.pos.x+dp.x, y:g.pos.y+dp.y};
            if 0<=np.x && np.x<=14 && 0<=np.y && np.y <= 14 {
                if gs.check_shift(g.id, np) && get_map_distance(major_pos, np, &gs.our)<best_dist {
                    best_dist = get_map_distance(major_pos, np, &gs.our);
                    best_d = d;
                    best_np = np;
                }
            }
        }
        if best_d!=255 && gs.check_shift(g.id, best_np) {
            return Op::Shift(g.id, best_np);
        }
    }
    // Try move troop
    let mut best_d = 255;
    let mut best_dist = 31;
    let mut best_np = major_pos;
    for d in vec![1,2,3,4] {
        let dp = Direction(d).to_delta_pos();
        let np = Position{x:major_pos.x+dp.x, y:major_pos.y+dp.y};
        if 0<=np.x && np.x<=14 && 0<=np.y && np.y <= 14 {
            for g in &gs.generals {
                if g.general_type == GeneralType::Mine && g.attitude == Attitude::Neutral {
                    if get_map_distance(g.pos, np, &gs.our)<best_dist {
                        best_dist = get_map_distance(g.pos, np, &gs.our);
                        best_d = d;
                        best_np = np;
                    }
                }
            }
        }
    }
    // eprintln!(">> {} {}", best_d, best_np);
    let num = gs.troop[major_pos.x as usize][major_pos.y as usize]-1;
    if best_d != 255 && gs.check_march(major_pos, best_np, num) {
        return Op::March(major_pos, Direction(best_d), num);
    }
    return Op::End;
}

pub fn sample_ai(init_gs: GameState) -> GameState {
    let mut gs = init_gs.clone();
    let mut ops = vec![];
    loop {
        let op = get_next_move(&gs);
        gs = operation::apply_op(&gs, op);
        ops.push(op);
        match op {
            Op::End => {
                operation::send_op(ops);
                return gs;
            }
            _ => {}
        };
    }
}