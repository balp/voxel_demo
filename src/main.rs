mod map;
mod textures;

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
use textures::BlockTexture;

#[derive(Resource, Clone)]
struct MyMainWorld {
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

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat| vox_mat.index_mapper())
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let static_ref: &'static Map = self.static_map;
        Box::new(move |_chunk_pos| get_voxel_fn(static_ref))
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(BlockTexture::get_texture())
    }
}
fn get_voxel_fn(
    world_map: &Map,
) -> Box<dyn FnMut(IVec3) -> WorldVoxel<BlockTexture> + Send + Sync + '_> {
    Box::new(move |pos: IVec3| {
        world_map.voxel_at(pos)
    })
}

#[derive(Resource, Clone, Default)]
struct VoxelTrace {
    start: Option<Vec3>,
    end: Vec3,
}
#[derive(Component)]
struct CursorCube {
    voxel_pos: IVec3,
    voxel_mat: u8,
}


const RED: u8 = 0;
const GREEN: u8 = 1;
const BLUE: u8 = 2;
const FULL_BRICK: u8 = 3;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
struct WaterSim;

#[derive(Resource, Clone)]
struct WaterWorld {
    water_sim: &'static WaterSim,
}

impl Default for WaterWorld {
    fn default() -> Self {
        let sim = Box::new(WaterSim::default());
        let water_sim: &'static WaterSim = Box::leak(Box::new(sim.clone()));
        Self { water_sim }
    }
}

// Start sim at 30, 67,
impl VoxelWorldConfig for WaterWorld {
    type MaterialIndex = u8;

    fn texture_index_mapper(&self) -> Arc<dyn Fn(u8) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat: u8| match vox_mat {
            RED => [1, 1, 1],
            GREEN => [2, 2, 2],
            BLUE => [3, 3, 3],
            FULL_BRICK | _ => [4, 4, 4],
        })
    }
}

impl WaterWorld {
    fn new() -> Self {
        let sim = Box::new(WaterSim::default());
        let water_sim: &'static WaterSim = Box::leak(Box::new(sim.clone()));
        Self { water_sim }
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
        .init_resource::<VoxelTrace>()
        .add_systems(Startup, (setup,).chain())
        .add_systems(Update, (close_on_esc, update_cursor_cube))
        .run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>,
         mut water_world: VoxelWorld<WaterWorld>) {
    // Cursor cube
    commands.spawn((
        Transform::from_xyz(0.0, -10.0, 0.0),
        Mesh3d(meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(0.5),
        }))),
        MeshMaterial3d(materials.add(Color::srgba_u8(124, 144, 255, 128))),
        CursorCube {
            voxel_pos: IVec3::new(0, -10, 0),
            voxel_mat: FULL_BRICK,
        },
    ));


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

fn update_cursor_cube(
    voxel_world_raycast: VoxelWorld<MyMainWorld>,
    mut trace: ResMut<VoxelTrace>,
    camera_info: Query<(&Camera, &GlobalTransform), With<VoxelWorldCamera<MyMainWorld>>>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut cursor_cube: Query<(&mut Transform, &mut CursorCube)>,
) {
    for ev in cursor_evr.read() {
        // Get a ray from the cursor position into the world
        let (camera, cam_gtf) = camera_info.single();
        let Ok(ray) = camera.viewport_to_world(cam_gtf, ev.position) else {
            return;
        };

        if let Some(result) = voxel_world_raycast.raycast(ray, &|(_pos, _vox)| true) {
            let (mut transform, mut cursor_cube) = cursor_cube.single_mut();

            // Camera could end up inside geometry - in that case just ignore the trace
            if let Some(normal) = result.normal {
                // Move the cursor cube to the position of the voxel we hit
                let voxel_pos = result.position + normal;
                transform.translation = voxel_pos + Vec3::splat(VOXEL_SIZE / 2.);
                cursor_cube.voxel_pos = voxel_pos.as_ivec3();
                println!("voxel_pos {:?}", cursor_cube.voxel_pos);
                // Update current trace end to the cursor cube position
                trace.end = transform.translation;
            }
        }
    }
}
