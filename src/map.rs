use bevy::math::IVec3;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) enum NodeType {
    #[default]
    Grass,
    Snow,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub(crate) struct Node {
    pub(crate) _type: NodeType,
    pub(crate) height: u8,
}

impl Node {
    fn new(_type: NodeType, height: u8) -> Self {
        Self { _type, height }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Map {
    pub(crate) size: Size,
    pub(crate) map: Vec<Vec<Node>>,
    min_x: i32,
    max_x: i32,
    min_z: i32,
    max_z: i32,
}

impl Map {
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
                    Node::new(NodeType::Snow, 8),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 8),
                ],
                vec![
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 5),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Snow, 5),
                    Node::new(NodeType::Snow, 6),
                    Node::new(NodeType::Snow, 7),
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
                    Node::new(NodeType::Grass, 8),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 8),
                    Node::new(NodeType::Grass, 2),
                ],
                vec![
                    Node::new(NodeType::Grass, 8),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 4),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 3),
                    Node::new(NodeType::Grass, 2),
                    Node::new(NodeType::Grass, 8),
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

    pub(crate) fn get(self: &Self, pos: IVec3) -> Option<Node> {
        if self.in_map(pos) {
            let x = (pos.x + self.min_x.abs()) as usize;
            let z = (pos.z + self.min_z.abs()) as usize;
            Some(self.map[x][z])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::IVec3;
    use super::*;

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
        assert_eq!(sut.get(pos), Some(Node::new(NodeType::Snow, 8)))
    }
    #[test]
    fn get_map_5x0x5() {
        let sut = Map::test_map();
        let pos = IVec3::new(5, 0, 4);
        let opt_node = sut.get(pos);
        assert_eq!(opt_node, Some(Node::new(NodeType::Grass, 8)))
    }


}