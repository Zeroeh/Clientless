use std::collections::HashMap;

// extern crate fnv;

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct GroundTile {
    pub x: i16,
    pub y: i16,
    pub tile_type: u16,
}

impl std::cmp::PartialEq for GroundTile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

impl WorldPosition {
    pub fn new() -> WorldPosition {
        WorldPosition { x: 0.0, y: 0.0 }
    }
    pub fn new_fill(x: f32, y: f32) -> WorldPosition {
        WorldPosition { x: x, y: y }
    }
}

#[derive(Debug, Clone)]
pub struct StatData {
    pub stat_type: u8,
    pub stat_value: i32,
    pub str_stat_value: String,
}

impl StatData {
    pub fn new() -> StatData {
        StatData {
            stat_type: 0,
            stat_value: 0,
            str_stat_value: String::new(),
        }
    }
    pub fn update_type(&mut self, value: u8) {
        self.stat_type = value;
    }
    pub fn update_value(&mut self, value: i32) {
        self.stat_value = value;
    }
    pub fn update_string(&mut self, value: String) {
        self.str_stat_value = value;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PositionRecords {
    pub time: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct SlotObjectData {
    pub object_id: i32,
    pub slot_id: u8,
    pub object_type: i32,
}

impl SlotObjectData {
    pub fn new() -> SlotObjectData {
        SlotObjectData {
            object_id: 0,
            slot_id: 0,
            object_type: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TradeItem {
    pub item: i32,
    pub slot_type: i32,
    pub tradeable: bool,
    pub included: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct MoveRecord {
    pub time: i32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct ObjectStatusData {
    pub object_id: i32,
    pub position: WorldPosition,
    pub stats: HashMap<u8, StatData>,
}

impl ObjectStatusData {
    pub fn new() -> ObjectStatusData {
        ObjectStatusData {
            object_id: 0,
            position: WorldPosition::new(),
            stats: HashMap::with_capacity(101), //we know how many stats can be in here
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectData {
    pub object_type: u16,
    pub status: ObjectStatusData,
}

impl WorldPosition {
    pub fn sq_distance_to(&self, target: &WorldPosition) -> f32 {
        let x = target.x - self.x;
        let y = target.y - self.y;
        x * x + y * y
    }
    pub fn distance_to(&self, target: &WorldPosition) -> f32 {
        f32::sqrt(self.sq_distance_to(target))
    }
    pub fn angle_to(&self, target: &WorldPosition) -> f32 {
        f32::atan2(target.y - self.y, target.x - self.x)
    }
    pub fn out_of_bounds(&self, width: f32) -> bool {
        self.x < 0.0 || self.y < 0.0 || self.x > width || self.y > width
    }
}

impl StatData {
    pub fn stat_to_u8(self, s: Stats) -> u8 {
        s as u8
    }
    /// checks the stat if it's a string stat
    pub fn is_string_stat(&self) -> bool {
        match Stats::u8_to_stat(self.stat_type) {
            Stats::NAME => return true,
            Stats::ACCOUNTID => return true,
            Stats::OWNERACCOUNTID => return true,
            Stats::GUILDNAME => return true,
            Stats::PETNAME => return true,
            _ => return false,
        };
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum Stats {
    MAXIMUMHP = 0u8,
    HP = 1u8,
    SIZE = 2u8,
    MAXIMUMMP = 3u8,
    MP = 4u8,
    NEXTLEVELEXPERIENCE = 5u8,
    EXPERIENCE = 6u8,
    LEVEL = 7u8,
    INVENTORY0 = 8u8, //gear
    INVENTORY1 = 9u8,
    INVENTORY2 = 10u8,
    INVENTORY3 = 11u8,
    INVENTORY4 = 12u8, //inventory
    INVENTORY5 = 13u8,
    INVENTORY6 = 14u8,
    INVENTORY7 = 15u8,
    INVENTORY8 = 16u8,
    INVENTORY9 = 17u8,
    INVENTORY10 = 18u8,
    INVENTORY11 = 19u8,
    ATTACK = 20u8,
    DEFENSE = 21u8,
    SPEED = 22u8,
    PLACEHOLDER1 = 23u8,
    PLACEHOLDER2 = 24u8,
    PLACEHOLDER3 = 25u8,
    VITALITY = 26u8,
    WISDOM = 27u8,
    DEXTERITY = 28u8,
    EFFECTS = 29u8,
    STARS = 30u8,
    NAME = 31u8, //string
    TEXTURE1 = 32u8,
    TEXTURE2 = 33u8,
    MERCHANDISETYPE = 34u8,
    CREDITS = 35u8,
    MERCHANDISEPRICE = 36u8,
    PORTALUSABLE = 37u8,
    ACCOUNTID = 38u8, //string
    ACCOUNTFAME = 39u8,
    MERCHANDISECURRENCY = 40u8,
    OBJECTCONNECTION = 41u8,
    MERCHANDISEREMAININGCOUNT = 42u8,
    MERCHANDISEREMAININGMINUTES = 43u8,
    MERCHANDISEDISCOUNT = 44u8,
    MERCHANDISERANKREQUIREMENT = 45u8,
    HEALTHBONUS = 46u8,
    MANABONUS = 47u8,
    ATTACKBONUS = 48u8,
    DEFENSEBONUS = 49u8,
    SPEEDBONUS = 50u8,
    VITALITYBONUS = 51u8,
    WISDOMBONUS = 52u8,
    DEXTERITYBONUS = 53u8,
    OWNERACCOUNTID = 54u8, //string
    RANKREQUIRED = 55u8,
    NAMECHOSEN = 56u8,
    CHARACTERFAME = 57u8,
    CHARACTERFAMEGOAL = 58u8,
    GLOWING = 59u8,
    SINKLEVEL = 60u8,
    ALTTEXTUREINDEX = 61u8,
    GUILDNAME = 62u8, //string
    GUILDRANK = 63u8,
    OXYGENBAR = 64u8,
    XPBOOSTERACTIVE = 65u8,
    XPBOOSTTIME = 66u8,
    LOOTDROPBOOSTTIME = 67u8,
    LOOTTIERBOOSTTIME = 68u8,
    HEALTHPOTIONCOUNT = 69u8,
    MAGICPOTIONCOUNT = 70u8,
    BACKPACK0 = 71u8,
    BACKPACK1 = 72u8,
    BACKPACK2 = 73u8,
    BACKPACK3 = 74u8,
    BACKPACK4 = 75u8,
    BACKPACK5 = 76u8,
    BACKPACK6 = 77u8,
    BACKPACK7 = 78u8,
    HASBACKPACK = 79u8,
    SKIN = 80u8,
    PETINSTANCEID = 81u8,
    PETNAME = 82u8, //string
    PETTYPE = 83u8,
    PETRARITY = 84u8,
    PETMAXIMUMLEVEL = 85u8,
    PETFAMILY = 86u8,
    PETPOINTS0 = 87u8,
    PETPOINTS1 = 88u8,
    PETPOINTS2 = 89u8,
    PETLEVEL0 = 90u8,
    PETLEVEL1 = 91u8,
    PETLEVEL2 = 92u8,
    PETABILITYTYPE0 = 93u8,
    PETABILITYTYPE1 = 94u8,
    PETABILITYTYPE2 = 95u8,
    EFFECTS2 = 96u8, //curse, petrify, all the new stats
    FORTUNETOKENS = 97u8,
    SUPPORTERPOINTS = 98u8,
    SUPPORTER = 99u8,
    CHALLENGERSTARBGSTAT = 100u8,
    SOMETHING = 101u8,
    UNKNOWN,
}

impl Stats {
    pub fn stat_to_u8(&self) -> u8 {
        match self {
            Stats::MAXIMUMHP => 0,
            Stats::HP => 1,
            Stats::SIZE => 2,
            Stats::MAXIMUMMP => 3,
            Stats::MP => 4,
            Stats::NEXTLEVELEXPERIENCE => 5,
            Stats::EXPERIENCE => 6,
            Stats::LEVEL => 7,
            Stats::INVENTORY0 => 8, //gear
            Stats::INVENTORY1 => 9,
            Stats::INVENTORY2 => 10,
            Stats::INVENTORY3 => 11,
            Stats::INVENTORY4 => 12, //inventory
            Stats::INVENTORY5 => 13,
            Stats::INVENTORY6 => 14,
            Stats::INVENTORY7 => 15,
            Stats::INVENTORY8 => 16,
            Stats::INVENTORY9 => 17,
            Stats::INVENTORY10 => 18,
            Stats::INVENTORY11 => 19,
            Stats::ATTACK => 20,
            Stats::DEFENSE => 21,
            Stats::SPEED => 22,
            Stats::PLACEHOLDER1 => 23,
            Stats::PLACEHOLDER2 => 24,
            Stats::PLACEHOLDER3 => 25,
            Stats::VITALITY => 26,
            Stats::WISDOM => 27,
            Stats::DEXTERITY => 28,
            Stats::EFFECTS => 29,
            Stats::STARS => 30,
            Stats::NAME => 31, //string
            Stats::TEXTURE1 => 32,
            Stats::TEXTURE2 => 33,
            Stats::MERCHANDISETYPE => 34,
            Stats::CREDITS => 35,
            Stats::MERCHANDISEPRICE => 36,
            Stats::PORTALUSABLE => 37,
            Stats::ACCOUNTID => 38, //string
            Stats::ACCOUNTFAME => 39,
            Stats::MERCHANDISECURRENCY => 40,
            Stats::OBJECTCONNECTION => 41,
            Stats::MERCHANDISEREMAININGCOUNT => 42,
            Stats::MERCHANDISEREMAININGMINUTES => 43,
            Stats::MERCHANDISEDISCOUNT => 44,
            Stats::MERCHANDISERANKREQUIREMENT => 45,
            Stats::HEALTHBONUS => 46,
            Stats::MANABONUS => 47,
            Stats::ATTACKBONUS => 48,
            Stats::DEFENSEBONUS => 49,
            Stats::SPEEDBONUS => 50,
            Stats::VITALITYBONUS => 51,
            Stats::WISDOMBONUS => 52,
            Stats::DEXTERITYBONUS => 53,
            Stats::OWNERACCOUNTID => 54, //string
            Stats::RANKREQUIRED => 55,
            Stats::NAMECHOSEN => 56,
            Stats::CHARACTERFAME => 57,
            Stats::CHARACTERFAMEGOAL => 58,
            Stats::GLOWING => 59,
            Stats::SINKLEVEL => 60,
            Stats::ALTTEXTUREINDEX => 61,
            Stats::GUILDNAME => 62, //string
            Stats::GUILDRANK => 63,
            Stats::OXYGENBAR => 64,
            Stats::XPBOOSTERACTIVE => 65,
            Stats::XPBOOSTTIME => 66,
            Stats::LOOTDROPBOOSTTIME => 67,
            Stats::LOOTTIERBOOSTTIME => 68,
            Stats::HEALTHPOTIONCOUNT => 69,
            Stats::MAGICPOTIONCOUNT => 70,
            Stats::BACKPACK0 => 71,
            Stats::BACKPACK1 => 72,
            Stats::BACKPACK2 => 73,
            Stats::BACKPACK3 => 74,
            Stats::BACKPACK4 => 75,
            Stats::BACKPACK5 => 76,
            Stats::BACKPACK6 => 77,
            Stats::BACKPACK7 => 78,
            Stats::HASBACKPACK => 79,
            Stats::SKIN => 80,
            Stats::PETINSTANCEID => 81,
            Stats::PETNAME => 82, //string
            Stats::PETTYPE => 83,
            Stats::PETRARITY => 84,
            Stats::PETMAXIMUMLEVEL => 85,
            Stats::PETFAMILY => 86,
            Stats::PETPOINTS0 => 87,
            Stats::PETPOINTS1 => 88,
            Stats::PETPOINTS2 => 89,
            Stats::PETLEVEL0 => 90,
            Stats::PETLEVEL1 => 91,
            Stats::PETLEVEL2 => 92,
            Stats::PETABILITYTYPE0 => 93,
            Stats::PETABILITYTYPE1 => 94,
            Stats::PETABILITYTYPE2 => 95,
            Stats::EFFECTS2 => 96, //curse, petrify, all the new stats
            Stats::FORTUNETOKENS => 97,
            Stats::SUPPORTERPOINTS => 98,
            Stats::SUPPORTER => 99,
            Stats::CHALLENGERSTARBGSTAT => 100,
            Stats::SOMETHING => 101,
            Stats::UNKNOWN => 255,
        }
    }
    pub fn u8_to_stat(v: u8) -> Stats {
        match v {
            0 => Stats::MAXIMUMHP,
            1 => Stats::HP,
            2 => Stats::SIZE,
            3 => Stats::MAXIMUMMP,
            4 => Stats::MP,
            5 => Stats::NEXTLEVELEXPERIENCE,
            6 => Stats::EXPERIENCE,
            7 => Stats::LEVEL,
            8 => Stats::INVENTORY0, //gear
            9 => Stats::INVENTORY1,
            10 => Stats::INVENTORY2,
            11 => Stats::INVENTORY3,
            12 => Stats::INVENTORY4, //inventory
            13 => Stats::INVENTORY5,
            14 => Stats::INVENTORY6,
            15 => Stats::INVENTORY7,
            16 => Stats::INVENTORY8,
            17 => Stats::INVENTORY9,
            18 => Stats::INVENTORY10,
            19 => Stats::INVENTORY11,
            20 => Stats::ATTACK,
            21 => Stats::DEFENSE,
            22 => Stats::SPEED,
            23 => Stats::PLACEHOLDER1,
            24 => Stats::PLACEHOLDER2,
            25 => Stats::PLACEHOLDER3,
            26 => Stats::VITALITY,
            27 => Stats::WISDOM,
            28 => Stats::DEXTERITY,
            29 => Stats::EFFECTS,
            30 => Stats::STARS,
            31 => Stats::NAME,
            32 => Stats::TEXTURE1,
            33 => Stats::TEXTURE2,
            34 => Stats::MERCHANDISETYPE,
            35 => Stats::CREDITS,
            36 => Stats::MERCHANDISEPRICE,
            37 => Stats::PORTALUSABLE,
            38 => Stats::ACCOUNTID,
            39 => Stats::ACCOUNTFAME,
            40 => Stats::MERCHANDISECURRENCY,
            41 => Stats::OBJECTCONNECTION,
            42 => Stats::MERCHANDISEREMAININGCOUNT,
            43 => Stats::MERCHANDISEREMAININGMINUTES,
            44 => Stats::MERCHANDISEDISCOUNT,
            45 => Stats::MERCHANDISERANKREQUIREMENT,
            46 => Stats::HEALTHBONUS,
            47 => Stats::MANABONUS,
            48 => Stats::ATTACKBONUS,
            49 => Stats::DEFENSEBONUS,
            50 => Stats::SPEEDBONUS,
            51 => Stats::VITALITYBONUS,
            52 => Stats::WISDOMBONUS,
            53 => Stats::DEXTERITYBONUS,
            54 => Stats::OWNERACCOUNTID,
            55 => Stats::RANKREQUIRED,
            56 => Stats::NAMECHOSEN,
            57 => Stats::CHARACTERFAME,
            58 => Stats::CHARACTERFAMEGOAL,
            59 => Stats::GLOWING,
            60 => Stats::SINKLEVEL,
            61 => Stats::ALTTEXTUREINDEX,
            62 => Stats::GUILDNAME,
            63 => Stats::GUILDRANK,
            64 => Stats::OXYGENBAR,
            65 => Stats::XPBOOSTERACTIVE,
            66 => Stats::XPBOOSTTIME,
            67 => Stats::LOOTDROPBOOSTTIME,
            68 => Stats::LOOTTIERBOOSTTIME,
            69 => Stats::HEALTHPOTIONCOUNT,
            70 => Stats::MAGICPOTIONCOUNT,
            71 => Stats::BACKPACK0,
            72 => Stats::BACKPACK1,
            73 => Stats::BACKPACK2,
            74 => Stats::BACKPACK3,
            75 => Stats::BACKPACK4,
            76 => Stats::BACKPACK5,
            77 => Stats::BACKPACK6,
            78 => Stats::BACKPACK7,
            79 => Stats::HASBACKPACK,
            80 => Stats::SKIN,
            81 => Stats::PETINSTANCEID,
            82 => Stats::PETNAME,
            83 => Stats::PETTYPE,
            84 => Stats::PETRARITY,
            85 => Stats::PETMAXIMUMLEVEL,
            86 => Stats::PETFAMILY,
            87 => Stats::PETPOINTS0,
            88 => Stats::PETPOINTS1,
            89 => Stats::PETPOINTS2,
            90 => Stats::PETLEVEL0,
            91 => Stats::PETLEVEL1,
            92 => Stats::PETLEVEL2,
            93 => Stats::PETABILITYTYPE0,
            94 => Stats::PETABILITYTYPE1,
            95 => Stats::PETABILITYTYPE2,
            96 => Stats::EFFECTS2,
            97 => Stats::FORTUNETOKENS,
            98 => Stats::SUPPORTERPOINTS,
            99 => Stats::SUPPORTER,
            100 => Stats::CHALLENGERSTARBGSTAT,
            101 => Stats::SOMETHING,
            _ => Stats::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub enum Effects {
    NOTHING = 0,
    DEAD = 1,
    QUIET = 2,
    WEAK = 3,
    SLOWED = 4,
    SICK = 5,
    DAZED = 6,
    STUNNED = 7,
    BLIND = 8,
    HALLUCINATING = 9,
    DRUNK = 10,
    CONFUSED = 11,
    STUNIMMUNE = 12,
    INVISIBLE = 13,
    PARALYZED = 14,
    SPEEDY = 15,
    BLEEDING = 16,
    ARMORBROKENIMMUNE = 17,
    HEALING = 18,
    DAMAGING = 19,
    BERSERK = 20,
    PAUSED = 21,
    STASIS = 22,
    STASISIMMUNE = 23,
    INVINCIBLE = 24,
    INVULNERABLE = 25,
    ARMORED = 26,
    ARMORBROKEN = 27,
    HEXED = 28,
    NINJASPEEDY = 29,
    UNSTABLE = 30,
    DARKNESS = 31,
    SLOWIMMUNE = 32,
    DAZEIMMUNE = 33,
    PARALYZEIMMUNE = 34,
    PETRIFIED = 35,
    PETRIFIEDIMMUNE = 36,
    PETSTASIS = 37,
    CURSE = 38,
    CURSEIMMUNE = 39,
    HPBOOST = 40,
    MPBOOST = 41,
    ATKBOOST = 42,
    DEFBOOST = 43,
    SPDBOOST = 44,
    VITBOOST = 45,
    WISBOOST = 46,
    DEXBOOST = 47,
    SILENCED = 48,
    EXPOSED = 49,
    ENERGIZED = 50,
    GROUNDDAMAGE = 99,
}

impl Effects {
    pub fn to_byte(&self) -> u8 {
        match self {
            Effects::NOTHING => 0,
            Effects::DEAD => 1,
            Effects::QUIET => 2,
            Effects::WEAK => 3,
            Effects::SLOWED => 4,
            Effects::SICK => 5,
            Effects::DAZED => 6,
            Effects::STUNNED => 7,
            Effects::BLIND => 8,
            Effects::HALLUCINATING => 9,
            Effects::DRUNK => 10,
            Effects::CONFUSED => 11,
            Effects::STUNIMMUNE => 12,
            Effects::INVISIBLE => 13,
            Effects::PARALYZED => 14,
            Effects::SPEEDY => 15,
            Effects::BLEEDING => 16,
            Effects::ARMORBROKENIMMUNE => 17,
            Effects::HEALING => 18,
            Effects::DAMAGING => 19,
            Effects::BERSERK => 20,
            Effects::PAUSED => 21,
            Effects::STASIS => 22,
            Effects::STASISIMMUNE => 23,
            Effects::INVINCIBLE => 24,
            Effects::INVULNERABLE => 25,
            Effects::ARMORED => 26,
            Effects::ARMORBROKEN => 27,
            Effects::HEXED => 28,
            Effects::NINJASPEEDY => 29,
            Effects::UNSTABLE => 30,
            Effects::DARKNESS => 31,
            Effects::SLOWIMMUNE => 32,
            Effects::DAZEIMMUNE => 33,
            Effects::PARALYZEIMMUNE => 34,
            Effects::PETRIFIED => 35,
            Effects::PETRIFIEDIMMUNE => 36,
            Effects::PETSTASIS => 37,
            Effects::CURSE => 38,
            Effects::CURSEIMMUNE => 39,
            Effects::HPBOOST => 40,
            Effects::MPBOOST => 41,
            Effects::ATKBOOST => 42,
            Effects::DEFBOOST => 43,
            Effects::SPDBOOST => 44,
            Effects::VITBOOST => 45,
            Effects::WISBOOST => 46,
            Effects::DEXBOOST => 47,
            Effects::SILENCED => 48,
            Effects::EXPOSED => 49,
            Effects::ENERGIZED => 50,
            Effects::GROUNDDAMAGE => 99,
        }
    }
}
