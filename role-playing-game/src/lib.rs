use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        match self.health {
            0 => Some(Player {
                health: 100, 
                mana:  if self.level < 10 {None} else {Some(100)}, 
                level: self.level
            }), 
            _ => None, 
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {mana_cost}")
        match self.mana {
            None => {
                self.health -= min(self.health, mana_cost);
                0
            },
            Some(mana) => {
                if mana_cost < mana {
                    self.mana = Some(mana - mana_cost);
                    return 2 * mana_cost;
                } else {
                    0
                }
            }
        }
    }
}
