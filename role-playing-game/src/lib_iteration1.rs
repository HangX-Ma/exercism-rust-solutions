pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        if self.health > 0 {
            return None;
        }
        let mut new = Player { 
            health: 100, 
            mana: None, 
            level: self.level 
        };
        if new.level >= 10 {
            new.mana = Some(100); 
        }
        Some(new)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {mana_cost}")
        if self.mana == None {
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            return 0;
        } else {
            if Some(mana_cost) > self.mana {
                return 0;
            }
            if let Some(mut mana_remain) = self.mana {
                mana_remain -= mana_cost;
                self.mana = Some(mana_remain);
            }
            return 2 * mana_cost;
        }
    }
}
