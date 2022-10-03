use crate::inventory::Material;

pub fn to_id(mat: Material) -> u16 {
    match mat {
        Material::Air => 0,
        Material::Stone => 1,
        Material::Grass => 2,
        Material::Dirt => 3,
        Material::Cobblestone => 4,
        Material::Wood => 5,
        Material::Sapling => 6,
        Material::Bedrock => 7,
        Material::Water => 8,
        Material::StationaryWater => 9,
        Material::Lava => 10,
        Material::StationaryLava => 11,
        Material::Sand => 12,
        Material::Gravel => 13,
        Material::GoldOre => 14,
        Material::IronOre => 15,
        Material::CoalOre => 16,
        Material::Log => 17,
        Material::Leaves => 18,
        Material::Sponge => 19,
        Material::Glass => 20,
        Material::LapisOre => 21,
        Material::LapisBlock => 22,
        Material::Dispenser => 23,
        Material::Sandstone => 24,
        Material::NoteBlock => 25,
        Material::BedBlock => 26,
        Material::PoweredRail => 27,
        Material::DetectorRail => 28,
        Material::PistonStickyBase => 29,
        Material::Web => 30,
        Material::LongGrass => 31,
        Material::DeadBush => 32,
        Material::PistonBase => 33,
        Material::PistonExtension => 34,
        Material::Wool => 35,
        Material::PistonMovingPiece => 36,
        Material::YellowFlower => 37,
        Material::RedRose => 38,
        Material::BrownMushroom => 39,
        Material::RedMushroom => 40,
        Material::GoldBlock => 41,
        Material::IronBlock => 42,
        Material::DoubleStep => 43,
        Material::Step => 44,
        Material::Brick => 45,
        Material::Tnt => 46,
        Material::Bookshelf => 47,
        Material::MossyCobblestone => 48,
        Material::Obsidian => 49,
        Material::Torch => 50,
        Material::Fire => 51,
        Material::MobSpawner => 52,
        Material::WoodStairs => 53,
        Material::Chest => 54,
        Material::RedstoneWire => 55,
        Material::DiamondOre => 56,
        Material::DiamondBlock => 57,
        Material::Workbench => 58,
        Material::Crops => 59,
        Material::Soil => 60,
        Material::Furnace => 61,
        Material::BurningFurnace => 62,
        Material::SignPost => 63,
        Material::WoodenDoor => 64,
        Material::Ladder => 65,
        Material::Rails => 66,
        Material::CobblestoneStairs => 67,
        Material::WallSign => 68,
        Material::Lever => 69,
        Material::StonePlate => 70,
        Material::IronDoorBlock => 71,
        Material::WoodPlate => 72,
        Material::RedstoneOre => 73,
        Material::GlowingRedstoneOre => 74,
        Material::RedstoneTorchOff => 75,
        Material::RedstoneTorchOn => 76,
        Material::StoneButton => 77,
        Material::Snow => 78,
        Material::Ice => 79,
        Material::SnowBlock => 80,
        Material::Cactus => 81,
        Material::Clay => 82,
        Material::SugarCaneBlock => 83,
        Material::Jukebox => 84,
        Material::Fence => 85,
        Material::Pumpkin => 86,
        Material::Netherrack => 87,
        Material::SoulSand => 88,
        Material::Glowstone => 89,
        Material::Portal => 90,
        Material::JackOLantern => 91,
        Material::CakeBlock => 92,
        Material::DiodeBlockOff => 93,
        Material::DiodeBlockOn => 94,
        Material::StainedGlass => 95,
        Material::TrapDoor => 96,
        Material::MonsterEggs => 97,
        Material::SmoothBrick => 98,
        Material::HugeMushroom1 => 99,
        Material::HugeMushroom2 => 100,
        Material::IronFence => 101,
        Material::ThinGlass => 102,
        Material::MelonBlock => 103,
        Material::PumpkinStem => 104,
        Material::MelonStem => 105,
        Material::Vine => 106,
        Material::FenceGate => 107,
        Material::BrickStairs => 108,
        Material::SmoothStairs => 109,
        Material::Mycel => 110,
        Material::WaterLily => 111,
        Material::NetherBrick => 112,
        Material::NetherFence => 113,
        Material::NetherBrickStairs => 114,
        Material::NetherWarts => 115,
        Material::EnchantmentTable => 116,
        Material::BrewingStand => 117,
        Material::Cauldron => 118,
        Material::EnderPortal => 119,
        Material::EnderPortalFrame => 120,
        Material::EnderStone => 121,
        Material::DragonEgg => 122,
        Material::RedstoneLampOff => 123,
        Material::RedstoneLampOn => 124,
        Material::WoodDoubleStep => 125,
        Material::WoodStep => 126,
        Material::Cocoa => 127,
        Material::SandstoneStairs => 128,
        Material::EmeraldOre => 129,
        Material::EnderChest => 130,
        Material::TripwireHook => 131,
        Material::Tripwire => 132,
        Material::EmeraldBlock => 133,
        Material::SpruceWoodStairs => 134,
        Material::BirchWoodStairs => 135,
        Material::JungleWoodStairs => 136,
        Material::Command => 137,
        Material::Beacon => 138,
        Material::CobbleWall => 139,
        Material::FlowerPot => 140,
        Material::Carrot => 141,
        Material::Potato => 142,
        Material::WoodButton => 143,
        Material::Skull => 144,
        Material::Anvil => 145,
        Material::TrappedChest => 146,
        Material::GoldPlate => 147,
        Material::IronPlate => 148,
        Material::RedstoneComparatorOff => 149,
        Material::RedstoneComparatorOn => 150,
        Material::DaylightDetector => 151,
        Material::RedstoneBlock => 152,
        Material::QuartzOre => 153,
        Material::Hopper => 154,
        Material::QuartzBlock => 155,
        Material::QuartzStairs => 156,
        Material::ActivatorRail => 157,
        Material::Dropper => 158,
        Material::StainedClay => 159,
        Material::StainedGlassPane => 160,
        Material::Leaves2 => 161,
        Material::Log2 => 162,
        Material::AcaciaStairs => 163,
        Material::DarkOakStairs => 164,
        Material::SlimeBlock => 165,
        Material::Barrier => 166,
        Material::IronTrapdoor => 167,
        Material::Prismarine => 168,
        Material::SeaLantern => 169,
        Material::HayBlock => 170,
        Material::Carpet => 171,
        Material::HardClay => 172,
        Material::CoalBlock => 173,
        Material::PackedIce => 174,
        Material::DoublePlant => 175,
        Material::StandingBanner => 176,
        Material::WallBanner => 177,
        Material::DaylightDetectorInverted => 178,
        Material::RedSandstone => 179,
        Material::RedSandstoneStairs => 180,
        Material::DoubleStoneSlab2 => 181,
        Material::StoneSlab2 => 182,
        Material::SpruceFenceGate => 183,
        Material::BirchFenceGate => 184,
        Material::JungleFenceGate => 185,
        Material::DarkOakFenceGate => 186,
        Material::AcaciaFenceGate => 187,
        Material::SpruceFence => 188,
        Material::BirchFence => 189,
        Material::JungleFence => 190,
        Material::DarkOakFence => 191,
        Material::AcaciaFence => 192,
        Material::SpruceDoor => 193,
        Material::BirchDoor => 194,
        Material::JungleDoor => 195,
        Material::AcaciaDoor => 196,
        Material::DarkOakDoor => 197,
        Material::EndRod => 198,
        Material::ChorusPlant => 199,
        Material::ChorusFlower => 200,
        Material::PurpurBlock => 201,
        Material::PurpurPillar => 202,
        Material::PurpurStairs => 203,
        Material::PurpurDoubleSlab => 204,
        Material::PurpurSlab => 205,
        Material::EndBricks => 206,
        Material::BeetrootBlock => 207,
        Material::GrassPath => 208,
        Material::EndGateway => 209,
        Material::CommandRepeating => 210,
        Material::CommandChain => 211,
        Material::FrostedIce => 212,
        Material::Magma => 213,
        Material::NetherWartBlock => 214,
        Material::RedNetherBrick => 215,
        Material::BoneBlock => 216,
        Material::StructureVoid => 217,
        Material::StructureBlock => 255,
        Material::IronSpade => 256,
        Material::IronPickaxe => 257,
        Material::IronAxe => 258,
        Material::FlintAndSteel => 259,
        Material::Apple => 260,
        Material::Bow => 261,
        Material::Arrow => 262,
        Material::Coal => 263,
        Material::Diamond => 264,
        Material::IronIngot => 265,
        Material::GoldIngot => 266,
        Material::IronSword => 267,
        Material::WoodSword => 268,
        Material::WoodSpade => 269,
        Material::WoodPickaxe => 270,
        Material::WoodAxe => 271,
        Material::StoneSword => 272,
        Material::StoneSpade => 273,
        Material::StonePickaxe => 274,
        Material::StoneAxe => 275,
        Material::DiamondSword => 276,
        Material::DiamondSpade => 277,
        Material::DiamondPickaxe => 278,
        Material::DiamondAxe => 279,
        Material::Stick => 280,
        Material::Bowl => 281,
        Material::MushroomSoup => 282,
        Material::GoldSword => 283,
        Material::GoldSpade => 284,
        Material::GoldPickaxe => 285,
        Material::GoldAxe => 286,
        Material::String => 287,
        Material::Feather => 288,
        Material::Sulphur => 289,
        Material::WoodHoe => 290,
        Material::StoneHoe => 291,
        Material::IronHoe => 292,
        Material::DiamondHoe => 293,
        Material::GoldHoe => 294,
        Material::Seeds => 295,
        Material::Wheat => 296,
        Material::Bread => 297,
        Material::LeatherHelmet => 298,
        Material::LeatherChestplate => 299,
        Material::LeatherLeggings => 300,
        Material::LeatherBoots => 301,
        Material::ChainmailHelmet => 302,
        Material::ChainmailChestplate => 303,
        Material::ChainmailLeggings => 304,
        Material::ChainmailBoots => 305,
        Material::IronHelmet => 306,
        Material::IronChestplate => 307,
        Material::IronLeggings => 308,
        Material::IronBoots => 309,
        Material::DiamondHelmet => 310,
        Material::DiamondChestplate => 311,
        Material::DiamondLeggings => 312,
        Material::DiamondBoots => 313,
        Material::GoldHelmet => 314,
        Material::GoldChestplate => 315,
        Material::GoldLeggings => 316,
        Material::GoldBoots => 317,
        Material::Flint => 318,
        Material::Pork => 319,
        Material::GrilledPork => 320,
        Material::Painting => 321,
        Material::GoldenApple => 322,
        Material::Sign => 323,
        Material::WoodDoor => 324,
        Material::Bucket => 325,
        Material::WaterBucket => 326,
        Material::LavaBucket => 327,
        Material::Minecart => 328,
        Material::Saddle => 329,
        Material::IronDoor => 330,
        Material::Redstone => 331,
        Material::SnowBall => 332,
        Material::Boat => 333,
        Material::Leather => 334,
        Material::MilkBucket => 335,
        Material::ClayBrick => 336,
        Material::ClayBall => 337,
        Material::SugarCane => 338,
        Material::Paper => 339,
        Material::Book => 340,
        Material::SlimeBall => 341,
        Material::StorageMinecart => 342,
        Material::PoweredMinecart => 343,
        Material::Egg => 344,
        Material::Compass => 345,
        Material::FishingRod => 346,
        Material::Watch => 347,
        Material::GlowstoneDust => 348,
        Material::RawFish => 349,
        Material::CookedFish => 350,
        Material::InkSack => 351,
        Material::Bone => 352,
        Material::Sugar => 353,
        Material::Cake => 354,
        Material::Bed => 355,
        Material::Diode => 356,
        Material::Cookie => 357,
        Material::Map => 358,
        Material::Shears => 359,
        Material::Melon => 360,
        Material::PumpkinSeeds => 361,
        Material::MelonSeeds => 362,
        Material::RawBeef => 363,
        Material::CookedBeef => 364,
        Material::RawChicken => 365,
        Material::CookedChicken => 366,
        Material::RottenFlesh => 367,
        Material::EnderPearl => 368,
        Material::BlazeRod => 369,
        Material::GhastTear => 370,
        Material::GoldNugget => 371,
        Material::NetherStalk => 372,
        Material::Potion => 373,
        Material::GlassBottle => 374,
        Material::SpiderEye => 375,
        Material::FermentedSpiderEye => 376,
        Material::BlazePowder => 377,
        Material::MagmaCream => 378,
        Material::BrewingStandItem => 379,
        Material::CauldronItem => 380,
        Material::EyeOfEnder => 381,
        Material::SpeckledMelon => 382,
        Material::MonsterEgg => 383,
        Material::ExpBottle => 384,
        Material::Fireball => 385,
        Material::BookAndQuill => 386,
        Material::WrittenBook => 387,
        Material::Emerald => 388,
        Material::ItemFrame => 389,
        Material::FlowerPotItem => 390,
        Material::CarrotItem => 391,
        Material::PotatoItem => 392,
        Material::BakedPotato => 393,
        Material::PoisonousPotato => 394,
        Material::EmptyMap => 395,
        Material::GoldenCarrot => 396,
        Material::SkullItem => 397,
        Material::CarrotStick => 398,
        Material::NetherStar => 399,
        Material::PumpkinPie => 400,
        Material::Firework => 401,
        Material::FireworkCharge => 402,
        Material::EnchantedBook => 403,
        Material::RedstoneComparator => 404,
        Material::NetherBrickItem => 405,
        Material::Quartz => 406,
        Material::ExplosiveMinecart => 407,
        Material::HopperMinecart => 408,
        Material::PrismarineShard => 409,
        Material::PrismarineCrystals => 410,
        Material::Rabbit => 411,
        Material::CookedRabbit => 412,
        Material::RabbitStew => 413,
        Material::RabbitFoot => 414,
        Material::RabbitHide => 415,
        Material::ArmorStand => 416,
        Material::IronBarding => 417,
        Material::GoldBarding => 418,
        Material::DiamondBarding => 419,
        Material::Leash => 420,
        Material::NameTag => 421,
        Material::CommandMinecart => 422,
        Material::Mutton => 423,
        Material::CookedMutton => 424,
        Material::Banner => 425,
        Material::EndCrystal => 426,
        Material::SpruceDoorItem => 427,
        Material::BirchDoorItem => 428,
        Material::JungleDoorItem => 429,
        Material::AcaciaDoorItem => 430,
        Material::DarkOakDoorItem => 431,
        Material::ChorusFruit => 432,
        Material::ChorusFruitPopped => 433,
        Material::Beetroot => 434,
        Material::BeetrootSeeds => 435,
        Material::BeetrootSoup => 436,
        Material::DragonsBreath => 437,
        Material::SplashPotion => 438,
        Material::SpectralArrow => 439,
        Material::TippedArrow => 440,
        Material::LingeringPotion => 441,
        Material::Shield => 442,
        Material::Elytra => 443,
        Material::BoatSpruce => 444,
        Material::BoatBirch => 445,
        Material::BoatJungle => 446,
        Material::BoatAcacia => 447,
        Material::BoatDarkOak => 448,
        Material::GoldRecord => 2256,
        Material::GreenRecord => 2257,
        Material::Record3 => 2258,
        Material::Record4 => 2259,
        Material::Record5 => 2260,
        Material::Record6 => 2261,
        Material::Record7 => 2262,
        Material::Record8 => 2263,
        Material::Record9 => 2264,
        Material::Record10 => 2265,
        Material::Record11 => 2266,
        Material::Record12 => 2267,
        _ => 0,
    }
}

