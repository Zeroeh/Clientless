use std::collections::HashMap;
use std::thread;
use std::time;

use crate::account;
use crate::network;
use crate::network::packets::client_packets;
use crate::network::packets::client_packets::ClientPacket;
use crate::network::packets::client_packets::ClientPackets;
use crate::network::packets::server_packets;
use crate::network::types;
use crate::network::types::Effects;
use crate::network::types::Stats;

/// Client is the main object for bots. Handles more game-intrinsic stuff rather than Account stuff.
/// NOTE: compartmentalize similar fields into separate structs to keep things more decoupled and clean, and prevents Client from getting too bloated
/// TODO: implement Debug derive later. Will probably end up impl'ing custom printer for Client
// #[derive(Clone)]
pub struct Client {
    pub base: account::Account,
    pub game_connection: network::GameConnection,
    pub factory_handle: Option<thread::JoinHandle<()>>,

    pub is_connected: bool,
    pub is_running: bool,
    pub object_id: i32,
    pub ign: String,
    pub current_map: String,
    pub time_keeper: TimeKeeper,
    pub movement: Movement,
    pub recon: ReconnectBase,
    pub stats: StatBase,
    pub combat: CombatBase,
    pub goods: TradeBase,
    pub objects: GameObjects,
    pub config: crate::Config,
}

impl Client {
    /// Creates a new instance of the Client struct
    pub fn new(a: account::Account, c: crate::Config) -> Client {
        Client {
            base: a,
            game_connection: network::GameConnection::new(),
            factory_handle: None,
            is_connected: false,
            is_running: false,
            object_id: -1,
            ign: String::with_capacity(10),
            current_map: String::with_capacity(20),
            time_keeper: TimeKeeper::new(),
            movement: Movement::new(),
            recon: ReconnectBase::new(),
            stats: StatBase::new(),
            combat: CombatBase::new(),
            goods: TradeBase::new(),
            objects: GameObjects::new(),
            config: c,
        }
    }
    /// Kills the client
    pub fn kill_client(&mut self) {
        self.recon.recon_queued = false;
        self.is_running = false;
        self.disconnect();
    }
}

impl Client {
    /// The main loop that the client will be in. Any thread/task splitting should be done before this call.
    pub fn game_loop(&mut self) {
        'z: while self.is_running == true {
            if self.is_connected == false {
                if self.recon.recon_queued == true {
                    if self.recon.check() {
                        println!(
                            "{} is over the reconnect limit, killing client...",
                            self.base.email
                        );
                        self.kill_client();
                        return;
                    }
                    thread::sleep(time::Duration::from_millis(
                        self.config.thread_delay_ms * self.recon.recon_wait_multiplier,
                    ));
                }
                self.connect(self.recon.current_server.clone(), 2050);
                //cool TODO: check if sockets Option is Ok, and set is_connected = true, instead of implied
                self.is_connected = true;
                self.recon.recon_queued = false;
                //todo: increment bots connected
                self.send_hello(
                    self.recon.game_id,
                    self.recon.game_key.clone(),
                    self.recon.game_key_time,
                );
                //at this point, this thread has to reach self.receive() before netfactory starts
                //otherwise this thread will lock up
            }
            'y: while self.is_connected == true {
                if self.recon.recon_queued == true {
                    self.disconnect();
                    break 'y;
                }
                self.receive();
                //Delay so that we dont hit 100% cpu usage, this sleep basically acts as our "FPS" limiter
                // thread::sleep(time::Duration::from_millis(
                //     self.config.thread_delay_ms,
                // ));
                //if this were a normal client with a game interface, we would do our draws here
                //we could also implement some form of mailbox MPI system for multi-system botting.
                //projectile handling will be implemented here later.
            }
            //wait for factory thread to die
            self.factory_handle.take().unwrap().join().unwrap();
        }
        //client is no longer running, maybe clean up resources?
    }
    /// Queues the client to reconnect to the server
    pub fn queue_recon(&mut self, gid: i32, key: Vec<u8>, key_time: u32) {
        if self.recon.blocking_reconnects == false {
            self.recon.recon_queued = true;
            self.recon.game_id = gid;
            self.recon.game_key = key;
            self.recon.game_key_time = key_time;
            self.recon.increment(false);
            self.clear_heaps();
            //do cleanups depending on the module
        }
    }
    /// Parses the update packet
    pub fn parse_update(&mut self, u: &server_packets::Update) {
        for tile in u.tiles.iter() {
            self.movement.tiles.insert(*tile, tile.tile_type);
        }
        for obj in u.new_objs.iter() {
            self.objects
                .entities
                .insert(obj.status.object_id, obj.clone());
            if obj.status.object_id == self.object_id {
                self.movement.current_position = obj.status.position;
                self.movement.target_position = self.movement.current_position; //so we dont go wandering off
                for stat in obj.status.stats.iter() {
                    // println!("Stat: {:?}", stat); //debug
                    if stat.1.stat_type == Stats::NAME.stat_to_u8() {
                        self.ign = stat.1.str_stat_value.clone();
                    }
                    self.stats.stat_map.insert(stat.1.stat_type, stat.1.clone());
                }
            }
        }
        for drop in u.drops.iter() {
            self.objects.entities.remove(drop);
        }
    }
    pub fn parse_newtick(&mut self, nt: &server_packets::NewTick) {
        for obj in nt.statuses.iter() {
            if obj.object_id == self.object_id {
                self.movement.server_position = obj.position;
                for stat in obj.stats.iter() {
                    self.stats.stat_map.insert(stat.1.stat_type, stat.1.clone());
                    // println!("Updated stat: {:?}", stat); //debug
                }
            }
            if obj.object_id == self.objects.target_object.object_id {
                self.movement.target_position = obj.position;
            }
            match self.objects.entities.get_mut(&obj.object_id) {
                Some(v) => {
                    v.status.position = obj.position;
                    for stat in obj.stats.iter() {
                        v.status.stats.insert(stat.1.stat_type, stat.1.clone());
                    }
                }
                None => (),
            }
        }
    }
    pub fn clear_heaps(&mut self) {
        self.objects.entities.clear();
        self.movement.tiles.clear();
        // self.movement.targets.clear();
        self.stats.stat_map.clear();
    }
    pub fn sleep_thread(&self, dur: u64) {
        thread::sleep(time::Duration::from_secs(dur));
    }
}

