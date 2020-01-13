use crate::network::buffer;
use crate::network::types;

pub trait ClientPacket {
    type Pkt;
    fn write(&self) -> buffer::Buffer;
    fn new() -> Self::Pkt;
}

#[derive(Debug)]
pub enum ClientPackets {
    HelloPacket(Hello),
    CreatePacket(Create),
    LoadPacket(Load),
    PongPacket(Pong),
    UpdateAckPacket(UpdateAck),
    MovePacket(Move),
    ShootAckPacket(ShootAck),
    AoEAckPacket(AoEAck),
    GotoAckPacket(GotoAck),
    PlayerShootPacket(PlayerShoot),
    UsePortalPacket(UsePortal),
    PlayerTextPacket(PlayerText),
    ChangeGuildRankPacket(ChangeGuildRank),
    JoinGuildPacket(JoinGuild),
    EscapePacket(Escape),
    CheckCreditsPacket(CheckCredits),
    CancelTradePacket(CancelTrade),
    AcceptTradePacket(AcceptTrade),
    ChangeTradePacket(ChangeTrade),
    RequestTradePacket(RequestTrade),
    GuildInvitePacket(GuildInvite),
    GuildRemovePacket(GuildRemove),
    CreateGuildPacket(CreateGuild),
    ChooseNamePacket(ChooseName),
    EditAccountListPacket(EditAccountList),
    BuyPacket(Buy),
    TeleportPacket(Teleport),
    InvDropPacket(InvDrop),
    InvSwapPacket(InvSwap),
    UseItemPacket(UseItem),
    SetConditionPacket(SetCondition),
    PlayerHitPacket(PlayerHit),
    EnemyHitPacket(EnemyHit),
    OtherHitPacket(OtherHit),
    SquareHitPacket(SquareHit),
    ReSkinPacket(ReSkin),
    PetUpgradeRequestPacket(PetUpgradeRequest),
    ActivePetUpdateSendPacket(ActivePetUpdateSend),
    EnterArenaPacket(EnterArena),
    AcceptArenaDeathPacket(AcceptArenaDeath),
    QuestRedeemPacket(QuestRedeem),
    QuestFetchAskPacket(QuestFetchAsk),
    KeyInfoRequestPacket(KeyInfoRequest),
    LoginRewardSendPacket(LoginRewardSend),
    QuestRoomMessagePacket(QuestRoomMessage),
    PetChangeSkinPacket(PetChangeSkin),
}

