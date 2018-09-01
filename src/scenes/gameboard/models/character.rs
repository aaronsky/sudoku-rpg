use super::ability::Ability;
use common::resources;
use ggez::Context;
use warmy;
use world::World;

pub struct Character {
    pub name: String,
    pub image: warmy::Res<resources::Image>,
    pub abilities: Vec<Ability>,
}

impl Character {
    pub fn new(name: &str, image_asset: &str, ctx: &mut Context, world: &mut World) -> Self {
        Character {
            name: name.to_string(),
            image: world
                .assets
                .get::<_, resources::Image>(&warmy::FSKey::new(format!("/images/characters/{}", image_asset)), ctx)
                .unwrap(),
            abilities: vec![],
        }
    }
}