impl Client {
    pub fn decide_move(&mut self) {
        self.movement.target_position = types::WorldPosition::new_fill(127.5, 183.0);
    }
    /// Moves the client position towards the target position
    pub fn move_to(&mut self, target: types::WorldPosition) {
        if self.movement.is_out_of_bounds(&target) {
            return;
        }
        if self.has_effect(Effects::PAUSED) == true {
            println!("Paused!");
            self.movement.last_position = self.movement.current_position;
            self.movement.current_position = types::WorldPosition::new_fill(
                -99999999999999999999999999999999999999.0,
                -99999999999999999999999999999999999999.0,
            );
            if self.movement.reset_pos == false {
                self.movement.reset_pos = true;
            }
            return; //dont calculate anything else, as we cant move
        } else if self.has_effect(Effects::PAUSED) == true && self.movement.reset_pos == true {
            self.movement.current_position = self.movement.server_position; //sets us back to whatever the server had recorded
            self.movement.reset_pos = false;
            return; //still cant move when coming out of being paused
        }
        let float_current_time = f32::from_bits(self.time_keeper.current_tick_time as u32);
        let float_last_time = f32::from_bits(self.time_keeper.last_tick_time as u32);
        let mut new_position = types::WorldPosition::new();
        let elapsed: f32;
        if float_current_time - float_last_time > 200.0 {
            elapsed = float_current_time - float_last_time;
        } else {
            elapsed = 200.0;
        }
        let step = self.get_move_speed() * elapsed;
        if self.movement.current_position.sq_distance_to(&target) > step * step {
            let angle = self.movement.current_position.angle_to(&target);
            new_position.x = self.movement.current_position.x + angle.cos() * step;
            new_position.y = self.movement.current_position.y + angle.sin() * step;
        } else {
            new_position = target;
        }
        self.movement.last_position = self.movement.current_position;
        self.movement.current_position = new_position;
    }
    /// Gets the clients move speed
    pub fn get_move_speed(&self) -> f32 {
        //todo: get speed mult from tiles
        if self.has_effect(Effects::SLOWED) == true {
            return MIN_MOVE_SPEED;
        }
        let mut move_speed: f32 = MIN_MOVE_SPEED
            + f32::from_bits(self.stats.get_item(Stats::SPEED) as u32) / 75.0
                * (MAX_MOVE_SPEED - MIN_MOVE_SPEED);
        if self.has_effect(Effects::SPEEDY) == true {
            move_speed *= 1.5;
        }
        move_speed * self.movement.move_multiplier
    }
    pub fn get_atk_freq(&self) -> f32 {
        if self.has_effect(Effects::DAZED) == true {
            return MIN_ATK_FREQ;
        }
        let mut atk_freq = MIN_ATK_FREQ
            + f32::from_bits(self.stats.get_item(Stats::DEXTERITY) as u32) / 75.0
                * (MAX_ATK_FREQ - MIN_ATK_FREQ);
        if self.has_effect(Effects::BERSERK) == true {
            atk_freq *= 1.5;
        }
        atk_freq
    }
    pub fn get_atk_mult(&self) -> f32 {
        if self.has_effect(Effects::WEAK) == true {
            return MIN_ATK_MULT;
        }
        let mut atk_mult: f32 = MIN_ATK_MULT
            + f32::from_bits(self.stats.get_item(Stats::ATTACK) as u32) / 75.0
                * (MAX_ATK_MULT - MIN_ATK_MULT);
        if self.has_effect(Effects::DAMAGING) == true {
            atk_mult *= 1.5;
        }
        atk_mult
    }
    /// Check if the client is affected by a status
    pub fn has_effect(&self, status: Effects) -> bool {
        self.stats.has_effect(status)
    }
}