impl ClientPackets {
    pub fn write(&self) -> buffer::Buffer {
        match self {
            ClientPackets::UpdateAckPacket(v) => v.write().finalize(super::UPDATEACK),
            ClientPackets::ShootAckPacket(v) => v.write().finalize(super::SHOOTACK),
            ClientPackets::PlayerShootPacket(v) => v.write().finalize(super::PLAYERSHOOT),
            ClientPackets::PongPacket(v) => v.write().finalize(super::PONG),
            ClientPackets::MovePacket(v) => v.write().finalize(super::MOVE),
            ClientPackets::GotoAckPacket(v) => v.write().finalize(super::GOTOACK),
            ClientPackets::AoEAckPacket(v) => v.write().finalize(super::AOEACK),
            ClientPackets::PlayerTextPacket(v) => v.write().finalize(super::PLAYERTEXT),
            ClientPackets::UsePortalPacket(v) => v.write().finalize(super::USEPORTAL),
            ClientPackets::ChangeGuildRankPacket(v) => v.write().finalize(super::CHANGEGUILDRANK),
            ClientPackets::JoinGuildPacket(v) => v.write().finalize(super::JOINGUILD),
            ClientPackets::EscapePacket(v) => v.write().finalize(super::ESCAPE),
            ClientPackets::CheckCreditsPacket(v) => v.write().finalize(super::CHECKCREDITS),
            ClientPackets::CancelTradePacket(v) => v.write().finalize(super::CANCELTRADE),
            ClientPackets::AcceptTradePacket(v) => v.write().finalize(super::ACCEPTTRADE),
            ClientPackets::ChangeTradePacket(v) => v.write().finalize(super::CHANGETRADE),
            ClientPackets::RequestTradePacket(v) => v.write().finalize(super::REQUESTTRADE),
            ClientPackets::GuildInvitePacket(v) => v.write().finalize(super::GUILDINVITE),
            ClientPackets::GuildRemovePacket(v) => v.write().finalize(super::GUILDREMOVE),
            ClientPackets::CreateGuildPacket(v) => v.write().finalize(super::CREATEGUILD),
            ClientPackets::ChooseNamePacket(v) => v.write().finalize(super::CHOOSENAME),
            ClientPackets::EditAccountListPacket(v) => v.write().finalize(super::EDITACCOUNTLIST),
            ClientPackets::BuyPacket(v) => v.write().finalize(super::BUY),
            ClientPackets::TeleportPacket(v) => v.write().finalize(super::TELEPORT),
            ClientPackets::InvDropPacket(v) => v.write().finalize(super::INVDROP),
            ClientPackets::InvSwapPacket(v) => v.write().finalize(super::INVSWAP),
            ClientPackets::UseItemPacket(v) => v.write().finalize(super::USEITEM),
            ClientPackets::SetConditionPacket(v) => v.write().finalize(super::SETCONDITION),
            ClientPackets::PlayerHitPacket(v) => v.write().finalize(super::PLAYERHIT),
            ClientPackets::EnemyHitPacket(v) => v.write().finalize(super::ENEMYHIT),
            ClientPackets::OtherHitPacket(v) => v.write().finalize(super::OTHERHIT),
            ClientPackets::SquareHitPacket(v) => v.write().finalize(super::SQUAREHIT),
            ClientPackets::ReSkinPacket(v) => v.write().finalize(super::RESKIN),
            ClientPackets::PetUpgradeRequestPacket(v) => {
                v.write().finalize(super::PETUPGRADEREQUEST)
            }
            ClientPackets::ActivePetUpdateSendPacket(v) => {
                v.write().finalize(super::ACTIVEPETUPDATESEND)
            }
            ClientPackets::EnterArenaPacket(v) => v.write().finalize(super::ENTERARENA),
            ClientPackets::QuestFetchAskPacket(v) => v.write().finalize(super::QUESTFETCHASK),
            ClientPackets::AcceptArenaDeathPacket(v) => v.write().finalize(super::ACCEPTARENADEATH),
            ClientPackets::QuestRedeemPacket(v) => v.write().finalize(super::QUESTREDEEM),
            ClientPackets::KeyInfoRequestPacket(v) => v.write().finalize(super::KEYINFOREQUEST),
            ClientPackets::LoginRewardSendPacket(v) => v.write().finalize(super::LOGINREWARDSEND),
            ClientPackets::QuestRoomMessagePacket(v) => v.write().finalize(super::QUESTROOMMESSAGE),
            ClientPackets::PetChangeSkinPacket(v) => v.write().finalize(super::PETCHANGESKIN),
            ClientPackets::LoadPacket(v) => v.write().finalize(super::LOAD),
            ClientPackets::CreatePacket(v) => v.write().finalize(super::CREATE),
            ClientPackets::HelloPacket(v) => v.write().finalize(super::HELLO),
        }
    }
}

#[derive(Debug)]
pub struct Hello {
    pub build_version: String,
    pub game_id: i32,
    pub guid: String, //rsa encrypted, b64 encoded
    pub random1: i32,
    pub password: String, //rsa encrypted, b64 encoded
    pub random2: i32,
    pub secret: String, //rsa encrypted, b64 encoded (this is for steam, kong, etc)
    pub key_time: u32,
    pub key: Vec<u8>,
    pub map_json: String,
    pub entry_tag: String,
    pub game_net: String, //rotmg
    pub game_net_user_id: String,
    pub play_platform: String, //rotmg
    pub platform_token: String,
    pub user_token: String,
    pub client_token: String, //XTeP7hERdchV5jrBZEYNebAqDPU6tKU6
}

