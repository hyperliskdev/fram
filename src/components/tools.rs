use bevy::prelude::*;


enum VehicleType {
    Car,
    Truck,
    Tractor,
}

enum AttachmentType {


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
