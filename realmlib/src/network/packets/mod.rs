pub mod client_packets;
pub mod server_packets;

use crate::client;
use crate::network::buffer;
use server_packets::ServerPacket;

/* mod.rs (packets module) - Module for packet controls, types, methods */

/// This trait implements methods for using an enum to grab the packet type
pub trait Packet {
    type Pkt;
    fn new() -> Self::Pkt;
}

/// Contains all packet types
pub enum Packets {
    ClientPackets(client_packets::ClientPackets),
    ServerPackets(server_packets::ServerPackets),
}

impl client::Client {
    pub fn evaluate_packet(&mut self, mut p: buffer::Buffer) {
        p.index = 5; //prepare the index for reading
        match p.data[4] {
            //comment out any packets that we dont want to handle, or that are buggy
            FAILURE => self.on_failure(server_packets::Failure::new(p)),
            MAPINFO => self.on_map_info(server_packets::MapInfo::new(p)),
            CREATESUCCESS => self.on_create_success(server_packets::CreateSuccess::new(p)),
            UPDATE => self.on_update(server_packets::Update::new(p)),
            NEWTICK => self.on_new_tick(server_packets::NewTick::new(p)),
            PING => self.on_ping(server_packets::Ping::new(p)),
            RECONNECT => self.on_reconnect(server_packets::Reconnect::new(p)),
            ALLYSHOOT => self.on_allyshoot(server_packets::AllyShoot::new(p)),
            ENEMYSHOOT => self.on_enemyshoot(server_packets::EnemyShoot::new(p)),
            INVRESULT => self.on_invresult(server_packets::InvResult::new(p)),
            TRADECHANGED => self.on_tradechanged(server_packets::TradeChanged::new(p)),
            TRADESTART => self.on_tradestart(server_packets::TradeStart::new(p)),
            TRADEACCEPTED => self.on_tradeaccepted(server_packets::TradeAccepted::new(p)),
            TRADEDONE => self.on_tradedone(server_packets::TradeDone::new(p)),
            TRADEREQUESTED => self.on_traderequested(server_packets::TradeRequested::new(p)),
            GOTO => self.on_goto(server_packets::Goto::new(p)),
            QUESTOBJID => self.on_questobjid(server_packets::QuestObjID::new(p)),
            AOE => self.on_aoe(server_packets::AoE::new(p)),
            SERVERPLAYERSHOOT => {
                self.on_server_player_shoot(server_packets::ServerPlayerShoot::new(p))
            }
            TEXT => self.on_text(server_packets::Text::new(p)),

            NOTIFICATION => self.on_notification(server_packets::Notification::new(p)),
            GLOBALNOTIFICATION => {
                self.on_global_notification(server_packets::GlobalNotification::new(p))
            }
            ACCOUNTLIST => self.on_accountlist(server_packets::AccountList::new(p)),
            DAMAGE => self.on_damage(server_packets::Damage::new(p)),
            SHOWEFFECT => self.on_showeffect(server_packets::ShowEffect::new(p)),
            DEATH => self.on_death(server_packets::Death::new(p)),
            BUYRESULT => self.on_buyresult(server_packets::BuyResult::new(p)),
            NAMERESULT => self.on_nameresult(server_packets::NameResult::new(p)),
            CREATEGUILDRESULT => {
                self.on_createguildresult(server_packets::CreateGuildResult::new(p))
            }
            CLIENTSTAT => self.on_clientstat(server_packets::ClientStat::new(p)),
            INVITEDTOGUILD => self.on_invited_to_guild(server_packets::InvitedToGuild::new(p)),
            PLAYSOUND => self.on_playsound(server_packets::PlaySound::new(p)),
            ACTIVEPETUPDATERECV => {
                self.on_activepetupdaterecv(server_packets::ActivePetUpdateRecv::new(p))
            }
            NEWABILITY => self.on_newability(server_packets::NewAbility::new(p)),
            PETYARDUPDATE => self.on_petyardupdate(server_packets::PetYardUpdate::new(p)),
            EVOLVEPET => self.on_evolvepet(server_packets::EvolvePet::new(p)),
            DELETEPET => self.on_deletepet(server_packets::DeletePet::new(p)),
            HATCHPET => self.on_hatchpet(server_packets::HatchPet::new(p)),
            IMMINENTARENAWAVE => {
                self.on_imminentarenawave(server_packets::ImminentArenaWave::new(p))
            }
            ARENADEATH => self.on_arenadeath(server_packets::ArenaDeath::new(p)),
            VERIFYEMAIL => self.on_verifyemail(server_packets::VerifyEmail::new(p)),
            RESKINUNLOCK => self.on_reskinunlock(server_packets::ReSkinUnlock::new(p)),
            PASSWORDPROMPT => self.on_passwordprompt(server_packets::PasswordPrompt::new(p)),
            QUESTFETCHRESPONSE => {
                self.on_questfetchresponse(server_packets::QuestFetchResponse::new(p))
            }
            QUESTREDEEMRESPONSE => {
                self.on_questredeemresponse(server_packets::QuestRedeemResponse::new(p))
            }
            KEYINFORESPONSE => self.on_keyinforesponse(server_packets::KeyInfoResponse::new(p)),
            LOGINREWARDRECV => self.on_loginrewardrecv(server_packets::LoginRewardRecv::new(p)),
            REALMHEROLEFT => self.on_realmheroleft(server_packets::RealmHeroLeft::new(p)),
            FILE => self.on_file(server_packets::File::new(p)),
            PIC => self.on_pic(server_packets::Pic::new(p)),
            _ => {
                println!(
                    "Got fucked packet id: {}. Maybe packets need to be updated?",
                    p.data[4]
                );
            }
        }
    }
    //todo: pass enum
    pub fn write_packet_id() {}
}