impl ClientPacket for Hello {
    type Pkt = Hello;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.build_version);
        p.write_i32(self.game_id);
        p.write_string(&self.guid);
        p.write_i32(self.random1);
        p.write_string(&self.password);
        p.write_i32(self.random2);
        p.write_string(&self.secret);
        p.write_u32(self.key_time);
        p.write_u16(self.key.len() as u16);
        for i in self.key.iter() {
            p.write_u8(*i);
        }
        p.write_utf_string(&self.map_json);
        p.write_string(&self.entry_tag);
        p.write_string(&self.game_net);
        p.write_string(&self.game_net_user_id);
        p.write_string(&self.play_platform);
        p.write_string(&self.platform_token);
        p.write_string(&self.user_token);
        p.write_string(&self.client_token);
        p
    }
    fn new() -> Self::Pkt {
        Hello {
            build_version: String::new(),
            game_id: 0,
            guid: String::new(),
            random1: 0,
            password: String::new(),
            random2: 0,
            secret: String::new(),
            key_time: 0,
            key: Vec::new(),
            map_json: String::new(),
            entry_tag: String::new(),
            game_net: String::new(),
            game_net_user_id: String::new(),
            play_platform: String::new(),
            platform_token: String::new(),
            user_token: String::new(),
            client_token: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Create {
    pub class_type: u16,
    pub skin_type: u16,
    pub is_challenger: bool,
}

impl ClientPacket for Create {
    type Pkt = Create;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u16(self.class_type);
        p.write_u16(self.skin_type);
        p.write_bool(self.is_challenger);
        p
    }
    fn new() -> Self::Pkt {
        Create {
            class_type: 0,
            skin_type: 0,
            is_challenger: false,
        }
    }
}

#[derive(Debug)]
pub struct Load {
    pub char_id: i32,
    pub is_from_arena: bool,
    pub is_challenger: bool,
}

impl ClientPacket for Load {
    type Pkt = Load;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.char_id);
        p.write_bool(self.is_from_arena);
        p.write_bool(self.is_challenger);
        p
    }
    fn new() -> Self::Pkt {
        Load {
            char_id: 0,
            is_from_arena: false,
            is_challenger: false,
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub tick_id: i32,
    pub time: i32,
    pub new_position: types::WorldPosition,
    pub records: Vec<types::PositionRecords>,
}

impl ClientPacket for Move {
    type Pkt = Move;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.tick_id);
        p.write_i32(self.time);
        p.write_world_position(&self.new_position);
        p.write_u16(self.records.len() as u16);
        for i in self.records.iter() {
            p.write_position_record(i);
        }
        p
    }
    fn new() -> Self::Pkt {
        Move {
            tick_id: 0,
            time: 0,
            new_position: types::WorldPosition::new(),
            records: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateAck {}

impl ClientPacket for UpdateAck {
    type Pkt = UpdateAck;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        UpdateAck {}
    }
}

#[derive(Debug)]
pub struct Pong {
    pub serial: i32,
    pub time: i32,
}

impl ClientPacket for Pong {
    type Pkt = Pong;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.serial);
        p.write_i32(self.time);
        p
    }
    fn new() -> Self::Pkt {
        Pong { serial: 0, time: 0 }
    }
}

#[derive(Debug)]
pub struct AoEAck {
    pub time: i32,
    pub position: types::WorldPosition,
}

impl ClientPacket for AoEAck {
    type Pkt = AoEAck;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_world_position(&self.position);
        p
    }
    fn new() -> Self::Pkt {
        AoEAck {
            time: 0,
            position: types::WorldPosition::new(),
        }
    }
}

#[derive(Debug)]
pub struct GroundDamage {
    pub time: i32,
    pub position: types::WorldPosition,
}

impl ClientPacket for GroundDamage {
    type Pkt = GroundDamage;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_world_position(&self.position);
        p
    }
    fn new() -> Self::Pkt {
        GroundDamage {
            time: 0,
            position: types::WorldPosition::new(),
        }
    }
}

#[derive(Debug)]
pub struct ShootAck {
    pub time: i32,
}

impl ClientPacket for ShootAck {
    type Pkt = ShootAck;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p
    }
    fn new() -> Self::Pkt {
        ShootAck { time: 0 }
    }
}

#[derive(Debug)]
pub struct GotoAck {
    pub time: i32,
}

impl ClientPacket for GotoAck {
    type Pkt = GotoAck;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p
    }
    fn new() -> Self::Pkt {
        GotoAck { time: 0 }
    }
}

#[derive(Debug)]
pub struct PlayerShoot {
    pub time: i32,
    pub bullet_id: u8,
    pub container_type: i16,
    pub position: types::WorldPosition,
    pub angle: f32,
}

impl ClientPacket for PlayerShoot {
    type Pkt = PlayerShoot;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_u8(self.bullet_id);
        p.write_i16(self.container_type);
        p.write_world_position(&self.position);
        p.write_f32(self.angle);
        p
    }
    fn new() -> Self::Pkt {
        PlayerShoot {
            time: 0,
            bullet_id: 0,
            container_type: 0,
            position: types::WorldPosition::new(),
            angle: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct UsePortal {
    pub object_id: i32,
}

impl ClientPacket for UsePortal {
    type Pkt = UsePortal;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.object_id);
        p
    }
    fn new() -> Self::Pkt {
        UsePortal { object_id: 0 }
    }
}

