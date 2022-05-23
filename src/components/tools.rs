use bevy::prelude::*;


enum VehicleType {
    Car,
    Truck,
    Tractor,
}

enum AttachmentType {
    Sowing,
    Harrowing,
    Agitating,
    Spreading,
    Baling,
    Trailer,    
}


enum AttachmentPoint {
    FRONT,
    MID,
    REAR,
}

#[derive(Component)]
struct Vehicle(VehicleType);

#[derive(Component)]
struct TractorAttatchment {
    attachmentPoint: AttachmentPoint,
    attachmentType: AttachmentType,
}

#[derive(Component)] 
struct DrivingInfo {
    steeringAngle: f32,
    direction: f32,
    speed: f32,
}