pub const FAILURE: u8 = 0;
pub const CREATESUCCESS: u8 = 101;
pub const CREATE: u8 = 61;
pub const PLAYERSHOOT: u8 = 30;
pub const MOVE: u8 = 42;
pub const PLAYERTEXT: u8 = 10;
pub const TEXT: u8 = 44;
pub const SERVERPLAYERSHOOT: u8 = 12;
pub const DAMAGE: u8 = 75;
pub const UPDATE: u8 = 62;
pub const UPDATEACK: u8 = 81;
pub const NOTIFICATION: u8 = 67;
pub const NEWTICK: u8 = 9;
pub const INVSWAP: u8 = 19;
pub const USEITEM: u8 = 11;
pub const SHOWEFFECT: u8 = 13;
pub const HELLO: u8 = 1;
pub const GOTO: u8 = 18;
pub const INVDROP: u8 = 55;
pub const INVRESULT: u8 = 95;
pub const RECONNECT: u8 = 45;
pub const PING: u8 = 8;
pub const PONG: u8 = 31;
pub const MAPINFO: u8 = 92;
pub const LOAD: u8 = 57;
pub const PIC: u8 = 83;
pub const SETCONDITION: u8 = 60;
pub const TELEPORT: u8 = 74;
pub const USEPORTAL: u8 = 47;
pub const DEATH: u8 = 46;
pub const BUY: u8 = 85;
pub const BUYRESULT: u8 = 22;
pub const AOE: u8 = 64;
pub const GROUNDDAMAGE: u8 = 103;
pub const PLAYERHIT: u8 = 90;
pub const ENEMYHIT: u8 = 25;
pub const AOEACK: u8 = 89;
pub const SHOOTACK: u8 = 100;
pub const OTHERHIT: u8 = 20;
pub const SQUAREHIT: u8 = 40;
pub const GOTOACK: u8 = 65;
pub const EDITACCOUNTLIST: u8 = 27;
pub const ACCOUNTLIST: u8 = 99;
pub const QUESTOBJID: u8 = 82;
pub const CHOOSENAME: u8 = 97;
pub const NAMERESULT: u8 = 21;
pub const CREATEGUILD: u8 = 59;
pub const CREATEGUILDRESULT: u8 = 26;
pub const GUILDREMOVE: u8 = 15;
pub const GUILDINVITE: u8 = 104;
pub const ALLYSHOOT: u8 = 49;
pub const ENEMYSHOOT: u8 = 35;
pub const REQUESTTRADE: u8 = 5;
pub const TRADEREQUESTED: u8 = 88;
pub const TRADESTART: u8 = 86;
pub const CHANGETRADE: u8 = 56;
pub const TRADECHANGED: u8 = 28;
pub const ACCEPTTRADE: u8 = 36;
pub const CANCELTRADE: u8 = 91;
pub const TRADEDONE: u8 = 34;
pub const TRADEACCEPTED: u8 = 14;
pub const CLIENTSTAT: u8 = 69;
pub const CHECKCREDITS: u8 = 102;
pub const ESCAPE: u8 = 105;
pub const FILE: u8 = 106;
pub const INVITEDTOGUILD: u8 = 77;
pub const JOINGUILD: u8 = 7;
pub const CHANGEGUILDRANK: u8 = 37;
pub const PLAYSOUND: u8 = 38;
pub const GLOBALNOTIFICATION: u8 = 66;
pub const RESKIN: u8 = 51;
pub const PETUPGRADEREQUEST: u8 = 16;
pub const ACTIVEPETUPDATESEND: u8 = 24;
pub const ACTIVEPETUPDATERECV: u8 = 76;
pub const NEWABILITY: u8 = 41;
pub const PETYARDUPDATE: u8 = 78;
pub const EVOLVEPET: u8 = 87;
pub const DELETEPET: u8 = 4;
pub const HATCHPET: u8 = 23;
pub const ENTERARENA: u8 = 17;
pub const IMMINENTARENAWAVE: u8 = 50;
pub const ARENADEATH: u8 = 68;
pub const ACCEPTARENADEATH: u8 = 80;
pub const VERIFYEMAIL: u8 = 39;
pub const RESKINUNLOCK: u8 = 107;
pub const PASSWORDPROMPT: u8 = 79;
pub const QUESTFETCHASK: u8 = 98;
pub const QUESTREDEEM: u8 = 58;
pub const QUESTFETCHRESPONSE: u8 = 6;
pub const QUESTREDEEMRESPONSE: u8 = 96;
pub const PETCHANGEFORMMSG: u8 = 53;
pub const KEYINFOREQUEST: u8 = 94;
pub const KEYINFORESPONSE: u8 = 63;
pub const LOGINREWARDSEND: u8 = 3;
pub const LOGINREWARDRECV: u8 = 93;
pub const QUESTROOMMESSAGE: u8 = 48;
pub const PETCHANGESKIN: u8 = 33;
pub const REALMHEROLEFT: u8 = 84;
pub const RESETDAILYQUESTS: u8 = 52;
