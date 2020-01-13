use crate::client;
use crate::network::packets::client_packets;
use crate::network::packets::client_packets::ClientPacket;
use crate::network::packets::client_packets::ClientPackets;
use crate::network::packets::server_packets;

/* game.rs - Contains the functions for game processing. Packet handling, game-related functions/utils, etc */

impl client::Client {
    pub fn on_failure(&mut self, f: server_packets::Failure) {
        println!("Failure! {0}: {1}", f.failure_id, f.failure_message);
        self.handle_failure(&f);
    }
    pub fn on_map_info(&mut self, mp: server_packets::MapInfo) {
        self.movement.map_width = mp.width;
        self.movement.map_height = mp.height;
        let mut load = client_packets::Load::new();
        load.char_id = self.base.char_id;
        self.current_map = mp.name;
        self.send(ClientPackets::LoadPacket(load).write());
    }
    pub fn on_create_success(&mut self, cs: server_packets::CreateSuccess) {
        self.object_id = cs.object_id;
        self.recon.reset();
        println!(
            "{0} joined {2}! ObjectID: {1}",
            self.base.email, self.object_id, self.current_map
        );
    }
    pub fn on_update(&mut self, u: server_packets::Update) {
        self.send(ClientPackets::UpdateAckPacket(client_packets::UpdateAck::new()).write());
        self.parse_update(&u);
    }
    pub fn on_new_tick(&mut self, nt: server_packets::NewTick) {
        let mut mov = client_packets::Move::new();
        self.time_keeper.last_tick_time = self.time_keeper.current_tick_time;
        self.time_keeper.current_tick_time = self.time_keeper.get_time();
        self.movement.tick_count += 1;
        if self.current_map != "Nexus" {
            self.send(ClientPackets::EscapePacket(client_packets::Escape::new()).write());
            
        }
        self.move_to(self.movement.target_position);
        mov.tick_id = nt.tick_id;
        mov.time = self.time_keeper.get_time();
        mov.new_position = self.movement.current_position;
        self.send(ClientPackets::MovePacket(mov).write());
        self.parse_newtick(&nt);
        self.movement.last_tick_id = nt.tick_id;
    }
    pub fn on_ping(&mut self, p: server_packets::Ping) {
        let mut pong = client_packets::Pong::new();
        pong.serial = p.serial;
        pong.time = self.time_keeper.get_time();
        self.send(ClientPackets::PongPacket(pong).write());
    }
    pub fn on_reconnect(&mut self, r: server_packets::Reconnect) {
        println!("{:?}", r);
        if self.recon.blocking_reconnects == true {
            return;
        }
        if r.host.is_empty() == false {
            self.recon.previous_server = self.recon.current_server.clone();
            self.recon.current_server = r.host;
        }
        println!("Reconnecting to {}", r.name);
        self.queue_recon(r.game_id, r.key, r.key_time as u32);
    }
    pub fn on_allyshoot(&mut self, _a: server_packets::AllyShoot) {}
    pub fn on_goto(&mut self, gt: server_packets::Goto) {
        let mut gack = client_packets::GotoAck::new();
        gack.time = self.time_keeper.get_time();
        self.send(ClientPackets::GotoAckPacket(gack).write());
        if gt.object_id == self.object_id {
            self.movement.current_position = gt.position;
        }
    }
    pub fn on_aoe(&mut self, a: server_packets::AoE) {
        let mut ack = client_packets::AoEAck::new();
        ack.position = self.movement.current_position;
        ack.time = self.time_keeper.get_time();
        if self.movement.current_position.distance_to(&a.position) < a.radius {}
        self.send(ClientPackets::AoEAckPacket(ack).write());
    }
    pub fn on_server_player_shoot(&mut self, s: server_packets::ServerPlayerShoot) {
        if s.owner_id == self.object_id {
            let mut sack = client_packets::ShootAck::new();
            sack.time = self.time_keeper.get_time();
            self.send(ClientPackets::ShootAckPacket(sack).write());
        }
    }
    pub fn on_notification(&mut self, _n: server_packets::Notification) {}
    pub fn on_global_notification(&mut self, _gn: server_packets::GlobalNotification) {}
    pub fn on_enemyshoot(&mut self, _e: server_packets::EnemyShoot) {
        let mut sack = client_packets::ShootAck::new();
        sack.time = self.time_keeper.get_time();
        self.send(ClientPackets::ShootAckPacket(sack).write());
    }
    pub fn on_tradeaccepted(&mut self, _ta: server_packets::TradeAccepted) {}
    pub fn on_tradechanged(&mut self, _tc: server_packets::TradeChanged) {}
    pub fn on_tradestart(&mut self, _ts: server_packets::TradeStart) {}
    pub fn on_tradedone(&mut self, _td: server_packets::TradeDone) {}
    pub fn on_traderequested(&mut self, tr: server_packets::TradeRequested) {
        let mut rt = client_packets::RequestTrade::new();
        println!("{} requests a trade!", &tr.name);
        rt.player_name = tr.name;
        self.send(ClientPackets::RequestTradePacket(rt).write());
    }
    pub fn on_text(&mut self, t: server_packets::Text) {
        match self.handle_text(&t) {
            Some(()) => (),
            None => (),
        };
    }
    pub fn on_invresult(&mut self, _i: server_packets::InvResult) {}
    pub fn on_accountlist(&mut self, _a: server_packets::AccountList) {}
    pub fn on_questobjid(&mut self, _q: server_packets::QuestObjID) {}
    pub fn on_death(&mut self, d: server_packets::Death) {
        println!("{0} died, killed by {1}", self.base.email, d.killed_by);
    }
    pub fn on_damage(&mut self, _d: server_packets::Damage) {}
    pub fn on_showeffect(&mut self, _s: server_packets::ShowEffect) {}
    pub fn on_buyresult(&mut self, _br: server_packets::BuyResult) {}
    pub fn on_nameresult(&mut self, _nr: server_packets::NameResult) {}
    pub fn on_createguildresult(&mut self, _cgr: server_packets::CreateGuildResult) {}
    pub fn on_clientstat(&mut self, _cs: server_packets::ClientStat) {}
    pub fn on_invited_to_guild(&mut self, _itg: server_packets::InvitedToGuild) {}
    pub fn on_loginrewardrecv(&mut self, _lrr: server_packets::LoginRewardRecv) {}
    pub fn on_keyinforesponse(&mut self, _kir: server_packets::KeyInfoResponse) {}
    pub fn on_questredeemresponse(&mut self, _qrr: server_packets::QuestRedeemResponse) {}
    pub fn on_questfetchresponse(&mut self, _qfr: server_packets::QuestFetchResponse) {}
    pub fn on_passwordprompt(&mut self, _pp: server_packets::PasswordPrompt) {}
    pub fn on_reskinunlock(&mut self, _ru: server_packets::ReSkinUnlock) {}
    pub fn on_verifyemail(&mut self, _ve: server_packets::VerifyEmail) {}
    pub fn on_arenadeath(&mut self, _ad: server_packets::ArenaDeath) {}
    pub fn on_imminentarenawave(&mut self, _iaw: server_packets::ImminentArenaWave) {}
    pub fn on_hatchpet(&mut self, _hp: server_packets::HatchPet) {}
    pub fn on_deletepet(&mut self, _dp: server_packets::DeletePet) {}
    pub fn on_evolvepet(&mut self, _ep: server_packets::EvolvePet) {}
    pub fn on_petyardupdate(&mut self, _pyu: server_packets::PetYardUpdate) {}
    pub fn on_newability(&mut self, _na: server_packets::NewAbility) {}
    pub fn on_activepetupdaterecv(&mut self, _apur: server_packets::ActivePetUpdateRecv) {}
    pub fn on_realmheroleft(&mut self, _rh: server_packets::RealmHeroLeft) {}
    pub fn on_playsound(&mut self, _ps: server_packets::PlaySound) {}
    pub fn on_file(&mut self, _f: server_packets::File) {}
    pub fn on_pic(&mut self, _p: server_packets::Pic) {}
}