pub fn to_material(material_id: u16) -> Material {
    match material_id {
        0 => Material::Air,
        1 => Material::Stone,
        2 => Material::Grass,
        3 => Material::Dirt,
        4 => Material::Cobblestone,
        5 => Material::Wood,
        6 => Material::Sapling,
        7 => Material::Bedrock,
        8 => Material::Water,
        9 => Material::StationaryWater,
        10 => Material::Lava,
        11 => Material::StationaryLava,
        12 => Material::Sand,
        13 => Material::Gravel,
        14 => Material::GoldOre,
        15 => Material::IronOre,
        16 => Material::CoalOre,
        17 => Material::Log,
        18 => Material::Leaves,
        19 => Material::Sponge,
        20 => Material::Glass,
        21 => Material::LapisOre,
        22 => Material::LapisBlock,
        23 => Material::Dispenser,
        24 => Material::Sandstone,
        25 => Material::NoteBlock,
        26 => Material::BedBlock,
        27 => Material::PoweredRail,
        28 => Material::DetectorRail,
        29 => Material::PistonStickyBase,
        30 => Material::Web,
        31 => Material::LongGrass,
        32 => Material::DeadBush,
        33 => Material::PistonBase,
        34 => Material::PistonExtension,
        35 => Material::Wool,
        36 => Material::PistonMovingPiece,
        37 => Material::YellowFlower,
        38 => Material::RedRose,
        39 => Material::BrownMushroom,
        40 => Material::RedMushroom,
        41 => Material::GoldBlock,
        42 => Material::IronBlock,
        43 => Material::DoubleStep,
        44 => Material::Step,
        45 => Material::Brick,
        46 => Material::Tnt,
        47 => Material::Bookshelf,
        48 => Material::MossyCobblestone,
        49 => Material::Obsidian,
        50 => Material::Torch,
        51 => Material::Fire,
        52 => Material::MobSpawner,
        53 => Material::WoodStairs,
        54 => Material::Chest,
        55 => Material::RedstoneWire,
        56 => Material::DiamondOre,
        57 => Material::DiamondBlock,
        58 => Material::Workbench,
        59 => Material::Crops,
        60 => Material::Soil,
        61 => Material::Furnace,
        62 => Material::BurningFurnace,
        63 => Material::SignPost,
        64 => Material::WoodenDoor,
        65 => Material::Ladder,
        66 => Material::Rails,
        67 => Material::CobblestoneStairs,
        68 => Material::WallSign,
        69 => Material::Lever,
        70 => Material::StonePlate,
        71 => Material::IronDoorBlock,
        72 => Material::WoodPlate,
        73 => Material::RedstoneOre,
        74 => Material::GlowingRedstoneOre,
        75 => Material::RedstoneTorchOff,
        76 => Material::RedstoneTorchOn,
        77 => Material::StoneButton,
        78 => Material::Snow,
        79 => Material::Ice,
        80 => Material::SnowBlock,
        81 => Material::Cactus,
        82 => Material::Clay,
        83 => Material::SugarCaneBlock,
        84 => Material::Jukebox,
        85 => Material::Fence,
        86 => Material::Pumpkin,
        87 => Material::Netherrack,
        88 => Material::SoulSand,
        89 => Material::Glowstone,
        90 => Material::Portal,
        91 => Material::JackOLantern,
        92 => Material::CakeBlock,
        93 => Material::DiodeBlockOff,
        94 => Material::DiodeBlockOn,
        95 => Material::StainedGlass,
        96 => Material::TrapDoor,
        97 => Material::MonsterEggs,
        98 => Material::SmoothBrick,
        99 => Material::HugeMushroom1,
        100 => Material::HugeMushroom2,
        101 => Material::IronFence,
        102 => Material::ThinGlass,
        103 => Material::MelonBlock,
        104 => Material::PumpkinStem,
        105 => Material::MelonStem,
        106 => Material::Vine,
        107 => Material::FenceGate,
        108 => Material::BrickStairs,
        109 => Material::SmoothStairs,
        110 => Material::Mycel,
        111 => Material::WaterLily,
        112 => Material::NetherBrick,
        113 => Material::NetherFence,
        114 => Material::NetherBrickStairs,
        115 => Material::NetherWarts,
        116 => Material::EnchantmentTable,
        117 => Material::BrewingStand,
        118 => Material::Cauldron,
        119 => Material::EnderPortal,
        120 => Material::EnderPortalFrame,
        121 => Material::EnderStone,
        122 => Material::DragonEgg,
        123 => Material::RedstoneLampOff,
        124 => Material::RedstoneLampOn,
        125 => Material::WoodDoubleStep,
        126 => Material::WoodStep,
        127 => Material::Cocoa,
        128 => Material::SandstoneStairs,
        129 => Material::EmeraldOre,
        130 => Material::EnderChest,
        131 => Material::TripwireHook,
        132 => Material::Tripwire,
        133 => Material::EmeraldBlock,
        134 => Material::SpruceWoodStairs,
        135 => Material::BirchWoodStairs,
        136 => Material::JungleWoodStairs,
        137 => Material::Command,
        138 => Material::Beacon,
        139 => Material::CobbleWall,
        140 => Material::FlowerPot,
        141 => Material::Carrot,
        142 => Material::Potato,
        143 => Material::WoodButton,
        144 => Material::Skull,
        145 => Material::Anvil,
        146 => Material::TrappedChest,
        147 => Material::GoldPlate,
        148 => Material::IronPlate,
        149 => Material::RedstoneComparatorOff,
        150 => Material::RedstoneComparatorOn,
        151 => Material::DaylightDetector,
        152 => Material::RedstoneBlock,
        153 => Material::QuartzOre,
        154 => Material::Hopper,
        155 => Material::QuartzBlock,
        156 => Material::QuartzStairs,
        157 => Material::ActivatorRail,
        158 => Material::Dropper,
        159 => Material::StainedClay,
        160 => Material::StainedGlassPane,
        161 => Material::Leaves2,
        162 => Material::Log2,
        163 => Material::AcaciaStairs,
        164 => Material::DarkOakStairs,
        165 => Material::SlimeBlock,
        166 => Material::Barrier,
        167 => Material::IronTrapdoor,
        168 => Material::Prismarine,
        169 => Material::SeaLantern,
        170 => Material::HayBlock,
        171 => Material::Carpet,
        172 => Material::HardClay,
        173 => Material::CoalBlock,
        174 => Material::PackedIce,
        175 => Material::DoublePlant,
        176 => Material::StandingBanner,
        177 => Material::WallBanner,
        178 => Material::DaylightDetectorInverted,
        179 => Material::RedSandstone,
        180 => Material::RedSandstoneStairs,
        181 => Material::DoubleStoneSlab2,
        182 => Material::StoneSlab2,
        183 => Material::SpruceFenceGate,
        184 => Material::BirchFenceGate,
        185 => Material::JungleFenceGate,
        186 => Material::DarkOakFenceGate,
        187 => Material::AcaciaFenceGate,
        188 => Material::SpruceFence,
        189 => Material::BirchFence,
        190 => Material::JungleFence,
        191 => Material::DarkOakFence,
        192 => Material::AcaciaFence,
        193 => Material::SpruceDoor,
        194 => Material::BirchDoor,
        195 => Material::JungleDoor,
        196 => Material::AcaciaDoor,
        197 => Material::DarkOakDoor,
        198 => Material::EndRod,
        199 => Material::ChorusPlant,
        200 => Material::ChorusFlower,
        201 => Material::PurpurBlock,
        202 => Material::PurpurPillar,
        203 => Material::PurpurStairs,
        204 => Material::PurpurDoubleSlab,
        205 => Material::PurpurSlab,
        206 => Material::EndBricks,
        207 => Material::BeetrootBlock,
        208 => Material::GrassPath,
        209 => Material::EndGateway,
        210 => Material::CommandRepeating,
        211 => Material::CommandChain,
        212 => Material::FrostedIce,
        213 => Material::Magma,
        214 => Material::NetherWartBlock,
        215 => Material::RedNetherBrick,
        216 => Material::BoneBlock,
        217 => Material::StructureVoid,
        255 => Material::StructureBlock,
        256 => Material::IronSpade,
        257 => Material::IronPickaxe,
        258 => Material::IronAxe,
        259 => Material::FlintAndSteel,
        260 => Material::Apple,
        261 => Material::Bow,
        262 => Material::Arrow,
        263 => Material::Coal,
        264 => Material::Diamond,
        265 => Material::IronIngot,
        266 => Material::GoldIngot,
        267 => Material::IronSword,
        268 => Material::WoodSword,
        269 => Material::WoodSpade,
        270 => Material::WoodPickaxe,
        271 => Material::WoodAxe,
        272 => Material::StoneSword,
        273 => Material::StoneSpade,
        274 => Material::StonePickaxe,
        275 => Material::StoneAxe,
        276 => Material::DiamondSword,
        277 => Material::DiamondSpade,
        278 => Material::DiamondPickaxe,
        279 => Material::DiamondAxe,
        280 => Material::Stick,
        281 => Material::Bowl,
        282 => Material::MushroomSoup,
        283 => Material::GoldSword,
        284 => Material::GoldSpade,
        285 => Material::GoldPickaxe,
        286 => Material::GoldAxe,
        287 => Material::String,
        288 => Material::Feather,
        289 => Material::Sulphur,
        290 => Material::WoodHoe,
        291 => Material::StoneHoe,
        292 => Material::IronHoe,
        293 => Material::DiamondHoe,
        294 => Material::GoldHoe,
        295 => Material::Seeds,
        296 => Material::Wheat,
        297 => Material::Bread,
        298 => Material::LeatherHelmet,
        299 => Material::LeatherChestplate,
        300 => Material::LeatherLeggings,
        301 => Material::LeatherBoots,
        302 => Material::ChainmailHelmet,
        303 => Material::ChainmailChestplate,
        304 => Material::ChainmailLeggings,
        305 => Material::ChainmailBoots,
        306 => Material::IronHelmet,
        307 => Material::IronChestplate,
        308 => Material::IronLeggings,
        309 => Material::IronBoots,
        310 => Material::DiamondHelmet,
        311 => Material::DiamondChestplate,
        312 => Material::DiamondLeggings,
        313 => Material::DiamondBoots,
        314 => Material::GoldHelmet,
        315 => Material::GoldChestplate,
        316 => Material::GoldLeggings,
        317 => Material::GoldBoots,
        318 => Material::Flint,
        319 => Material::Pork,
        320 => Material::GrilledPork,
        321 => Material::Painting,
        322 => Material::GoldenApple,
        323 => Material::Sign,
        324 => Material::WoodDoor,
        325 => Material::Bucket,
        326 => Material::WaterBucket,
        327 => Material::LavaBucket,
        328 => Material::Minecart,
        329 => Material::Saddle,
        330 => Material::IronDoor,
        331 => Material::Redstone,
        332 => Material::SnowBall,
        333 => Material::Boat,
        334 => Material::Leather,
        335 => Material::MilkBucket,
        336 => Material::ClayBrick,
        337 => Material::ClayBall,
        338 => Material::SugarCane,
        339 => Material::Paper,
        340 => Material::Book,
        341 => Material::SlimeBall,
        342 => Material::StorageMinecart,
        343 => Material::PoweredMinecart,
        344 => Material::Egg,
        345 => Material::Compass,
        346 => Material::FishingRod,
        347 => Material::Watch,
        348 => Material::GlowstoneDust,
        349 => Material::RawFish,
        350 => Material::CookedFish,
        351 => Material::InkSack,
        352 => Material::Bone,
        353 => Material::Sugar,
        354 => Material::Cake,
        355 => Material::Bed,
        356 => Material::Diode,
        357 => Material::Cookie,
        358 => Material::Map,
        359 => Material::Shears,
        360 => Material::Melon,
        361 => Material::PumpkinSeeds,
        362 => Material::MelonSeeds,
        363 => Material::RawBeef,
        364 => Material::CookedBeef,
        365 => Material::RawChicken,
        366 => Material::CookedChicken,
        367 => Material::RottenFlesh,
        368 => Material::EnderPearl,
        369 => Material::BlazeRod,
        370 => Material::GhastTear,
        371 => Material::GoldNugget,
        372 => Material::NetherStalk,
        373 => Material::Potion,
        374 => Material::GlassBottle,
        375 => Material::SpiderEye,
        376 => Material::FermentedSpiderEye,
        377 => Material::BlazePowder,
        378 => Material::MagmaCream,
        379 => Material::BrewingStandItem,
        380 => Material::CauldronItem,
        381 => Material::EyeOfEnder,
        382 => Material::SpeckledMelon,
        383 => Material::MonsterEgg,
        384 => Material::ExpBottle,
        385 => Material::Fireball,
        386 => Material::BookAndQuill,
        387 => Material::WrittenBook,
        388 => Material::Emerald,
        389 => Material::ItemFrame,
        390 => Material::FlowerPotItem,
        391 => Material::CarrotItem,
        392 => Material::PotatoItem,
        393 => Material::BakedPotato,
        394 => Material::PoisonousPotato,
        395 => Material::EmptyMap,
        396 => Material::GoldenCarrot,
        397 => Material::SkullItem,
        398 => Material::CarrotStick,
        399 => Material::NetherStar,
        400 => Material::PumpkinPie,
        401 => Material::Firework,
        402 => Material::FireworkCharge,
        403 => Material::EnchantedBook,
        404 => Material::RedstoneComparator,
        405 => Material::NetherBrickItem,
        406 => Material::Quartz,
        407 => Material::ExplosiveMinecart,
        408 => Material::HopperMinecart,
        409 => Material::PrismarineShard,
        410 => Material::PrismarineCrystals,
        411 => Material::Rabbit,
        412 => Material::CookedRabbit,
        413 => Material::RabbitStew,
        414 => Material::RabbitFoot,
        415 => Material::RabbitHide,
        416 => Material::ArmorStand,
        417 => Material::IronBarding,
        418 => Material::GoldBarding,
        419 => Material::DiamondBarding,
        420 => Material::Leash,
        421 => Material::NameTag,
        422 => Material::CommandMinecart,
        423 => Material::Mutton,
        424 => Material::CookedMutton,
        425 => Material::Banner,
        426 => Material::EndCrystal,
        427 => Material::SpruceDoorItem,
        428 => Material::BirchDoorItem,
        429 => Material::JungleDoorItem,
        430 => Material::AcaciaDoorItem,
        431 => Material::DarkOakDoorItem,
        432 => Material::ChorusFruit,
        433 => Material::ChorusFruitPopped,
        434 => Material::Beetroot,
        435 => Material::BeetrootSeeds,
        436 => Material::BeetrootSoup,
        437 => Material::DragonsBreath,
        438 => Material::SplashPotion,
        439 => Material::SpectralArrow,
        440 => Material::TippedArrow,
        441 => Material::LingeringPotion,
        442 => Material::Shield,
        443 => Material::Elytra,
        444 => Material::BoatSpruce,
        445 => Material::BoatBirch,
        446 => Material::BoatJungle,
        447 => Material::BoatAcacia,
        448 => Material::BoatDarkOak,
        2256 => Material::GoldRecord,
        2257 => Material::GreenRecord,
        2258 => Material::Record3,
        2259 => Material::Record4,
        2260 => Material::Record5,
        2261 => Material::Record6,
        2262 => Material::Record7,
        2263 => Material::Record8,
        2264 => Material::Record9,
        2265 => Material::Record10,
        2266 => Material::Record11,
        2267 => Material::Record12,
        _ => Material::Air,
    }
}

