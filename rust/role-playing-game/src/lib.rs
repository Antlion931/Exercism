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
        (self.health == 0).then_some(
            Player{
                health: 100,
                mana: (self.level >= 10).then_some(100), 
                level: self.level
            })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) =>  if let Some(new_mana) = mana.checked_sub(mana_cost) {
                   self.mana = Some(new_mana);
                   return mana_cost * 2;
                }
            None => self.health = self.health.saturating_sub(mana_cost), 
        }
        0
    }
}
