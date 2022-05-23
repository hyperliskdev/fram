use std::vec;

use bevy::{math::Vec2, prelude::*};


enum LandState {
    CROP_SELECTION,
    LAND_PREP,
    SEED_SELECTION,
    SEED_PLANTING,
    IRRIGATION,
    CROP_GROWTH,
    FERTILIZING,
    HARVESTING,
}
enum LandType {
    Pot,
    Field,    
}

#[derive(Component)]
struct Land {
    landState: LandState,
    landType: LandType,
    quality: i32,
}


