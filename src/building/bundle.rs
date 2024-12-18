use bevy::prelude::*;

use super::{
    components::{Building, BuildingTemplateMarker, BuildingType, CanBuild, CoveringTiles},
    BuildableColor,
};

#[derive(Bundle)]
pub struct BuildingBundle {
    pub building: Building,
    pub building_type: BuildingType,
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
            sprite: SpriteBundle {
                texture,
                transform,
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
    sprite: SpriteBundle,
}

impl BuildingMarkerBundle {
    pub fn build_marker(marker_type: BuildingType, sprite_texture: Handle<Image>) -> Self {
        BuildingMarkerBundle {
            marker: BuildingTemplateMarker,
            marker_type,
            can_build: CanBuild(false),
            covering_tiles: CoveringTiles(vec![]),
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
            asset_server.load(BuildingType::Theatre.sprite()).clone(),
        )
    }
}
