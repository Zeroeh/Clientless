use crate::network::buffer;
use crate::network::types;

pub trait ServerPacket {
    type Pkt;
    fn new(p: buffer::Buffer) -> Self::Pkt;
}

pub enum ServerPackets {}

#[derive(Debug)]
pub struct Failure {
    pub failure_id: i32,
    pub failure_message: String,
}

impl ServerPacket for Failure {
    type Pkt = Failure;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Failure {
            failure_id: p.read_i32(),
            failure_message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct MapInfo {
    pub width: i32,
    pub height: i32,
    pub name: String,
    pub display_name: String,
    pub realm_name: String,
    pub difficulty: i32,
    pub fp: u32,
    pub background: i32,
    pub allow_player_teleport: bool,
    pub show_displays: bool,
    pub client_xml: Vec<String>,
    pub extra_xml: Vec<String>,
}

impl ServerPacket for MapInfo {
    type Pkt = MapInfo;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let width = p.read_i32();
        let height = p.read_i32();
        let name = p.read_string();
        let display_name = p.read_string();
        let realm_name = p.read_string();
        let difficulty = p.read_i32();
        let fp = p.read_u32();
        let background = p.read_i32();
        let allow_player_teleport = p.read_bool();
        let show_displays = p.read_bool();
        let mut size = p.read_u16();
        let mut client_xml = Vec::new();
        if size == 0 {
            //skip
        } else {
            for _ in 0..size {
                client_xml.push(p.read_utf_string());
            }
        }
        size = p.read_u16();
        let mut extra_xml = Vec::new();
        if size == 0 {
        } else {
            for _ in 0..size {
                extra_xml.push(p.read_utf_string());
            }
        }
        MapInfo {
            width: width,
            height: height,
            name: name,
            display_name: display_name,
            realm_name: realm_name,
            difficulty: difficulty,
            fp: fp,
            background: background,
            allow_player_teleport: allow_player_teleport,
            show_displays: show_displays,
            client_xml: client_xml,
            extra_xml: extra_xml,
        }
    }
}

#[derive(Debug)]
pub struct CreateSuccess {
    pub object_id: i32,
    pub char_id: i32,
}

impl ServerPacket for CreateSuccess {
    type Pkt = CreateSuccess;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        CreateSuccess {
            object_id: p.read_i32(),
            char_id: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Update {
    pub tiles: Vec<types::GroundTile>,
    pub new_objs: Vec<types::ObjectData>,
    pub drops: Vec<i32>,
}

impl ServerPacket for Update {
    type Pkt = Update;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let tile_size = p.read_u16();
        let mut tiles: Vec<types::GroundTile> = Vec::new();
        for _ in 0..tile_size {
            tiles.push(p.read_ground_tile());
        }
        let obj_size = p.read_u16();
        let mut objects: Vec<types::ObjectData> = Vec::new();
        for _ in 0..obj_size {
            objects.push(p.read_object_data());
        }
        let drops_size = p.read_u16();
        let mut drops: Vec<i32> = Vec::new();
        for _ in 0..drops_size {
            drops.push(p.read_i32());
        }
        Update {
            tiles: tiles,
            new_objs: objects,
            drops: drops,
        }
    }
}

#[derive(Debug)]
pub struct NewTick {
    pub tick_id: i32,
    pub tick_time: i32,
    pub statuses: Vec<types::ObjectStatusData>,
}

impl ServerPacket for NewTick {
    type Pkt = NewTick;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let tid = p.read_i32();
        let tim = p.read_i32();
        let size = p.read_u16();
        let mut stats: Vec<types::ObjectStatusData> = Vec::new();
        for _ in 0..size {
            stats.push(p.read_object_status_data());
        }
        NewTick {
            tick_id: tid,
            tick_time: tim,
            statuses: stats,
        }
    }
}

#[derive(Debug)]
pub struct Ping {
    pub serial: i32,
}

impl ServerPacket for Ping {
    type Pkt = Ping;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Ping {
            serial: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Reconnect {
    pub name: String,
    pub host: String,
    pub stats: String,
    pub port: i32,
    pub game_id: i32,
    pub key_time: i32,
    pub is_from_arena: bool,
    pub key: Vec<u8>,
}

impl ServerPacket for Reconnect {
    type Pkt = Reconnect;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let name = p.read_string();
        let host = p.read_string();
        let stats = p.read_string();
        let port = p.read_i32();
        let game_id = p.read_i32();
        let key_time = p.read_i32();
        let is_from_arena = p.read_bool();
        let size = p.read_u16();
        let mut keys = Vec::new();
        for _ in 0..size {
            keys.push(p.read_u8());
        }
        Reconnect {
            name: name,
            host: host,
            stats: stats,
            port: port,
            game_id: game_id,
            key_time: key_time,
            is_from_arena: is_from_arena,
            key: keys,
        }
    }
}

#[derive(Debug)]
pub struct AoE {
    pub position: types::WorldPosition,
    pub radius: f32,
    pub damage: u16,
    pub effects: u8, //types::conditionEffect
    pub effect_duration: f32,
    pub origin_type: i16,
    pub color: i32,
    pub armor_pierce: bool,
}

impl ServerPacket for AoE {
    type Pkt = AoE;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        AoE {
            position: p.read_world_position(),
            radius: p.read_f32(),
            damage: p.read_u16(),
            effects: p.read_u8(),
            effect_duration: p.read_f32(),
            origin_type: p.read_i16(),
            color: p.read_i32(),
            armor_pierce: p.read_bool(),
        }
    }
}

#[derive(Debug)]
pub struct Goto {
    pub object_id: i32,
    pub position: types::WorldPosition,
}

impl ServerPacket for Goto {
    type Pkt = Goto;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Goto {
            object_id: p.read_i32(),
            position: p.read_world_position(),
        }
    }
}

