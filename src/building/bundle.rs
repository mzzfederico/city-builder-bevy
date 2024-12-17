use bevy::prelude::*;

use super::{
    components::{
        Building, BuildingSize, BuildingTemplateMarker, BuildingType, CanBuild, CoveringTiles,
    },
    BuildableColor,
};

#[derive(Bundle)]
pub struct BuildingBundle {
    pub building: Building,
    pub building_type: BuildingType,
    pub building_size: BuildingSize,
    pub sprite: SpriteBundle,
}

impl BuildingBundle {
    pub fn build(
        building_type: BuildingType,
        position: Vec3,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let transform = Transform::from_xyz(position.x, position.y, position.z);
        let texture = asset_server.load(building_type.sprite());

        Self {
            building: Building,
            building_type,
            building_size: BuildingSize(building_type.size()),
            sprite: SpriteBundle {
                texture,
                transform: Transform::from(transform),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct BuildingMarkerBundle {
    marker: BuildingTemplateMarker,
    can_build: CanBuild,
    marker_type: BuildingType,
    covering_tiles: CoveringTiles,
    building_size: BuildingSize,
    sprite: SpriteBundle,
}

impl BuildingMarkerBundle {
    pub fn build_marker(
        marker_type: BuildingType,
        size: BuildingSize,
        sprite_texture: Handle<Image>,
    ) -> Self {
        BuildingMarkerBundle {
            marker: BuildingTemplateMarker,
            marker_type,
            can_build: CanBuild(false),
            covering_tiles: CoveringTiles(vec![]),
            building_size: size,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: BuildableColor::default().into(),
                    ..default()
                },
                texture: sprite_texture,
                transform: Transform::from_xyz(100000000., 100000000., 2.),
                ..Default::default()
            },
        }
    }
    pub fn theatre(asset_server: Res<AssetServer>) -> Self {
        Self::build_marker(
            BuildingType::Theatre,
            BuildingSize(BuildingType::Theatre.size()),
            asset_server.load(BuildingType::Theatre.sprite()).clone(),
        )
    }
}
