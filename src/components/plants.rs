use bevy::prelude::*;

#[derive(Component)]
enum CropCategory {
    Herb,
    Vegetable,
    Fruit,
    Grain,
    Cosmetic,
}

#[derive(Component)]
enum SoilQuality {
    POOR,
    DECENT,
    BETTER,
    GOOD,
    GREAT,
    HEALTHY,
}

#[derive(Component)]
struct LandShape {
    verticies: Vec<Vec2>,
}

#[derive(Component)]
enum GrowState {
    Seed,
    Underground,
    Sprout,
    Vegative,
    Ripening,
    Ripe,
}

#[derive(Component)]
struct Sunlight(u32);

#[derive(Component)]
struct CropName(String);

#[derive(Component)]
struct PlantedInfo {
    timePlanted: f32,
    growTime: f32,

    // This planted information can be use to make a
    // compantion growth system or something like that.
    nearbyPlants: Vec<CropName>,
    expectedYield: i32,
}
