use common::resources;
use ggez::Context;
use warmy;
use world::World;

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub status: AbilityStatus,
    pub icon: warmy::Res<resources::Image>,
}

#[derive(Debug, Copy, Clone)]
pub enum AbilityStatus {
    Active,
    Inactive,
    InStock,
}

impl Ability {
    pub fn new(name: &str, icon_asset: &str, ctx: &mut Context, world: &mut World) -> Self {
        let status = AbilityStatus::InStock;
        Ability {
            name: name.to_string(),
            status,
            icon: world
                .assets
                .get::<_, resources::Image>(
                    &warmy::FSKey::new(ability_asset_path(icon_asset, &status)),
                    ctx,
                ).unwrap(),
        }
    }
}

fn ability_asset_path(name: &str, status: &AbilityStatus) -> String {
    let suffix = match status {
        AbilityStatus::Active => "Activated",
        AbilityStatus::Inactive => "Inactive",
        AbilityStatus::InStock => "InStock",
    };
    format!("/images/badges/{}_{}.png", name, suffix)
}
