use std::vec;

use bevy::{math::Vec2, prelude::*};

enum FarmLandState {
    CROP_SELECTION,
    LAND_PREP,
    SEED_SELECTION,
    SEED_PLANTING,
    IRRIGATION,
    CROP_GROWTH,
    FERTILIZING,
    HARVESTING,
}
enum FarmLandType {
    Pot,
    Field,
}

#[derive(Component)]
struct FarmLand {
    landState: FarmLandState,
    landType: FarmLandType,
    quality: i32,
}