#[derive(Debug)]
pub struct PlayerText {
    pub message: String,
}

impl ClientPacket for PlayerText {
    type Pkt = PlayerText;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.message);
        p
    }
    fn new() -> Self::Pkt {
        PlayerText {
            message: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct ChangeGuildRank {
    pub name: String,
    pub rank: i32,
}

impl ClientPacket for ChangeGuildRank {
    type Pkt = ChangeGuildRank;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.name);
        p.write_i32(self.rank);
        p
    }
    fn new() -> Self::Pkt {
        ChangeGuildRank {
            name: String::new(),
            rank: 0,
        }
    }
}

#[derive(Debug)]
pub struct JoinGuild {
    pub guild_name: String,
}

impl ClientPacket for JoinGuild {
    type Pkt = JoinGuild;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.guild_name);
        p
    }
    fn new() -> Self::Pkt {
        JoinGuild {
            guild_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Escape {}

impl ClientPacket for Escape {
    type Pkt = Escape;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        Escape {}
    }
}

#[derive(Debug)]
pub struct CheckCredits {}

impl ClientPacket for CheckCredits {
    type Pkt = CheckCredits;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        CheckCredits {}
    }
}

#[derive(Debug)]
pub struct CancelTrade {}

impl ClientPacket for CancelTrade {
    type Pkt = CancelTrade;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        CancelTrade {}
    }
}

#[derive(Debug)]
pub struct AcceptTrade {
    pub my_offers: Vec<bool>,
    pub their_offers: Vec<bool>,
}