#[derive(Debug)]
pub struct AllyShoot {
    pub bullet_id: u8,
    pub owner_id: i32,
    pub container_type: i16,
    pub angle: f32,
}

impl ServerPacket for AllyShoot {
    type Pkt = AllyShoot;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        AllyShoot {
            bullet_id: p.read_u8(),
            owner_id: p.read_i32(),
            container_type: p.read_i16(),
            angle: p.read_f32(),
        }
    }
}

#[derive(Debug)]
pub struct Text {
    pub name: String,
    pub object_id: i32,
    pub stars: i32,
    pub bubble_time: u8,
    pub recipient: String,
    pub message: String,
    pub clean_message: String,
    pub supporter: bool,
}

impl ServerPacket for Text {
    type Pkt = Text;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Text {
            name: p.read_string(),
            object_id: p.read_i32(),
            stars: p.read_i32(),
            bubble_time: p.read_u8(),
            recipient: p.read_string(),
            message: p.read_string(),
            clean_message: p.read_string(),
            supporter: p.read_bool(),
        }
    }
}

#[derive(Debug)]
pub struct ServerPlayerShoot {
    pub bullet_id: u8,
    pub owner_id: i32,
    pub container_type: i32,
    pub starting_pos: types::WorldPosition,
    pub angle: f32,
    pub damage: i16,
}

impl ServerPacket for ServerPlayerShoot {
    type Pkt = ServerPlayerShoot;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ServerPlayerShoot {
            bullet_id: p.read_u8(),
            owner_id: p.read_i32(),
            container_type: p.read_i32(),
            starting_pos: p.read_world_position(),
            angle: p.read_f32(),
            damage: p.read_i16(),
        }
    }
}

#[derive(Debug)]
pub struct Notification {
    pub object_id: i32,
    pub message: String,
    pub color: i32,
}

