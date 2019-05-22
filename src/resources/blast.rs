use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};


#[derive(Clone)]
pub struct BlastResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_number: usize,
}

