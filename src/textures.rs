
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


// Using enum for material index allows for more than u8::MAX number of materials.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum BlockTexture {
    #[default]
    GrassBrick,
    SnowyBrick,
    DirtBrick,
    SandBrick,
    GravelBrick,
    StoneBrick,
    RockBrick,
    WaterBrick,
    FullBrick,
}

impl BlockTexture {
    pub(crate) fn index_mapper(&self) -> [u32; 3] {
        match self {
            BlockTexture::GrassBrick => [23, 23, 23],
            BlockTexture::SnowyBrick => [49, 49, 49],
            BlockTexture::DirtBrick => [9, 9, 9],
            BlockTexture::SandBrick => [48, 48, 48],
            BlockTexture::GravelBrick => [24, 24, 24],
            BlockTexture::StoneBrick => [50, 50, 50],
            BlockTexture::RockBrick => [46, 46, 46],
            BlockTexture::WaterBrick => [78, 78, 78],
            BlockTexture::FullBrick => [9, 9, 9],
        }
    }

    pub fn get_texture() -> (String, u32) {
        ("voxel_textures_all.png".into(), 85)
    }
}