impl client::Client {
    pub fn handle_text(&mut self, t: &server_packets::Text) -> Option<()> {
        // if t.recipient == self.ign {
            if t.name == "{REDACTED}" { //insert ign of your main
                let mut args = t.message.split_whitespace();
                match args.next().unwrap() {
                    "tiles" => println!("Tiles: {:?}", self.movement.tiles),
                    "drops" => println!("Drops: {:?}", self.goods.drops),
                    "grab" => {
                        match self
                            .objects
                            .get_obj_by_type(args.next()?.parse().unwrap_or(0))
                        {
                            Some(v) => {
                                self.objects.target_object = v.status;
                            }
                            None => (),
                        }
                    }
                    "vault" => {
                        self.queue_recon(-5, Vec::new(), u32::max_value());
                    }
                    "nexus" => {
                        self.send(
                            ClientPackets::EscapePacket(client_packets::Escape::new()).write(),
                        );
                    }
                    "fnexus" => {
                        self.queue_recon(-2, Vec::new(), u32::max_value());
                    }
                    "enter" => {
                        let mut up = client_packets::UsePortal::new();
                        up.object_id = self.objects.stored_object.object_id;
                        self.send(ClientPackets::UsePortalPacket(up).write());
                    }
                    "ping" => {
                        self.send_text(format!("/t {} Pong!", t.name));
                    }
                    "trade" => {
                        self.send_text(format!("/trade {}", t.name));
                    }
                    "stop" => {
                        self.objects.target_object.object_id = self.object_id;
                    }
                    "follow" => match self.objects.get_obj_by_id(t.object_id) {
                        Some(v) => {
                            self.movement.target_position = v.status.position;
                            self.objects.target_object = v.status;
                        }
                        None => (),
                    },
                    "recon" => {
                        self.queue_recon(
                            self.recon.game_id,
                            self.recon.game_key.clone(),
                            self.recon.game_key_time,
                        );
                    }
                    "teleport" => {
                        let mut tp = client_packets::Teleport::new();
                        match args.next() {
                            Some(v) => {
                                tp.object_id = v.parse().unwrap_or(t.object_id);
                            }
                            None => tp.object_id = t.object_id,
                        };
                        //if teleport is on cooldown, will walk towards this object
                        self.movement.target_position = self.objects.get_obj_by_id(t.object_id).unwrap().status.position;
                        self.send(ClientPackets::TeleportPacket(tp).write());
                    }
                    "range" => {
                        println!(
                            "Found: {:?}",
                            self.objects.get_entities_in_range(
                                &self.movement.current_position,
                                self.object_id
                            )
                        );
                    }
                    "kick" => {
                        let player_to_kick = args.next()?;
                        let mut gr = client_packets::GuildRemove::new();
                        gr.player_name = player_to_kick.to_owned();
                        self.send(ClientPackets::GuildRemovePacket(gr).write());
                    }
                    "rank" => {
                        let player_to_rank = args.next()?;
                        let mut cgr = client_packets::ChangeGuildRank::new();
                        cgr.name = player_to_rank.to_owned();
                        cgr.rank = args.next()?.parse().unwrap_or(0);
                        self.send(ClientPackets::ChangeGuildRankPacket(cgr).write());
                    }
                    _ => (),
                }
            }
        // }
        None
    }
    pub fn handle_failure(&mut self, _f: &server_packets::Failure) {}
}
