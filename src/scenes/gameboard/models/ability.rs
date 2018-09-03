use common::resources;
use ggez::Context;
use warmy;
use world::World;

pub struct Ability {
    pub name: String,
    pub usable: bool,
    pub icon: warmy::Res<resources::Image>,
}

impl Ability {
    pub fn new(name: &str, icon_asset: &str, ctx: &mut Context, world: &mut World) -> Self {
        Ability {
            name: name.to_string(),
            usable: true,
            icon: world
                .assets
                .get::<_, resources::Image>(
                    &warmy::FSKey::new(format!("/images/badges/{}", icon_asset)),
                    ctx,
                ).unwrap(),
        }
    }

    // pub fn set_usable(mut self, value: bool) -> Self {
    //     self.usable = value;
    //     self
    // }
}
