mod map;

use crate::map::Size;
use bevy::pbr::{CascadeShadowConfigBuilder, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderDefVal, ShaderRef, SpecializedMeshPipelineError,
};
use bevy_voxel_world::prelude::*;
use bevy_voxel_world::rendering::{vertex_layout, VOXEL_TEXTURE_SHADER_HANDLE};
use map::{Map, NodeType};
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};
use std::f32::consts::PI;
use std::sync::Arc;
// Using enum for material index allows for more than u8::MAX number of materials.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
enum BlockTexture {
    #[default]
    SnowyBrick,
    FullBrick,
    GrassBrick,
}

#[derive(Resource, Clone)]
struct MyMainWorld {
    //    map: Box<Map>,
    static_map: &'static Map,
}

impl Default for MyMainWorld {
    fn default() -> Self {
        warn!("MyMainWorld::default() called");
        let map = Box::new(Map::noise_map(Size::new(20, 20)));
        let static_map: &'static Map = Box::leak(Box::new(map.clone()));
        Self { static_map }
    }
}

impl MyMainWorld {
    fn new() -> Self {
        warn!("MyMainWorld::new() called");
        let map = Box::new(Map::noise_map(Size::new(200, 200)));
        let static_map: &'static Map = Box::leak(Box::new(map.clone()));
        Self { static_map }
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
            BlockTexture::GrassBrick => [23, 23, 23],
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
fn get_voxel_fn(
    world_map: &Map,
) -> Box<dyn FnMut(IVec3) -> WorldVoxel<BlockTexture> + Send + Sync + '_> {
    Box::new(move |pos: IVec3| {
        let node = world_map.get(pos);
        match node {
            None => WorldVoxel::Unset,
            Some(n) => {
                if pos.y < (n.height as i32) {
                    WorldVoxel::Solid(BlockTexture::FullBrick)
                } else if pos.y == (n.height as i32) {
                    match n._type {
                        NodeType::Grass => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Snow => WorldVoxel::Solid(BlockTexture::SnowyBrick),
                        NodeType::Dirt => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Sand => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Gravel => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Stone => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Rock => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Water => WorldVoxel::Solid(BlockTexture::GrassBrick),
                    }
                } else {
                    WorldVoxel::Air
                }
            }
        }
    })
}

const RED: u8 = 0;
const GREEN: u8 = 1;
const BLUE: u8 = 2;
#[derive(Resource, Clone, Default)]
struct WaterWorld;

impl VoxelWorldConfig for WaterWorld {
    type MaterialIndex = u8;

    fn texture_index_mapper(&self) -> Arc<dyn Fn(u8) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat: u8| match vox_mat {
            RED => [1, 1, 1],
            GREEN => [2, 2, 2],
            BLUE | _ => [3, 3, 3],
        })
    }
}

impl WaterWorld {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterVoxelMaterial {
    // We're not using any uniforms in this example
    _unused: u32,
}

impl Material for WaterVoxelMaterial {
    fn vertex_shader() -> ShaderRef {
        // You can use the default shader from bevy_voxel_world for the vertex shader for simplicity
        VOXEL_TEXTURE_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        "water_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor
            .vertex
            .shader_defs
            .contains(&ShaderDefVal::Bool("PREPASS_PIPELINE".into(), true))
        {
            return Ok(());
        }

        // Use `vertex_layout()` from `bevy_voxel_world` to get the correct vertex layout
        let vertex_layout = layout.0.get_layout(&vertex_layout())?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

fn main() {
    assert_eq!(size_of::<WorldVoxel>(), 2);
    assert_eq!(size_of::<WorldVoxel<BlockTexture>>(), 1);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<WaterVoxelMaterial>::default())
        .add_plugins(
            VoxelWorldPlugin::with_config(WaterWorld::new())
                .with_material(WaterVoxelMaterial { _unused: 0 }),
        )
        .add_plugins((LookTransformPlugin, UnrealCameraPlugin::default()))
        .add_plugins(VoxelWorldPlugin::with_config(MyMainWorld::new()))
        .add_systems(Startup, (setup,).chain())
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands, mut water_world: VoxelWorld<WaterWorld>) {
    // Camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            // This tells bevy_voxel_world to use this cameras transform to calculate spawning area
            VoxelWorldCamera::<MyMainWorld>::default(),
            VoxelWorldCamera::<WaterWorld>::default(),
        ))
        .insert(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(10.0, 10.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
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

    for i in -20..20 {
        for j in -20..20 {
            water_world.set_voxel(IVec3::new(i, 0, j), WorldVoxel::Solid(BLUE));
        }
    }
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
