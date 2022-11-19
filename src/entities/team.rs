//!
//! A team
//! HanishKVC, 2022
//!

use sdl2::pixels::Color;
use sdl2::ttf::Font;

use loggerk::{ldebug, log_d};

use crate::entities::{self, ENTITY_WIDTH, ENTITY_HEIGHT};
use crate::entities::gentity::GEntity;
use crate::sdlx::{SdlX, self};
use crate::playdata::{PlayerCodedData, self};



#[derive(Debug)]
pub struct Team<'a> {
    name: String,
    color: Color,
    players: Vec<GEntity<'a>>,
    pmoves: Vec<(f32,f32)>,
    pchgmovs: Vec<i32>,
}

impl<'a> Team<'a> {

    pub fn new(name: &str, color: Color, nplayers: i32, font: &'a Font) -> Team<'a> {
        let mut team = Team {
            name: name.to_string(),
            color: color,
            players: Vec::new(),
            pmoves: Vec::new(),
            pchgmovs: Vec::new(),
        };
        let bx = (rand::random::<u32>() % entities::SCREEN_WIDTH) as f32;
        for i in 0..nplayers {
            let fx = (rand::random::<u32>() % (entities::SCREEN_WIDTH/4)) as f32;
            let fy = (rand::random::<u32>() % entities::SCREEN_HEIGHT) as f32;
            team.players.push(GEntity::new(i.to_string().as_str(), (bx+fx, fy), (ENTITY_WIDTH, ENTITY_HEIGHT), team.color, font));
            team.pmoves.push((0.0,0.0));
            let mut chgmov = fx.round() as i32;
            if chgmov == 0 {
                chgmov = 1;
            }
            team.pchgmovs.push(chgmov);
        }
        print!("INFO:PGND:Team:Created:{}:{:#?}\n", team.name, team);
        team
    }

    pub fn update(&mut self, playersdata: Vec<PlayerCodedData>, babsolute: bool, inframes: f32) {
        for player in playersdata {
            ldebug!(&format!("DBUG:PPGND:Team:{}:{:?}", self.name, player));
            let pi = player.0 as usize;
            for pd in player.1 {
                match pd {
                    playdata::PlayerData::Pos(fx, fy) => {
                        // Position
                        if babsolute {
                            self.players[pi].pos_set_abs(fx, fy);
                        } else {
                            self.players[pi].move_to_in_frames((fx, fy), inframes);
                        }
                    },
                    playdata::PlayerData::Stamina(fstamina) => {
                        // Stamina
                        //self.players[ppos.0 as usize].set_fcolor(1.0-fstamina, 1.0);
                        let istamina = (fstamina * 100.0).round() as i32;
                        let stamina_color = match istamina {
                            0..=30 => Color::RED,
                            31..=70 => Color::YELLOW,
                            71..=100 => Color::GREEN,
                            _ => todo!(),
                        };
                        //self.players[ppos.0 as usize].set_nxarc(0.8, fstamina, color);
                        self.players[pi].set_ll_color(stamina_color);
                        self.players[pi].set_rl_color(stamina_color);
                    },
                    playdata::PlayerData::Card(card) => {
                        // Cards
                        let mut card_color = sdlx::COLOR_INVISIBLE;
                        if let playdata::Cards::Red = card {
                            card_color = Color::RED;
                        } else if let playdata::Cards::Yellow = card {
                            card_color = Color::YELLOW;
                        }
                        self.players[pi].set_tl_color(card_color);
                        self.players[pi].set_bl_color(card_color);
                    },
                }
            }
        }
    }

    pub fn next_frame(&mut self) {
        for i in 0..self.players.len() {
            self.players[i].next_frame();
        }
    }

    pub fn draw(&self, sx: &mut SdlX) {
        for i in 0..self.players.len() {
            self.players[i].draw(sx);
        }
    }

}

impl<'a> Team<'a> {

    pub fn adjust_players(&mut self, colorsel: u8) {
        for i in 0..self.players.len() {
            self.players[i].colorsel = colorsel;
        }
    }

}
