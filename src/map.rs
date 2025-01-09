use bevy::math::IVec3;
use bevy_voxel_world::prelude::WorldVoxel;
use noise::{HybridMulti, NoiseFn, Perlin};
use crate::textures::BlockTexture;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) enum NodeType {
    #[default]
    Grass,
    Snow,
    Dirt,
    Sand,
    Gravel,
    Stone,
    Rock,
    Water,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) struct MapNode {
    pub(crate) surface_type: NodeType,
    pub(crate) height: i8,
}

impl MapNode {
    fn new(surface_type: NodeType, height: i8) -> Self {
        Self { surface_type, height }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Map {
    pub(crate) size: Size,
    pub(crate) map: Vec<Vec<MapNode>>,
    min_x: i32,
    max_x: i32,
    min_z: i32,
    max_z: i32,
}

impl Map {
    pub(crate) fn noise_map(size: Size) -> Self {
        let mut noise = HybridMulti::<Perlin>::new(1234);
        noise.octaves = 5;
        noise.frequency = 1.1;
        noise.lacunarity = 2.8;
        noise.persistence = 0.4;

        let min_x = 0 - (size.width / 2) as i32;
        let max_x = min_x + size.width as i32;
        let min_z = 0 - (size.height / 2) as i32;
        let max_z = min_z + size.height as i32;
        let mut map = Vec::new();
        for x in min_x..max_x {
            let mut row = Vec::new();
            for z in min_z..max_z {
                let float_height = noise.get([x as f64 / 1000.0, z as f64 / 1000.0]) * 50.0;
                let height = float_height.floor() as i8;
                // println!("new float height: {} {}", float_height, height);
                let surface_type = match height {
                    x if x < 0 => NodeType::Gravel,
                    x if x == 0 => NodeType::Sand,
                    x if x > 15 => NodeType::Stone,
                    x if x > 30 => NodeType::Rock,
                    x if x > 35 => NodeType::Snow,
                    _ => NodeType::Grass,
                };
                row.push(MapNode {
                    surface_type,
                    height,
                })
            }
            // println!();
            map.push(row);
        }

        let mut m = Self {
            size,
            map,
            min_x,
            max_x,
            min_z,
            max_z,
        };
        m.set_surface(IVec3::new(30,0, 67), NodeType::Water);
        m
    }
    pub(crate) fn test_map() -> Self {
        let width = 11;
        let height = 10;
        let min_x = 0 - (width / 2) as i32;
        let max_x = min_x + width as i32;
        let min_z = 0 - (height / 2) as i32;
        let max_z = min_z + height as i32;

        Map {
            size: Size { width, height },
            map: vec![
                vec![
                    MapNode::new(NodeType::Snow, 8),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Snow, 8),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Snow, 5),
                    MapNode::new(NodeType::Snow, 6),
                    MapNode::new(NodeType::Snow, 7),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 5),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 6),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 6),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 5),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 1),
                    MapNode::new(NodeType::Grass, 1),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 1),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 8),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 8),
                    MapNode::new(NodeType::Grass, 2),
                ],
                vec![
                    MapNode::new(NodeType::Grass, 8),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 4),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 3),
                    MapNode::new(NodeType::Grass, 2),
                    MapNode::new(NodeType::Grass, 8),
                ],
            ],
            min_x,
            max_x,
            min_z,
            max_z,
        }
    }

    fn in_map(self: &Self, pos: IVec3) -> bool {
        pos.x >= self.min_x && pos.x < self.max_x && pos.z >= self.min_z && pos.z < self.max_z
    }

    pub(crate) fn get(self: &Self, pos: IVec3) -> Option<MapNode> {
        if self.in_map(pos) {
            let x = (pos.x + self.min_x.abs()) as usize;
            let z = (pos.z + self.min_z.abs()) as usize;
            Some(self.map[x][z])
        } else {
            None
        }
    }

    pub(crate) fn voxel_at(self: &Self, pos: IVec3) -> WorldVoxel<BlockTexture> {
        let node = self.get(pos);
        match node {
            None => WorldVoxel::Unset,
            Some(n) => {
                if pos.y < (n.height as i32) {
                    WorldVoxel::Solid(BlockTexture::FullBrick)
                } else if pos.y == (n.height as i32) {
                    match n.surface_type {
                        NodeType::Grass => WorldVoxel::Solid(BlockTexture::GrassBrick),
                        NodeType::Snow => WorldVoxel::Solid(BlockTexture::SnowyBrick),
                        NodeType::Dirt => WorldVoxel::Solid(BlockTexture::DirtBrick),
                        NodeType::Sand => WorldVoxel::Solid(BlockTexture::SandBrick),
                        NodeType::Gravel => WorldVoxel::Solid(BlockTexture::GravelBrick),
                        NodeType::Stone => WorldVoxel::Solid(BlockTexture::StoneBrick),
                        NodeType::Rock => WorldVoxel::Solid(BlockTexture::RockBrick),
                        NodeType::Water => WorldVoxel::Solid(BlockTexture::WaterBrick),
                    }
                } else {
                    WorldVoxel::Air
                }
            }
        }
    }

    fn set_surface(self: &mut Self, pos: IVec3, surface: NodeType) {
        if self.in_map(pos) {
            let x = (pos.x + self.min_x.abs()) as usize;
            let z = (pos.z + self.min_z.abs()) as usize;
            let node = &mut self.map[x][z];
            node.surface_type = surface;
        }


    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::IVec3;

    #[test]
    fn in_map_n5xn5() {
        let sut = Map::test_map();
        let pos = IVec3::new(-5, -5, -5);
        assert_eq!(sut.in_map(pos), true)
    }
    #[test]
    fn outside_map_n6xn5() {
        let sut = Map::test_map();
        let pos = IVec3::new(-6, -5, -5);
        assert_eq!(sut.in_map(pos), false)
    }
    #[test]
    fn outside_map_n5xn6() {
        let sut = Map::test_map();
        let pos = IVec3::new(-5, -5, -6);
        assert_eq!(sut.in_map(pos), false)
    }
    #[test]
    fn in_map_n6xn5_zn6() {
        let sut = Map::test_map();
        let pos = IVec3::new(-5, -6, -5);
        assert_eq!(sut.in_map(pos), true)
    }
    #[test]
    fn in_map_5x5() {
        let sut = Map::test_map();
        let pos = IVec3::new(5, -5, 4);
        assert_eq!(sut.in_map(pos), true)
    }
    #[test]
    fn outside_map_6x5() {
        let sut = Map::test_map();
        let pos = IVec3::new(6, -5, 4);
        assert_eq!(sut.in_map(pos), false)
    }
    #[test]
    fn outside_map_5x6() {
        let sut = Map::test_map();
        let pos = IVec3::new(5, -5, 6);
        assert_eq!(sut.in_map(pos), false)
    }
    #[test]
    fn get_map_n5x0xn5() {
        let sut = Map::test_map();
        let pos = IVec3::new(-5, 0, -5);
        assert_eq!(sut.get(pos), Some(MapNode::new(NodeType::Snow, 8)))
    }
    #[test]
    fn get_map_5x0x5() {
        let sut = Map::test_map();
        let pos = IVec3::new(5, 0, 4);
        let opt_node = sut.get(pos);
        assert_eq!(opt_node, Some(MapNode::new(NodeType::Grass, 8)))
    }

    #[test]
    fn get_noise_map_20x20() {
        let size = Size {
            width: 20,
            height: 20,
        };
        let sut = Map::noise_map(size);
        assert_eq!(sut.size, size);
        let position = sut.get(IVec3::new(0, 0, 0));
        assert_ne!(position, None);
    }
}
