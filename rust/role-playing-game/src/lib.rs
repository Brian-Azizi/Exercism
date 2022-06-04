// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: match self.level < 10 {
                    true => None,
                    false => Some(100),
                },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => match mana >= mana_cost {
                true => {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                }
                false => 0,
            },
            None => {
                self.health = match mana_cost > self.health {
                    true => 0,
                    false => self.health - mana_cost,
                };
                0
            }
        }
    }
}
