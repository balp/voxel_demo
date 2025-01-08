mod map;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::Arc;
use bevy::pbr::CascadeShadowConfigBuilder;
use map::{Map, NodeType};

// Using enum for material index allows for more than u8::MAX number of materials.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
enum BlockTexture {
    #[default]
    SnowyBrick,
    FullBrick,
    Grass,
}

#[derive(Resource, Clone)]
struct MyMainWorld {
//    map: Box<Map>,
    static_map: &'static Map,
}

impl Default for MyMainWorld {
    fn default() -> Self {
        warn!("MyMainWorld::default() called");
        let map = Box::new(Map::test_map());
        let static_map: &'static Map = Box::leak(Box::new(map.clone()));
        Self {
            static_map,
        }
    }
}

impl MyMainWorld {
    fn new() -> Self {
        warn!("MyMainWorld::new() called");
        let map = Box::new(Map::test_map());
        let static_map: &'static Map = Box::leak(Box::new(map.clone()));
        Self {
            static_map,
        }
    }
}

impl VoxelWorldConfig for MyMainWorld {
    type MaterialIndex = BlockTexture;

    // 00 - brick_grey.png
    // 01 - brick_red.png
    // 02 - cactus_inside.png
    // 03 - cactus_side.png
    // 04 - cactus_top.png
    // 05 - cotton_blue.png
    // 06 - cotton_green.png
    // 07 - cotton_red.png
    // 08 - cotton_tan.png
    // 09 - dirt.png
    // 10 - dirt_grass.png
    // 11 - dirt_sand.png
    // 12 - dirt_snow.png
    // 13 - fence_stone.png
    // 14 - fence_wood.png
    // 15 - glass.png
    // 16 - glass_frame.png
    // 17 - grass1.png
    // 18 - grass2.png
    // 19 - grass3.png
    // 20 - grass4.png
    // 21 - grass_brown.png
    // 22 - grass_tan.png
    // 23 - grass_top.png
    // 24 - gravel_dirt.png
    // 25 - gravel_stone.png
    // 26 - greysand.png
    // 27 - greystone.png
    // 28 - greystone_ruby.png
    // 29 - greystone_ruby_alt.png
    // 30 - greystone_sand.png
    // 31 - ice.png
    // 32 - lava.png
    // 33 - leaves.png
    // 34 - leaves_orange.png
    // 35 - leaves_orange_transparent.png
    // 36 - leaves_transparent.png
    // 37 - mushroom_brown.png
    // 38 - mushroom_red.png
    // 39 - mushroom_tan.png
    // 40 - oven.png
    // 41 - redsand.png
    // 42 - redstone.png
    // 43 - redstone_emerald.png
    // 44 - redstone_emerald_alt.png
    // 45 - redstone_sand.png
    // 46 - rock.png
    // 47 - rock_moss.png
    // 48 - sand.png
    // 49 - snow.png
    // 50 - stone.png
    // 51 - stone_browniron.png
    // 52 - stone_browniron_alt.png
    // 53 - stone_coal.png
    // 54 - stone_coal_alt.png
    // 55 - stone_diamond.png
    // 56 - stone_diamond_alt.png
    // 57 - stone_dirt.png
    // 58 - stone_gold.png
    // 59 - stone_gold_alt.png
    // 60 - stone_grass.png
    // 61 - stone_iron.png
    // 62 - stone_iron_alt.png
    // 63 - stone_sand.png
    // 64 - stone_silver.png
    // 65 - stone_silver_alt.png
    // 66 - stone_snow.png
    // 67 - table.png
    // 68 - track_corner.png
    // 69 - track_corner_alt.png
    // 70 - track_straight.png
    // 71 - track_straight_alt.png
    // 72 - trunk_bottom.png
    // 73 - trunk_mid.png
    // 74 - trunk_side.png
    // 75 - trunk_top.png
    // 76 - trunk_white_side.png
    // 77 - trunk_white_top.png
    // 78 - water.png
    // 79 - wheat_stage1.png
    // 80 - wheat_stage2.png
    // 81 - wheat_stage3.png
    // 82 - wheat_stage4.png
    // 83 - wood.png
    // 84 - wood_red.png
    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat| match vox_mat {
            BlockTexture::SnowyBrick => [49, 49, 49],
            BlockTexture::FullBrick => [9, 9, 9],
            BlockTexture::Grass => [23, 23, 23],
        })
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let static_ref: &'static Map = self.static_map;
        Box::new(move |_chunk_pos| get_voxel_fn(static_ref))
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("voxel_textures_all.png".into(), 85))
    }
}
fn get_voxel_fn(world_map: &Map) -> Box<dyn FnMut(IVec3) -> WorldVoxel<BlockTexture> + Send + Sync + '_> {
    Box::new(move |pos: IVec3| {
        let node = world_map.get(pos);
        match node {
            None => WorldVoxel::Unset,
            Some(n) => {
                if pos.y < (n.height as i32) {
                    WorldVoxel::Solid(BlockTexture::FullBrick)
                } else if pos.y == (n.height as i32) {
                    match n._type {
                        NodeType::Grass => {WorldVoxel::Solid(BlockTexture::Grass)}
                        NodeType::Snow => {WorldVoxel::Solid(BlockTexture::SnowyBrick)}
                    }

                } else {
                    WorldVoxel::Air
                }
            },
        }
    })
}


fn main() {
    assert_eq!(size_of::<WorldVoxel>(), 2);
    assert_eq!(size_of::<WorldVoxel<BlockTexture>>(), 1);

    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(LogPlugin {
        //     filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".into(),
        //     level: bevy::log::Level::DEBUG,
        //     custom_layer: |_| None,
        // }))
        .add_plugins(VoxelWorldPlugin::with_config(MyMainWorld::new()))
        .add_systems(Startup, (setup, ).chain())
        .add_systems(Update, (move_camera, close_on_esc))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        // This tells bevy_voxel_world to use this cameras transform to calculate spawning area
        VoxelWorldCamera::<MyMainWorld>::default(),
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .build(),
    ));

}

fn move_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<VoxelWorldCamera<MyMainWorld>>>,
) {
    let mut transform = query.single_mut();
    let time_seconds = time.elapsed_secs();
    let rotation = 0.2;
    transform.translation.x = 25.0 * (time_seconds * rotation).sin();
    transform.translation.z = 25.0 * (time_seconds * rotation).cos();
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