pub fn get_all_targets() -> HashMap<usize, types::WorldPosition> {
    let mut vm: Vec<types::WorldPosition> = Vec::with_capacity(10);
    let mut map: HashMap<usize, types::WorldPosition> = HashMap::with_capacity(10);
    vm.push(types::WorldPosition::new_fill(127.5, 179.0));
    vm.push(types::WorldPosition::new_fill(132.0, 183.0));
    vm.push(types::WorldPosition::new_fill(127.5, 187.0));
    vm.push(types::WorldPosition::new_fill(123.0, 183.0));
    let mut i = 0;
    for x in vm.iter() {
        map.insert(i, *x);
        i += 1;
    }
    map
}

/// Movement struct implements fields and methods for client movement related stuff
#[derive(Debug, Clone)]
pub struct Movement {
    pub current_position: types::WorldPosition,
    pub target_position: types::WorldPosition,
    pub last_position: types::WorldPosition,
    pub server_position: types::WorldPosition,
    pub move_multiplier: f32,
    pub tick_count: u64,
    pub last_tick_id: i32,
    pub reset_pos: bool,
    pub map_width: i32,
    pub map_height: i32,
    pub move_arc: f32,
    pub radius: f32,
    pub target_code: usize,
    pub targets: HashMap<usize, types::WorldPosition>,
    pub tiles: HashMap<types::GroundTile, u16>,
}

impl Movement {
    pub fn new() -> Movement {
        Movement {
            current_position: types::WorldPosition::new(),
            target_position: types::WorldPosition::new(),
            last_position: types::WorldPosition::new(),
            server_position: types::WorldPosition::new(),
            move_multiplier: 0.8,
            tick_count: 0,
            last_tick_id: 0,
            reset_pos: false,
            map_width: 0,
            map_height: 0,
            move_arc: 0.0,
            radius: 4.0,
            target_code: 0,
            targets: get_all_targets(),
            tiles: HashMap::with_capacity(256*256),
        }
    }
    fn is_out_of_bounds(&self, pos: &types::WorldPosition) -> bool {
        pos.x < 0.0 || pos.y < 0.0 || pos.x > self.map_width as f32 || pos.y > self.map_width as f32
    }
    pub fn rotate_clockwise(&mut self, target: types::WorldPosition) {
        self.move_arc += 0.1;
        self.target_position.x = target.x + f32::cos(self.move_arc) * self.radius;
        self.target_position.y = target.y + f32::sin(self.move_arc) * self.radius;
    }
    pub fn rotate_counterclockwise(&mut self, target: types::WorldPosition) {
        self.move_arc += 0.1;
        self.target_position.x = target.x + f32::sin(self.move_arc) * self.radius;
        self.target_position.y = target.y + f32::cos(self.move_arc) * self.radius;
    }
    pub fn next_target(&mut self) {
        match self.targets.get(&self.target_code) {
            Some(v) => {
                self.target_position = *v;
                self.target_code += 1;
            }
            None => self.target_code = 0,
        }
        // println!("Map: {:?}", self.targets); //debugging targets
    }
    pub fn update_targets(&mut self, v: Vec<types::WorldPosition>) {
        let mut i = 0;
        for &x in v.iter() {
            self.targets.insert(i, x);
            i += 1;
        }
    }
}