impl ServerPacket for Notification {
    type Pkt = Notification;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Notification {
            object_id: p.read_i32(),
            message: p.read_string(),
            color: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalNotification {
    pub type_id: i32,
    pub text: String,
}

impl ServerPacket for GlobalNotification {
    type Pkt = GlobalNotification;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        GlobalNotification {
            type_id: p.read_i32(),
            text: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct EnemyShoot {
    pub bullet_id: u8,
    pub owner_id: i32,
    pub bullet_type: u8,
    pub location: types::WorldPosition,
    pub angle: f32,
    pub damage: i16,
    pub num_shots: u8,
    pub angle_inc: f32,
}

impl ServerPacket for EnemyShoot {
    type Pkt = EnemyShoot;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let bid = p.read_u8();
        let owner = p.read_i32();
        let btype = p.read_u8();
        let loc = p.read_world_position();
        let ang = p.read_f32();
        let dmg = p.read_i16();
        let mut nums = 1;
        let mut anginc = 0.0;
        if p.data[p.index..].len() > 0 {
            nums = p.read_u8();
            anginc = p.read_f32();
        }
        EnemyShoot {
            bullet_id: bid,
            owner_id: owner,
            bullet_type: btype,
            location: loc,
            angle: ang,
            damage: dmg,
            num_shots: nums,
            angle_inc: anginc,
        }
    }
}

#[derive(Debug)]
pub struct AccountList {
    pub account_list_id: i32,
    pub account_ids: Vec<String>,
    pub lock_action: i32,
}

impl ServerPacket for AccountList {
    type Pkt = AccountList;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let acc_list = p.read_i32();
        let size = p.read_u16();
        let mut ids: Vec<String> = Vec::new();
        for _ in 0..size {
            ids.push(p.read_string());
        }
        let laction = p.read_i32();
        AccountList {
            account_list_id: acc_list,
            account_ids: ids,
            lock_action: laction,
        }
    }
}

#[derive(Debug)]
pub struct QuestObjID {
    pub object_id: i32,
}

impl ServerPacket for QuestObjID {
    type Pkt = QuestObjID;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        QuestObjID {
            object_id: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct InvResult {
    pub result: i32,
}

impl ServerPacket for InvResult {
    type Pkt = InvResult;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        InvResult {
            result: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct TradeAccepted {
    pub my_offers: Vec<bool>,
    pub their_offers: Vec<bool>,
}

impl ServerPacket for TradeAccepted {
    type Pkt = TradeAccepted;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let mut moffers = Vec::new();
        let mut toffers = Vec::new();
        let msize = p.read_u16();
        for _ in 0..msize {
            moffers.push(p.read_bool());
        }
        let tsize = p.read_u16();
        for _ in 0..tsize {
            toffers.push(p.read_bool());
        }
        TradeAccepted {
            my_offers: moffers,
            their_offers: toffers,
        }
    }
}

#[derive(Debug)]
pub struct TradeStart {
    pub my_items: Vec<types::TradeItem>,
    pub their_name: String,
    pub their_items: Vec<types::TradeItem>,
}

impl ServerPacket for TradeStart {
    type Pkt = TradeStart;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let mut mitems = Vec::new();
        let mut titems = Vec::new();
        let msize = p.read_u16();
        for _ in 0..msize {
            mitems.push(p.read_trade_item());
        }
        let n = p.read_string();
        let tsize = p.read_u16();
        for _ in 0..tsize {
            titems.push(p.read_trade_item());
        }
        TradeStart {
            my_items: mitems,
            their_name: n,
            their_items: titems,
        }
    }
}

#[derive(Debug)]
pub struct TradeChanged {
    pub their_offers: Vec<bool>,
}

impl ServerPacket for TradeChanged {
    type Pkt = TradeChanged;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let size = p.read_u16();
        let mut toff = Vec::new();
        for _ in 0..size {
            toff.push(p.read_bool());
        }
        TradeChanged { their_offers: toff }
    }
}

/// TradeSuccessful 0
/// PlayerCancelled 1
///	TradeError ?
#[derive(Debug)]
pub struct TradeDone {
    pub result_code: i32,
    pub message: String,
}

impl ServerPacket for TradeDone {
    type Pkt = TradeDone;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        TradeDone {
            result_code: p.read_i32(),
            message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct TradeRequested {
    pub name: String,
}

impl ServerPacket for TradeRequested {
    type Pkt = TradeRequested;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        TradeRequested {
            name: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct Damage {
    pub target_id: i32,
    pub effects: u8,
    pub damage: u16,
    pub killed: bool,
    pub bullet_id: u8,
    pub object_id: i32,
}

impl ServerPacket for Damage {
    type Pkt = Damage;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Damage {
            target_id: p.read_i32(),
            effects: p.read_u8(),
            damage: p.read_u16(),
            killed: p.read_bool(),
            bullet_id: p.read_u8(),
            object_id: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ShowEffect {
    pub effect: u8, //NOT u8!!!
    pub target_id: i32,
    pub pos_a: types::WorldPosition,
    pub pos_b: types::WorldPosition,
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub duration: f32,
}

impl ServerPacket for ShowEffect {
    type Pkt = ShowEffect;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ShowEffect {
            effect: p.read_u8(),
            target_id: p.read_i32(),
            pos_a: p.read_world_position(),
            pos_b: p.read_world_position(),
            alpha: p.read_u8(),
            red: p.read_u8(),
            green: p.read_u8(),
            blue: p.read_u8(),
            duration: p.read_f32(),
        }
    }
}

#[derive(Debug)]
pub struct Pic {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl ServerPacket for Pic {
    type Pkt = Pic;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let w = p.read_i32();
        let h = p.read_i32();
        let s = p.read_u16();
        let mut d = Vec::new();
        for _ in 0..s {
            d.push(p.read_u8());
        }
        Pic {
            width: w,
            height: h,
            data: d,
        }
    }
}

#[derive(Debug)]
pub struct Death {
    pub account_id: String,
    pub char_id: i32,
    pub killed_by: String,
    pub zombie_type: i32,
    pub zombie_id: i32,
}

impl ServerPacket for Death {
    type Pkt = Death;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        Death {
            account_id: p.read_string(),
            char_id: p.read_i32(),
            killed_by: p.read_string(),
            zombie_type: p.read_i32(),
            zombie_id: p.read_i32(),
        }
    }
}

/// Unknown -1
///	Success 0
/// InvalidCharacter 1
///	ItemNotFound 2
///	NotEnoughGold 3
///	InventoryFull 4
/// TooLowRank 5
///	NotEnoughFame 6
///	PetFeedSuccess 7
#[derive(Debug)]
pub struct BuyResult {
    pub result_code: i32,
    pub message: String,
}

impl ServerPacket for BuyResult {
    type Pkt = BuyResult;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        BuyResult {
            result_code: p.read_i32(),
            message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct NameResult {
    pub success: bool,
    pub error_message: String,
}

impl ServerPacket for NameResult {
    type Pkt = NameResult;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        NameResult {
            success: p.read_bool(),
            error_message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct CreateGuildResult {
    pub success: bool,
    pub error_message: String,
}

impl ServerPacket for CreateGuildResult {
    type Pkt = CreateGuildResult;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        CreateGuildResult {
            success: p.read_bool(),
            error_message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct ClientStat {
    pub name: String,
    pub value: i32,
}

impl ServerPacket for ClientStat {
    type Pkt = ClientStat;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ClientStat {
            name: p.read_string(),
            value: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub file_name: String,
    pub data: Vec<u8>,
}

impl ServerPacket for File {
    type Pkt = File;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        let mut d = Vec::new();
        let fnd = p.read_string();
        let s = p.read_u32();
        for _ in 0..s {
            d.push(p.read_u8());
        }
        File {
            file_name: fnd,
            data: d,
        }
    }
}

#[derive(Debug)]
pub struct InvitedToGuild {
    pub inviter_name: String,
    pub guild_name: String,
}

impl ServerPacket for InvitedToGuild {
    type Pkt = InvitedToGuild;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        InvitedToGuild {
            inviter_name: p.read_string(),
            guild_name: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct PlaySound {
    pub owner_id: i32,
    pub sound_id: u8,
}

impl ServerPacket for PlaySound {
    type Pkt = PlaySound;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        PlaySound {
            owner_id: p.read_i32(),
            sound_id: p.read_u8(),
        }
    }
}

#[derive(Debug)]
pub struct ActivePetUpdateRecv {
    pub instance_id: i32,
}

impl ServerPacket for ActivePetUpdateRecv {
    type Pkt = ActivePetUpdateRecv;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ActivePetUpdateRecv {
            instance_id: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct NewAbility {
    pub ability_type: i32,
}

impl ServerPacket for NewAbility {
    type Pkt = NewAbility;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        NewAbility {
            ability_type: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct PetYardUpdate {
    pub yard_type: i32,
}

impl ServerPacket for PetYardUpdate {
    type Pkt = PetYardUpdate;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        PetYardUpdate {
            yard_type: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct EvolvePet {
    pub pet_id: i32,
    pub initial_skin: i32,
    pub final_skin: i32,
}

impl ServerPacket for EvolvePet {
    type Pkt = EvolvePet;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        EvolvePet {
            pet_id: p.read_i32(),
            initial_skin: p.read_i32(),
            final_skin: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct DeletePet {
    pub pet_id: i32,
}

impl ServerPacket for DeletePet {
    type Pkt = DeletePet;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        DeletePet {
            pet_id: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct HatchPet {
    pub pet_name: String,
    pub pet_skin: i32,
    pub item_type: i32,
}

impl ServerPacket for HatchPet {
    type Pkt = HatchPet;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        HatchPet {
            pet_name: p.read_string(),
            pet_skin: p.read_i32(),
            item_type: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ImminentArenaWave {
    pub current_runtime: i32,
}

impl ServerPacket for ImminentArenaWave {
    type Pkt = ImminentArenaWave;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ImminentArenaWave {
            current_runtime: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ArenaDeath {
    pub cost: i32,
}

impl ServerPacket for ArenaDeath {
    type Pkt = ArenaDeath;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ArenaDeath { cost: p.read_i32() }
    }
}

#[derive(Debug)]
pub struct VerifyEmail {}

impl ServerPacket for VerifyEmail {
    type Pkt = VerifyEmail;
    fn new(_p: buffer::Buffer) -> Self::Pkt {
        VerifyEmail {}
    }
}

#[derive(Debug)]
pub struct ReSkinUnlock {
    pub skin_id: i32,
    pub is_pet_skin: i32,
}

impl ServerPacket for ReSkinUnlock {
    type Pkt = ReSkinUnlock;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        ReSkinUnlock {
            skin_id: p.read_i32(),
            is_pet_skin: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct PasswordPrompt {
    pub clean_password_status: i32,
}

impl ServerPacket for PasswordPrompt {
    type Pkt = PasswordPrompt;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        PasswordPrompt {
            clean_password_status: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct QuestFetchResponse {
    pub tier: i32,
    pub goal: String,
    pub description: String,
    pub image: String,
}

impl ServerPacket for QuestFetchResponse {
    type Pkt = QuestFetchResponse;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        QuestFetchResponse {
            tier: p.read_i32(),
            goal: p.read_string(),
            description: p.read_string(),
            image: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct QuestRedeemResponse {
    pub success: bool,
    pub message: String,
}

impl ServerPacket for QuestRedeemResponse {
    type Pkt = QuestRedeemResponse;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        QuestRedeemResponse {
            success: p.read_bool(),
            message: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct KeyInfoResponse {
    pub name: String,
    pub description: String,
    pub creator: String,
}

impl ServerPacket for KeyInfoResponse {
    type Pkt = KeyInfoResponse;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        KeyInfoResponse {
            name: p.read_string(),
            description: p.read_string(),
            creator: p.read_string(),
        }
    }
}

#[derive(Debug)]
pub struct LoginRewardRecv {
    pub item_id: i32,
    pub quantity: i32,
    pub gold: i32,
}

impl ServerPacket for LoginRewardRecv {
    type Pkt = LoginRewardRecv;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        LoginRewardRecv {
            item_id: p.read_i32(),
            quantity: p.read_i32(),
            gold: p.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct RealmHeroLeft {
    pub heroes_left: i32,
}

impl ServerPacket for RealmHeroLeft {
    type Pkt = RealmHeroLeft;
    fn new(mut p: buffer::Buffer) -> Self::Pkt {
        RealmHeroLeft {
            heroes_left: p.read_i32(),
        }
    }
}
