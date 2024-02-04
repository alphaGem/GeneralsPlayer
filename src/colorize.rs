use crate::*;
use crate::map;

pub trait Colorize {
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn orange(&self) -> String;
    fn blue(&self) -> String;
    fn purple(&self) -> String;
    fn cyan(&self) -> String;
    fn lightgrey(&self) -> String;

    fn darkgrey(&self) -> String;
    fn lightred(&self) -> String;
    fn lightgreen(&self) -> String;
    fn yellow(&self) -> String;
    fn lightblue(&self) -> String;
    fn pink(&self) -> String;
    fn lightcyan(&self) -> String;
    fn white(&self) -> String;

    fn black_bg(&self) -> String;
    fn red_bg(&self) -> String;
    fn green_bg(&self) -> String;
    fn orange_bg(&self) -> String;
    fn blue_bg(&self) -> String;
    fn purple_bg(&self) -> String;
    fn cyan_bg(&self) -> String;
    fn lightgrey_bg(&self) -> String;

    fn plain_bg(&self) -> String;
    fn sand_bg(&self) -> String;
    fn swamp_bg(&self) -> String;
    fn border_bg(&self) -> String;
    fn neutral_fg(&self) -> String;
    fn hostile_fg(&self) -> String;
    fn friendly_fg(&self) -> String;

    fn bold(&self) -> String;
    fn underline(&self) -> String;
}

impl Colorize for str {
    fn black(&self) -> String  {return String::from("\x1b[30m")+self+"\x1b[0m";}
    fn red(&self) -> String  {return String::from("\x1b[31m")+self+"\x1b[0m";}
    fn green(&self) -> String  {return String::from("\x1b[32m")+self+"\x1b[0m";}
    fn orange(&self) -> String  {return String::from("\x1b[33m")+self+"\x1b[0m";}
    fn blue(&self) -> String  {return String::from("\x1b[34m")+self+"\x1b[0m";}
    fn purple(&self) -> String  {return String::from("\x1b[35m")+self+"\x1b[0m";}
    fn cyan(&self) -> String  {return String::from("\x1b[36m")+self+"\x1b[0m";}
    fn lightgrey(&self) -> String  {return String::from("\x1b[37m")+self+"\x1b[0m";}

    fn darkgrey(&self) -> String  {return String::from("\x1b[90m")+self+"\x1b[0m";}
    fn lightred(&self) -> String  {return String::from("\x1b[91m")+self+"\x1b[0m";}
    fn lightgreen(&self) -> String  {return String::from("\x1b[92m")+self+"\x1b[0m";}
    fn yellow(&self) -> String  {return String::from("\x1b[93m")+self+"\x1b[0m";}
    fn lightblue(&self) -> String  {return String::from("\x1b[94m")+self+"\x1b[0m";}
    fn pink(&self) -> String  {return String::from("\x1b[95m")+self+"\x1b[0m";}
    fn lightcyan(&self) -> String  {return String::from("\x1b[96m")+self+"\x1b[0m";}
    fn white(&self) -> String  {return String::from("\x1b[97m")+self+"\x1b[0m";}

    fn black_bg(&self) -> String  {return String::from("\x1b[40m")+self+"\x1b[0m";}
    fn red_bg(&self) -> String  {return String::from("\x1b[41m")+self+"\x1b[0m";}
    fn green_bg(&self) -> String  {return String::from("\x1b[42m")+self+"\x1b[0m";}
    fn orange_bg(&self) -> String  {return String::from("\x1b[43m")+self+"\x1b[0m";}
    fn blue_bg(&self) -> String  {return String::from("\x1b[44m")+self+"\x1b[0m";}
    fn purple_bg(&self) -> String  {return String::from("\x1b[45m")+self+"\x1b[0m";}
    fn cyan_bg(&self) -> String  {return String::from("\x1b[46m")+self+"\x1b[0m";}
    fn lightgrey_bg(&self) -> String  {return String::from("\x1b[47m")+self+"\x1b[0m";}
    
    fn plain_bg(&self) -> String {return String::from("\x1b[48;5;255m")+self+"\x1b[0m";}
    fn sand_bg(&self) -> String {return String::from("\x1b[48;5;230m")+self+"\x1b[0m";}
    fn swamp_bg(&self) -> String {return String::from("\x1b[48;5;195m")+self+"\x1b[0m";}
    fn border_bg(&self) -> String {return String::from("\x1b[48;5;231m")+self+"\x1b[0m";}

    fn neutral_fg(&self) -> String {return String::from("\x1b[38;5;232m")+self+"\x1b[0m";}
    fn hostile_fg(&self) -> String {return String::from("\x1b[38;5;196m")+self+"\x1b[0m";}
    fn friendly_fg(&self) -> String {return String::from("\x1b[38;5;46m")+self+"\x1b[0m";}

