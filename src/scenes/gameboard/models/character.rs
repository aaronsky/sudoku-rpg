use super::ability::Ability;
use common::resources;
use ggez::Context;
use warmy;
use world::World;

#[derive(Debug, Copy, Clone)]
pub enum CharacterKind {
    Character,
    Opponent,
}

pub struct Character {
    pub name: String,
    pub kind: CharacterKind,
    pub image: warmy::Res<resources::Image>,
    pub abilities: Vec<Ability>,
}

impl Character {
    pub fn new(
        name: &str,
        kind: CharacterKind,
        image_asset: &str,
        ctx: &mut Context,
        world: &mut World,
    ) -> Self {
        Character {
            name: name.to_string(),
            kind,
            image: world
                .assets
                .get::<_, resources::Image>(
                    &warmy::FSKey::new(format!("/images/characters/{}", image_asset)),
                    ctx,
                ).unwrap(),
            abilities: vec![],
        }
    }

    pub fn add_ability(mut self, ability: Ability) -> Self {
        self.abilities.push(ability);
        self
    }
}