impl ClientPacket for AcceptTrade {
    type Pkt = AcceptTrade;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u16(self.my_offers.len() as u16);
        for t in self.my_offers.iter() {
            p.write_bool(*t);
        }
        p.write_u16(self.their_offers.len() as u16);
        for t in self.their_offers.iter() {
            p.write_bool(*t);
        }
        p
    }
    fn new() -> Self::Pkt {
        AcceptTrade {
            my_offers: Vec::new(),
            their_offers: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ChangeTrade {
    pub my_offers: Vec<bool>,
}

impl ClientPacket for ChangeTrade {
    type Pkt = ChangeTrade;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u16(self.my_offers.len() as u16);
        for t in self.my_offers.iter() {
            p.write_bool(*t);
        }
        p
    }
    fn new() -> Self::Pkt {
        ChangeTrade {
            my_offers: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RequestTrade {
    pub player_name: String,
}

impl ClientPacket for RequestTrade {
    type Pkt = RequestTrade;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.player_name);
        p
    }
    fn new() -> Self::Pkt {
        RequestTrade {
            player_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct GuildInvite {
    pub player_name: String,
}

impl ClientPacket for GuildInvite {
    type Pkt = GuildInvite;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.player_name);
        p
    }
    fn new() -> Self::Pkt {
        GuildInvite {
            player_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct GuildRemove {
    pub player_name: String,
}

impl ClientPacket for GuildRemove {
    type Pkt = GuildRemove;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.player_name);
        p
    }
    fn new() -> Self::Pkt {
        GuildRemove {
            player_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct CreateGuild {
    pub guild_name: String,
}

impl ClientPacket for CreateGuild {
    type Pkt = CreateGuild;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.guild_name);
        p
    }
    fn new() -> Self::Pkt {
        CreateGuild {
            guild_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct ChooseName {
    pub my_name: String,
}

impl ClientPacket for ChooseName {
    type Pkt = ChooseName;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.my_name);
        p
    }
    fn new() -> Self::Pkt {
        ChooseName {
            my_name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct EditAccountList {
    pub account_list_id: i32,
    pub add: bool,
    pub object_id: i32,
}

impl ClientPacket for EditAccountList {
    type Pkt = EditAccountList;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.account_list_id);
        p.write_bool(self.add);
        p.write_i32(self.object_id);
        p
    }
    fn new() -> Self::Pkt {
        EditAccountList {
            account_list_id: 0,
            add: false,
            object_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct Buy {
    pub object_id: i32,
    pub quantity: i32,
}

impl ClientPacket for Buy {
    type Pkt = Buy;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.object_id);
        p.write_i32(self.quantity);
        p
    }
    fn new() -> Self::Pkt {
        Buy {
            object_id: 0,
            quantity: 0,
        }
    }
}

#[derive(Debug)]
pub struct Teleport {
    pub object_id: i32,
}

impl ClientPacket for Teleport {
    type Pkt = Teleport;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.object_id);
        p
    }
    fn new() -> Self::Pkt {
        Teleport { object_id: 0 }
    }
}

#[derive(Debug)]
pub struct InvDrop {
    pub slot: types::SlotObjectData,
}

impl ClientPacket for InvDrop {
    type Pkt = InvDrop;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_slot_object(&self.slot);
        p
    }
    fn new() -> Self::Pkt {
        InvDrop {
            slot: types::SlotObjectData::new(),
        }
    }
}

#[derive(Debug)]
pub struct UseItem {
    pub time: i32,
    pub item: types::SlotObjectData,
    pub position: types::WorldPosition,
    pub use_type: u8,
}

impl ClientPacket for UseItem {
    type Pkt = UseItem;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_slot_object(&self.item);
        p.write_world_position(&self.position);
        p.write_u8(self.use_type);
        p
    }
    fn new() -> Self::Pkt {
        UseItem {
            time: 0,
            item: types::SlotObjectData::new(),
            position: types::WorldPosition::new(),
            use_type: 0,
        }
    }
}

#[derive(Debug)]
pub struct InvSwap {
    pub time: i32,
    pub position: types::WorldPosition,
    pub old_slot: types::SlotObjectData,
    pub new_slot: types::SlotObjectData,
}

impl ClientPacket for InvSwap {
    type Pkt = InvSwap;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_world_position(&self.position);
        p.write_slot_object(&self.old_slot);
        p.write_slot_object(&self.new_slot);
        p
    }
    fn new() -> Self::Pkt {
        InvSwap {
            time: 0,
            position: types::WorldPosition::new(),
            old_slot: types::SlotObjectData::new(),
            new_slot: types::SlotObjectData::new(),
        }
    }
}

#[derive(Debug)]
pub struct SetCondition {
    pub condition_effect: u8,
    pub condition_duration: f32,
}

impl ClientPacket for SetCondition {
    type Pkt = SetCondition;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u8(self.condition_effect);
        p.write_f32(self.condition_duration);
        p
    }
    fn new() -> Self::Pkt {
        SetCondition {
            condition_effect: 0,
            condition_duration: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct PlayerHit {
    pub bullet_id: u8,
    pub object_id: i32,
}

impl ClientPacket for PlayerHit {
    type Pkt = PlayerHit;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u8(self.bullet_id);
        p.write_i32(self.object_id);
        p
    }
    fn new() -> Self::Pkt {
        PlayerHit {
            bullet_id: 0,
            object_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct EnemyHit {
    pub time: i32,
    pub bullet_id: u8,
    pub target_id: i32,
    pub killed: bool,
}

impl ClientPacket for EnemyHit {
    type Pkt = EnemyHit;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_u8(self.bullet_id);
        p.write_i32(self.target_id);
        p.write_bool(self.killed);
        p
    }
    fn new() -> Self::Pkt {
        EnemyHit {
            time: 0,
            bullet_id: 0,
            target_id: 0,
            killed: false,
        }
    }
}

#[derive(Debug)]
pub struct OtherHit {
    pub time: i32,
    pub bullet_id: u8,
    pub object_id: i32,
    pub target_id: i32,
}

impl ClientPacket for OtherHit {
    type Pkt = OtherHit;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_u8(self.bullet_id);
        p.write_i32(self.object_id);
        p.write_i32(self.target_id);
        p
    }
    fn new() -> Self::Pkt {
        OtherHit {
            time: 0,
            bullet_id: 0,
            object_id: 0,
            target_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct SquareHit {
    pub time: i32,
    pub bullet_id: u8,
    pub object_id: i32,
}

impl ClientPacket for SquareHit {
    type Pkt = SquareHit;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.time);
        p.write_u8(self.bullet_id);
        p.write_i32(self.object_id);
        p
    }
    fn new() -> Self::Pkt {
        SquareHit {
            time: 0,
            bullet_id: 0,
            object_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct ReSkin {
    pub skin_id: i32,
}

impl ClientPacket for ReSkin {
    type Pkt = ReSkin;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.skin_id);
        p
    }
    fn new() -> Self::Pkt {
        ReSkin { skin_id: 0 }
    }
}

#[derive(Debug)]
pub struct PetUpgradeRequest {
    pub pet_trans_type: u8,
    pub pet_id_one: i32,
    pub pet_id_two: i32,
    pub object_id: i32,
    pub slot: types::SlotObjectData,
    pub payment_type: u8,
}

impl ClientPacket for PetUpgradeRequest {
    type Pkt = PetUpgradeRequest;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u8(self.pet_trans_type);
        p.write_i32(self.pet_id_one);
        p.write_i32(self.pet_id_two);
        p.write_i32(self.object_id);
        p.write_slot_object(&self.slot);
        p.write_u8(self.payment_type);
        p
    }
    fn new() -> Self::Pkt {
        PetUpgradeRequest {
            pet_trans_type: 0,
            pet_id_one: 0,
            pet_id_two: 0,
            object_id: 0,
            slot: types::SlotObjectData::new(),
            payment_type: 0,
        }
    }
}

#[derive(Debug)]
pub struct ActivePetUpdateSend {
    pub command_type: u8,
    pub instance_id: i32,
}

impl ClientPacket for ActivePetUpdateSend {
    type Pkt = ActivePetUpdateSend;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_u8(self.command_type);
        p.write_i32(self.instance_id);
        p
    }
    fn new() -> Self::Pkt {
        ActivePetUpdateSend {
            command_type: 0,
            instance_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct EnterArena {
    pub currency: i32,
}

impl ClientPacket for EnterArena {
    type Pkt = EnterArena;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.currency);
        p
    }
    fn new() -> Self::Pkt {
        EnterArena { currency: 0 }
    }
}

#[derive(Debug)]
pub struct AcceptArenaDeath {}

impl ClientPacket for AcceptArenaDeath {
    type Pkt = AcceptArenaDeath;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        AcceptArenaDeath {}
    }
}

#[derive(Debug)]
pub struct QuestRedeem {
    pub slot: types::SlotObjectData,
}

impl ClientPacket for QuestRedeem {
    type Pkt = QuestRedeem;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_slot_object(&self.slot);
        p
    }
    fn new() -> Self::Pkt {
        QuestRedeem {
            slot: types::SlotObjectData::new(),
        }
    }
}

#[derive(Debug)]
pub struct QuestFetchAsk {}

impl ClientPacket for QuestFetchAsk {
    type Pkt = QuestFetchAsk;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        QuestFetchAsk {}
    }
}

#[derive(Debug)]
pub struct KeyInfoRequest {
    pub item_type: i32,
}

impl ClientPacket for KeyInfoRequest {
    type Pkt = KeyInfoRequest;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.item_type);
        p
    }
    fn new() -> Self::Pkt {
        KeyInfoRequest { item_type: 0 }
    }
}

#[derive(Debug)]
pub struct LoginRewardSend {
    pub claim_key: String, //a b64 encoded string, obtained from https://realmofthemadgodhrd.appspot.com/dailyLogin/fetchCalendar
    pub claim_type: String, // is "consecutive" or "nonconsecutive"
}

impl ClientPacket for LoginRewardSend {
    type Pkt = LoginRewardSend;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_string(&self.claim_key);
        p.write_string(&self.claim_type);
        p
    }
    fn new() -> Self::Pkt {
        LoginRewardSend {
            claim_key: String::new(),
            claim_type: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct QuestRoomMessage {}

impl ClientPacket for QuestRoomMessage {
    type Pkt = QuestRoomMessage;
    fn write(&self) -> buffer::Buffer {
        let p = buffer::new_buffer(5);
        p
    }
    fn new() -> Self::Pkt {
        QuestRoomMessage {}
    }
}

#[derive(Debug)]
pub struct PetChangeSkin {
    pub pet_id: i32,
    pub skin_type: i32,
    pub currency: i32,
}

impl ClientPacket for PetChangeSkin {
    type Pkt = PetChangeSkin;
    fn write(&self) -> buffer::Buffer {
        let mut p = buffer::new();
        p.write_i32(self.pet_id);
        p.write_i32(self.skin_type);
        p.write_i32(self.currency);
        p
    }
    fn new() -> Self::Pkt {
        PetChangeSkin {
            pet_id: 0,
            skin_type: 0,
            currency: 0,
        }
    }
}