    fn bold(&self) -> String {return String::from("\x1b[1m")+self+"\x1b[0m";}
    fn underline(&self) -> String {return String::from("\x1b[4m")+self+"\x1b[0m";}
}

impl GameState {
    pub fn print(&self) {
        let title = format!("Round {}", self.turn).bold().underline();
        let osw;
        if let Some(sw) = &self.our.sw {
            osw = format!("{}", sw);
        }
        else {
            osw = "Nothing".to_string();
        }
        let oc = format!(
            "Our Coin {} | Tech motor={} Raft={} Track={} Rela.={} | SW = {}",
            self.our.coin,
            self.our.tech_tree.motor,
            self.our.tech_tree.raft,
            self.our.tech_tree.track,
            self.our.tech_tree.relativity,
            osw
        ).friendly_fg();
        let tsw;
        if let Some(sw) = &self.their.sw {
            tsw = format!("{}", sw);
        }
        else {
            tsw = "Nothing".to_string();
        }
        let tc = format!(
            "Their Coin {} | Tech motor={} Raft={} Track={} Rela.={} | SW = {}",
            self.their.coin,
            self.their.tech_tree.motor,
            self.their.tech_tree.raft,
            self.their.tech_tree.track,
            self.their.tech_tree.relativity,
            tsw
        ).hostile_fg();
        eprintln!("{}", title);
        eprintln!("{}", oc);
        eprintln!("{}", tc);
        eprintln!("{}", "  │ 00│ 01│ 02│ 03│ 04│ 05│ 06│ 07│ 08│ 09│ 10│ 11│ 12│ 13│ 14".neutral_fg().border_bg());
        eprintln!("{}", "──╋━━━┿━━━┿━━━┿━━━┿━━━╋━━━┿━━━┿━━━┿━━━┿━━━╋━━━┿━━━┿━━━┿━━━┿━━━╋".neutral_fg().border_bg());
        for i in 0..15 {
            eprint!("{}",format!("{:02}",i).neutral_fg().border_bg());
            for j in 0..15 {
                const GENERAL_ID:[&str;32] = 
                  ["Ø","ł","A","B","C","D","!","@","#","$","%","^","&","*",":",";","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T"];
                //  0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20
                let troop = self.troop[i][j];
                let troop_str;
                let general_str;
                if troop >= 100 {
                    troop_str = format!("{}",troop);
                }
                else if troop > 0 || self.cell[i][j] != general::NOTHING || self.owner[i][j] != Attitude::Neutral{
                    troop_str = format!("{:02}", troop);
                }
                else {
                    troop_str = "  ".to_string();
                }

                if self.cell[i][j] == general::NOTHING {
                    general_str = " ";
                }
                else if self.cell[i][j].0 > 30 {
                    general_str = "?";
                }
                else {
                    general_str = GENERAL_ID[self.cell[i][j].0 as usize];
                }
                let mut info = format!("{}{}", general_str, troop_str);
                match self.owner[i][j] {
                    Attitude::Friendly => info = info.friendly_fg(),
                    Attitude::Hostile => info = info.hostile_fg(),
                    Attitude::Neutral => info = info.neutral_fg(),
                }
                // usage of MAP[i][j], a mut static recording the terrain
                // it is actually safe because we guarantee that we only write into MAP once before all usage
                unsafe { 
                    match map::MAP[i][j] {
                        Terrain::Plain => info = info.plain_bg(),
                        Terrain::Sand => info = info.sand_bg(),
                        Terrain::Swamp => info = info.swamp_bg()
                    }
                }
                if self.cell[i][j]!=general::NOTHING {
                    match self.generals[self.cell[i][j].0 as usize].general_type {
                        GeneralType::Main|GeneralType::Sub => info = info.bold(),
                        GeneralType::Mine => info = info.bold().underline(),
                    }
                }
                if j%5==0 {
                    eprint!("{}{}", "┃".neutral_fg().border_bg(), info);
                }
                else {
                    eprint!("{}{}", "│".neutral_fg().border_bg(), info);
                }
            }
            eprintln!("{}", "┃".neutral_fg().border_bg());
            if i%5==4 {
                eprintln!("{}", "──╋━━━┿━━━┿━━━┿━━━┿━━━╋━━━┿━━━┿━━━┿━━━┿━━━╋━━━┿━━━┿━━━┿━━━┿━━━╋".neutral_fg().border_bg());
            }
            else {
                eprintln!("{}", "──╂───┼───┼───┼───┼───╂───┼───┼───┼───┼───╂───┼───┼───┼───┼───╂".neutral_fg().border_bg());
            }
        }
    }
}