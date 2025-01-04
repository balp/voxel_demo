use bevy::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::Arc;
use bevy::log::LogPlugin;

// Using enum for material index allows for more than u8::MAX number of materials.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
enum BlockTexture {
    #[default]
    SnowyBrick,
    FullBrick,
    Grass,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
enum NodeType {
    #[default]
    Grass,
    Snow,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
struct Node {
    _type: NodeType,
    height: u8,
}

impl Node {
    fn new(_type: NodeType, height: u8) -> Self {
        Self { _type, height }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
struct Map {
    size: Size,
    map: Vec<Vec<Node>>,
}

impl Map {
    fn test_map() -> Self {
        Map {
            size: Size { width: 10, height: 10 },
            map: vec![
                vec![
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                ],
                vec![
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 6),
                    Node::new(NodeType::Grass, 7),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                ],
                vec![
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                ],
                vec![
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                ],
                vec![
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 6),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                ],
                vec![
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 6),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                ],
                vec![
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                ],
                vec![
                    Node::new(NodeType::Grass, 1),
                    Node::new(NodeType::Grass, 1),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                ],
                vec![
                    Node::new(NodeType::Grass, 1),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                ],
                vec![
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                ],
                vec![
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 1),
                ],
            ],
        }
    }

    fn in_map(self: &Self, pos: IVec3) -> bool {
        let min_x = 0 - (self.size.width / 2) as i32;
        let max_x = (self.size.width / 2) as i32;
        let min_z = 0 - (self.size.height / 2) as i32;
        let max_z = (self.size.height / 2) as i32;
        pos.x > min_x && pos.x < max_x && pos.z > min_z && pos.z < max_z
    }

    fn get(self: &Self, pos: IVec3) -> Option<Node> {
        // warn!("get {:?}", pos);
        if self.in_map(pos) {
            let min_x = 0 - (self.size.width / 2) as i32;
            let min_z = 0 - (self.size.height / 2) as i32;
            let x = (pos.x + min_x.abs()) as usize;
            let z = (pos.z + min_z.abs()) as usize;
            Some(self.map[x][z])
        } else {
            None
        }
    }
}

#[derive(Resource, Clone, Default)]
struct MyMainWorld;

impl VoxelWorldConfig for MyMainWorld {
    type MaterialIndex = BlockTexture;

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|vox_mat| match vox_mat {
            BlockTexture::SnowyBrick => [0, 1, 2],
            BlockTexture::FullBrick => [2, 2, 2],
            BlockTexture::Grass => [3, 3, 3],
        })
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos| get_voxel_fn())
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("example_voxel_texture.png".into(), 4))
    }
}
fn get_voxel_fn() -> Box<dyn FnMut(IVec3) -> WorldVoxel<BlockTexture> + Send + Sync> {
    let world_map = Map::test_map();

    Box::new(move |pos: IVec3| {
        let node = world_map.get(pos);
        match node {
            None => WorldVoxel::Unset,
            Some(n) => {
                if pos.y < (n.height as i32) {
                    WorldVoxel::Solid(BlockTexture::Grass)
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
        .add_plugins(VoxelWorldPlugin::with_config(MyMainWorld))
        .add_systems(Startup, (setup, create_voxel_scene).chain())
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

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn create_voxel_scene(mut voxel_world: VoxelWorld<MyMainWorld>) {
    // // Then we can use the `u8` consts to specify the type of voxel
    //
    // // 20 by 20 floor
    // let x_size = 40;
    // let y_size = 30;
    // let min_x = 0 - (x_size / 2);
    // let max_x = (x_size / 2);
    // let min_y = 0 - (y_size / 2);
    // let max_y = (x_size / 2);
    //
    // for x in min_x..max_x {
    //     for z in min_y..max_y {
    //         voxel_world.set_voxel(IVec3::new(x, -1, z), WorldVoxel::Solid(BlockTexture::Grass));
    //         // Grassy floor
    //     }
    // }
    //
    // // Some bricks
    // voxel_world.set_voxel(
    //     IVec3::new(0, 0, 0),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(1, 0, 0),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(0, 0, 1),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(0, 0, -1),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(-1, 0, 0),
    //     WorldVoxel::Solid(BlockTexture::FullBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(-2, 0, 0),
    //     WorldVoxel::Solid(BlockTexture::FullBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(-1, 1, 0),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(-2, 1, 0),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
    // voxel_world.set_voxel(
    //     IVec3::new(0, 1, 0),
    //     WorldVoxel::Solid(BlockTexture::SnowyBrick),
    // );
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<VoxelWorldCamera<MyMainWorld>>>,
) {
    let mut transform = query.single_mut();
    let time_seconds = time.elapsed_secs();
    transform.translation.x = 25.0 * (time_seconds * 0.1).sin();
    transform.translation.z = 25.0 * (time_seconds * 0.1).cos();
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