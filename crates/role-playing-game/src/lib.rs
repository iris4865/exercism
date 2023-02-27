use std::cmp::min;

#[cfg(test)]
mod tests;

const MAX_HEALTH: u32 = 100;
const MAX_MANA: u32 = 100;
const MANA_LEVEL_THRESHOLD: u32 = 9;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        let mana = match level {
            0..=MANA_LEVEL_THRESHOLD => None,
            _ => Some(MAX_MANA),
        };

        Self {
            health: MAX_HEALTH,
            mana,
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health = self.health - min(self.health, mana_cost);
                0
            }
        }
    }
}
