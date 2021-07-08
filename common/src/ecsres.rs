use specs::Entity;

#[derive(Default, Copy, Clone)]
pub struct DeltaTime(pub f64);

#[derive(Default, Clone)]
pub struct Player(pub Option<Entity>);