/// TimeKeeper keeps track of a clients time, ticks, and anything related to time
#[derive(Debug, Clone)]
pub struct TimeKeeper {
    pub startup_time: time::Instant,
    pub last_tick_id: i32,
    pub current_tick_time: i32,
    pub last_tick_time: i32,
    pub swap_speed_ms: i32,
    pub thread_delay_ms: i32,
}

impl TimeKeeper {
    pub fn new() -> TimeKeeper {
        TimeKeeper {
            startup_time: time::Instant::now(),
            last_tick_id: 0,
            current_tick_time: 0,
            last_tick_time: 0,
            swap_speed_ms: 750,
            thread_delay_ms: 2000, //TODO: grab and set from config file
        }
    }
    /// Resets the client instances startup clock (Example: when switching servers)
    pub fn new_clock(&mut self) {
        self.startup_time = time::Instant::now();
    }
    /// Returns the time elapsed in ms since the last initialization of the client timer
    pub fn get_time(&self) -> i32 {
        self.startup_time.elapsed().as_millis() as i32
    }
    pub fn network_time(&self) -> i32 {
        if self.last_tick_id != 0 {
            return 200 * self.last_tick_id as i32;
        }
        return 0;
    }
}

#[derive(Debug, Clone)]
pub struct ReconnectBase {
    pub current_server: String,
    pub previous_server: String,
    pub blocking_reconnects: bool,
    pub recon_queued: bool,
    pub recon_attempts: u32,
    pub recon_wait_multiplier: u64,
    pub recon_allowed_attempts: u32,
    pub game_id: i32,
    pub game_key: Vec<u8>,
    pub game_key_time: u32,
}

impl ReconnectBase {
    pub fn new() -> ReconnectBase {
        ReconnectBase {
            current_server: String::new(),
            previous_server: String::new(),
            blocking_reconnects: false,
            recon_queued: false,
            recon_allowed_attempts: 3,
            recon_attempts: 0,
            recon_wait_multiplier: 1,
            game_id: -2,
            game_key: Vec::new(),
            game_key_time: u32::max_value(),
        }
    }
    pub fn reset(&mut self) {
        self.blocking_reconnects = false;
        self.recon_attempts = 0;
        self.recon_wait_multiplier = 1;
    }
    /// Increments the recon attempts. If w is true, also increments the wait period
    pub fn increment(&mut self, w: bool) {
        if self.recon_attempts > self.recon_allowed_attempts {
            self.recon_queued = false;
        }
        self.recon_attempts += 1;
        if w == true {
            self.recon_wait_multiplier += 1;
        }
    }
    pub fn check(&self) -> bool {
        self.recon_attempts > self.recon_allowed_attempts
    }
}

#[derive(Debug)]
pub struct StatBase {
    pub stat_map: HashMap<u8, types::StatData>,
}

impl StatBase {
    pub fn new() -> StatBase {
        StatBase {
            stat_map: HashMap::with_capacity(102),
        }
    }
    pub fn get_item(&self, item: Stats) -> i32 {
        self.stat_map[&item.stat_to_u8()].stat_value
    }
    pub fn has_effect(&self, status: Effects) -> bool {
        if status.to_byte() > 31 {
            let condition = self.get_item(Stats::EFFECTS2);
            let effect_bit = 1 << (status.to_byte() - 32);
            (condition & effect_bit) == effect_bit
        } else {
            let condition = self.get_item(Stats::EFFECTS);
            let effect_bit = 1 << (status.to_byte() - 1);
            (condition & effect_bit) == effect_bit
        }
    }
}

/// All things having to do with projectiles and combat
#[derive(Debug)]
pub struct CombatBase {
    pub current_bullet_id: u8,
    pub last_attack_time: i32,
}

impl CombatBase {
    pub fn new() -> CombatBase {
        CombatBase {
            current_bullet_id: 0,
            last_attack_time: 0,
        }
    }
    pub fn get_bullet_id(&mut self) -> u8 {
        let bullet = self.current_bullet_id;
        self.current_bullet_id = (self.current_bullet_id + 1) % 128;
        bullet
    }
}

#[derive(Debug)]
pub struct TradeBase {
    pub drops: Vec<i32>,
}

