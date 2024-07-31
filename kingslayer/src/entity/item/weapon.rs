use serde::{Deserialize, Serialize};

use crate::{dice_roll, entity::Entity};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    name: String,
    desc: String,
    inspect: String,
    damage: u32,
}

impl Weapon {
    pub fn new(name: &str, inspect: &str, damage: u32) -> Self {
        Self {
            name: name.to_owned(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_owned(),
            damage,
        }
    }

    pub fn damage(&self) -> u32 {
        dice_roll(1, self.damage)
    }
}

impl Entity for Weapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}