pub fn get_stack_size(mat: Material) -> u8 {
    match mat {
        Material::IronShovel => 1,
        Material::IronPickaxe => 1,
        Material::IronAxe => 1,
        Material::FlintAndSteel => 1,
        Material::Bow => 1,
        Material::IronSword => 1,
        Material::WoodenSword => 1,
        Material::WoodenShovel => 1,
        Material::WoodenPickaxe => 1,
        Material::WoodenAxe => 1,
        Material::StoneSword => 1,
        Material::StoneShovel => 1,
        Material::StonePickaxe => 1,
        Material::StoneAxe => 1,
        Material::DiamondSword => 1,
        Material::DiamondShovel => 1,
        Material::DiamondPickaxe => 1,
        Material::DiamondAxe => 1,
        Material::MushroomStew => 1,
        Material::GoldenSword => 1,
        Material::GoldenShovel => 1,
        Material::GoldenPickaxe => 1,
        Material::GoldenAxe => 1,
        Material::WoodenHoe => 1,
        Material::StoneHoe => 1,
        Material::IronHoe => 1,
        Material::DiamondHoe => 1,
        Material::GoldenHoe => 1,
        Material::LeatherHelmet => 1,
        Material::LeatherChestplate => 1,
        Material::LeatherLeggings => 1,
        Material::LeatherBoots => 1,
        Material::ChainmailHelmet => 1,
        Material::ChainmailChestplate => 1,
        Material::ChainmailLeggings => 1,
        Material::ChainmailBoots => 1,
        Material::IronHelmet => 1,
        Material::IronChestplate => 1,
        Material::IronLeggings => 1,
        Material::IronBoots => 1,
        Material::DiamondHelmet => 1,
        Material::DiamondChestplate => 1,
        Material::DiamondLeggings => 1,
        Material::DiamondBoots => 1,
        Material::GoldenHelmet => 1,
        Material::GoldenChestplate => 1,
        Material::GoldenLeggings => 1,
        Material::GoldenBoots => 1,
        Material::Sign => 16,
        Material::Bucket => 16,
        Material::WaterBucket => 1,
        Material::LavaBucket => 1,
        Material::Minecart => 1,
        Material::Saddle => 1,
        Material::Snowball => 16,
        Material::Boat => 1,
        Material::MilkBucket => 1,
        Material::ChestMinecart => 1,
        Material::FurnaceMinecart => 1,
        Material::Egg => 16,
        Material::FishingRod => 1,
        Material::Cake => 1,
        Material::Bed => 1,
        Material::Shears => 1,
        Material::EnderPearl => 16,
        Material::Potion => 1,
        Material::WritableBook => 1,
        Material::WrittenBook => 16,
        Material::CarrotOnAStick => 1,
        Material::EnchantedBook => 1,
        Material::TntMinecart => 1,
        Material::HopperMinecart => 1,
        Material::RabbitStew => 1,
        Material::ArmorStand => 16,
        Material::IronHorseArmor => 1,
        Material::GoldenHorseArmor => 1,
        Material::DiamondHorseArmor => 1,
        Material::CommandBlockMinecart => 1,
        Material::Banner => 16,
        Material::BeetrootSoup => 1,
        Material::SplashPotion => 1,
        Material::LingeringPotion => 1,
        Material::Shield => 1,
        Material::Elytra => 1,
        Material::SpruceBoat => 1,
        Material::BirchBoat => 1,
        Material::JungleBoat => 1,
        Material::AcaciaBoat => 1,
        Material::DarkOakBoat => 1,
        Material::GoldRecord => 1,
        Material::GreenRecord => 1,
        Material::Record3 => 1,
        Material::Record4 => 1,
        Material::Record5 => 1,
        Material::Record6 => 1,
        Material::Record7 => 1,
        Material::Record8 => 1,
        Material::Record9 => 1,
        Material::Record10 => 1,
        Material::Record11 => 1,
        Material::Record12 => 1,
        _ => 64,
    }
}