impl TradeBase {
    pub fn new() -> TradeBase {
        TradeBase { drops: Vec::with_capacity(1) }
    }
}

#[derive(Debug)]
pub struct GameObjects {
    pub entities: HashMap<i32, types::ObjectData>,
    pub target_object: types::ObjectStatusData,
    pub stored_object: types::ObjectStatusData,
}

impl GameObjects {
    pub fn new() -> GameObjects {
        GameObjects {
            entities: HashMap::new(),
            target_object: types::ObjectStatusData::new(),
            stored_object: types::ObjectStatusData::new(),
        }
    }
    pub fn get_obj_by_type(&self, t: u16) -> Option<types::ObjectData> {
        for obj in self.entities.iter() {
            if obj.1.object_type == t {
                return Some(obj.1.clone());
            }
        }
        None
    }
    pub fn get_obj_by_id(&self, id: i32) -> Option<types::ObjectData> {
        for obj in self.entities.iter() {
            if obj.1.status.object_id == id {
                return Some(obj.1.clone());
            }
        }
        None
    }
    /// Returns a vec of most gameobjects within 1.0 tiles. The first object is stored.
    pub fn get_entities_in_range(
        &mut self,
        pos: &types::WorldPosition,
        ignore: i32,
    ) -> Vec<types::ObjectData> {
        let mut objs: Vec<types::ObjectData> = Vec::with_capacity(self.entities.len());
        for obj in self.entities.iter() {
            if pos.distance_to(&obj.1.status.position) < 1.0 && &obj.1.status.object_id != &ignore {
                objs.push(obj.1.clone());
            }
        }
        if objs.is_empty() == false {
            self.stored_object = objs[0].status.clone();
        }
        objs
    }
    pub fn get_obj_by_name(&self, name: String) -> Option<types::ObjectStatusData> {
        for obj in self.entities.iter() {
            // obj.1.status.stats[NAME]
            for stat in obj.1.status.stats.iter() {
                if stat.1.is_string_stat() {
                    if stat.1.str_stat_value == name {
                        return Some(obj.1.status.clone());
                    }
                }
            }
        }
        None
    }
}

impl Client {
    pub fn shoot(&mut self, mut angle: f32) {
        if self.has_effect(Effects::STUNNED) || self.has_effect(Effects::PAUSED) {
            return;
        }
        let time = self.time_keeper.get_time();
        //get item from slot0 and get rate of fire
        let attack_period = 1.0 / self.get_atk_freq() * (1.0 / 100.0); //replace 100 with weapon rate of fire
        if time < self.combat.last_attack_time + f32::to_bits(attack_period) as i32 {
            return;
        }
        self.combat.last_attack_time = time;
        let arc_rads = 0.0 / 180.0 * std::f32::consts::PI; //replace 0.0 with items arc
        let mut total_arc = arc_rads * (1.0 - 1.0); //replace 1-1 with item.numprojectiles-1
        if arc_rads <= 0.0 {
            total_arc = 0.0
        }
        angle -= total_arc / 2.0;
        for _ in 0..1 {
            //replace 1 with item.numprojectiles
            let mut ps = client_packets::PlayerShoot::new();
            ps.time = time;
            ps.bullet_id = self.combat.get_bullet_id();
            ps.angle = angle;
            ps.container_type = self.stats.get_item(Stats::INVENTORY0) as i16;
            ps.position = self.movement.current_position;
            ps.position.x += f32::cos(angle) * 0.3;
            ps.position.y += f32::sin(angle) * 0.3;
            self.send(ClientPackets::PlayerShootPacket(ps).write());
            //todo: push projectile onto projectile map
        }
    }
    pub fn send_text(&mut self, text: String) {
        let mut t = client_packets::PlayerText::new();
        t.message = text;
        self.send(ClientPackets::PlayerTextPacket(t).write());
    }
    pub fn track_player(&mut self, name: String) {
        match self.objects.get_obj_by_name(name) {
            Some(v) => println!("Tracked: {:?}", v.position),
            None => (),
        }
    }
}

pub const MIN_MOVE_SPEED: f32 = 0.0041;
pub const MAX_MOVE_SPEED: f32 = 0.00961;
pub const MIN_ATK_MULT: f32 = 0.5;
pub const MAX_ATK_MULT: f32 = 2.0;
pub const MIN_ATK_FREQ: f32 = 0.0015;
pub const MAX_ATK_FREQ: f32 = 0.008;
