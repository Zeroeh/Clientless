use std::collections::HashMap;

use crate::network::types;

#[derive(Clone, Debug)]
pub struct Buffer {
    pub index: usize,
    pub data: Vec<u8>,
}

pub fn new() -> Buffer {
    Buffer {
        index: 0,
        data: Vec::new(),
    }
}

pub fn new_with_header() -> Buffer {
    Buffer {
        index: 0,
        data: vec![0, 0, 0, 0, 0],
    }
}

pub fn new_buffer(size: i32) -> Buffer {
    Buffer {
        index: 0,
        data: Vec::with_capacity(size as usize),
    }
}

impl Buffer {
    pub fn advance(&mut self, amount: usize) -> usize {
        self.index += amount;
        return amount;
    }
    pub fn reset(&mut self) {
        self.index = 0;
        for x in self.data.iter_mut() {
            *x = 0;
        }
    }
    pub fn finalize(mut self, id: u8) -> Self {
        let idx: u32 = self.index as u32 + 5; //add 5 to account for packet header
        self.index = 0;
        let mut p2 = new();
        p2.write_u32(idx);
        self.data.insert(0, p2.data[0]);
        self.data.insert(1, p2.data[1]);
        self.data.insert(2, p2.data[2]);
        self.data.insert(3, p2.data[3]);
        self.data.insert(4, id);
        self
    }
    pub fn resize(mut self) -> Buffer {
        self.index = 0;
        let size = self.read_u32();
        if size == 0 {
            return self;
        }
        self.index = 0;
        let tmp = self.data.clone();
        self.data.resize(size as usize, 0);
        self.data[0] = tmp[0];
        self.data[1] = tmp[1];
        self.data[2] = tmp[2];
        self.data[3] = tmp[3];
        self.data[4] = tmp[4];
        self
    }
}

const SIZE_BYTE: usize = 1;
const SIZE_SHORT: usize = 2;
const SIZE_INT: usize = 4;
const SIZE_LONG: usize = 8;

impl Buffer {
    // Read functions
    pub fn read_u64(&mut self) -> u64 {
        let s = &self.data[self.index..self.index + SIZE_LONG];
        self.index += SIZE_LONG;
        return (s[7] as u64)
            | (s[6] as u64) << 8
            | (s[5] as u64) << 16
            | (s[4] as u64) << 24
            | (s[3] as u64) << 32
            | (s[2] as u64) << 40
            | (s[1] as u64) << 48
            | (s[0] as u64) << 56;
    }
    pub fn read_u32(&mut self) -> u32 {
        let s = &self.data[self.index..self.index + SIZE_INT];
        self.index += SIZE_INT;
        return (s[3] as u32) | (s[2] as u32) << 8 | (s[1] as u32) << 16 | (s[0] as u32) << 24;
    }
    pub fn read_u16(&mut self) -> u16 {
        let s = &self.data[self.index..self.index + SIZE_SHORT];
        self.index += SIZE_SHORT;
        return (s[1] as u16) | (s[0] as u16) << 8;
    }
    pub fn read_u8(&mut self) -> u8 {
        let s = &self.data[self.index];
        self.index += SIZE_BYTE;
        return *s;
    }
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }
    pub fn read_f64(&mut self) -> f64 {
        return f64::from_bits(self.read_u64());
    }
    pub fn read_f32(&mut self) -> f32 {
        return f32::from_bits(self.read_u32());
    }
    pub fn read_string(&mut self) -> String {
        let size = self.read_u16();
        let mut s = String::new();
        if size == 0 {
            return s;
        };
        for _ in 0..size {
            s.push(self.read_u8() as char);
        }
        return s;
    }
    pub fn read_utf_string(&mut self) -> String {
        let size = self.read_u32();
        let mut s = String::new();
        if size == 0 {
            return s;
        };
        for _ in 0..size {
            s.push(self.read_u8() as char);
        }
        return s;
    }
    pub fn read_bool(&mut self) -> bool {
        let x = self.read_u8();
        if x == 0 {
            return false;
        } else {
            return true;
        }
    }

    // Write functions
    pub fn write_u64(&mut self, num: u64) {
        let mut x: [u8; SIZE_LONG] = [0; SIZE_LONG];
        x[0] = (num >> 56) as u8;
        x[1] = (num >> 48) as u8;
        x[2] = (num >> 40) as u8;
        x[3] = (num >> 32) as u8;
        x[4] = (num >> 24) as u8;
        x[5] = (num >> 16) as u8;
        x[6] = (num >> 8) as u8;
        x[7] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_LONG;
    }
    pub fn write_u32(&mut self, num: u32) {
        let mut x: [u8; SIZE_INT] = [0; SIZE_INT];
        x[0] = (num >> 24) as u8;
        x[1] = (num >> 16) as u8;
        x[2] = (num >> 8) as u8;
        x[3] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_INT;
    }
    pub fn write_u16(&mut self, num: u16) {
        let mut x: [u8; SIZE_SHORT] = [0; SIZE_SHORT];
        x[0] = (num >> 8) as u8;
        x[1] = (num) as u8;
        for item in x.iter() {
            self.data.push(*item);
        }
        self.index += SIZE_SHORT;
    }
    pub fn write_u8(&mut self, num: u8) {
        self.data.push(num);
        self.index += SIZE_BYTE;
    }
    pub fn write_i64(&mut self, num: i64) {
        self.write_u64(num as u64);
    }
    pub fn write_i32(&mut self, num: i32) {
        self.write_u32(num as u32);
    }
    pub fn write_i16(&mut self, num: i16) {
        self.write_u16(num as u16);
    }
    pub fn write_i8(&mut self, num: i8) {
        self.write_u8(num as u8);
    }
    pub fn write_f64(&mut self, num: f64) {
        self.write_u64(num.to_bits());
    }
    pub fn write_f32(&mut self, num: f32) {
        self.write_u32(num.to_bits());
    }
    pub fn write_string(&mut self, val: &String) {
        let x = val.clone();
        self.write_u16(x.len() as u16);
        if x.is_empty() == true {
            return;
        }
        for i in x.into_bytes().iter() {
            self.write_u8(*i);
        }
    }
    pub fn write_utf_string(&mut self, val: &String) {
        let x = val.clone();
        self.write_u32(x.len() as u32);
        if x.is_empty() == true {
            return;
        }
        for i in x.into_bytes().iter() {
            self.write_u8(*i);
        }
    }
    pub fn write_bool(&mut self, b: bool) {
        if b == false {
            self.write_u8(0);
        } else {
            self.write_u8(1);
        }
    }
}

//TODO: move these to types.rs (use a trait?)
impl Buffer {
    pub fn write_world_position(&mut self, loc: &types::WorldPosition) {
        self.write_f32(loc.x);
        self.write_f32(loc.y);
    }
    pub fn write_position_record(&mut self, record: &types::PositionRecords) {
        self.write_i32(record.time);
    }
    pub fn write_slot_object(&mut self, slot: &types::SlotObjectData) {
        self.write_i32(slot.object_id);
        self.write_u8(slot.slot_id);
        self.write_i32(slot.object_type);
    }
    pub fn read_world_position(&mut self) -> types::WorldPosition {
        types::WorldPosition {
            x: self.read_f32(),
            y: self.read_f32(),
        }
    }
    pub fn read_ground_tile(&mut self) -> types::GroundTile {
        types::GroundTile {
            x: self.read_i16(),
            y: self.read_i16(),
            tile_type: self.read_u16(),
        }
    }
    pub fn read_object_data(&mut self) -> types::ObjectData {
        types::ObjectData {
            object_type: self.read_u16(),
            status: self.read_object_status_data(),
        }
    }
    pub fn read_object_status_data(&mut self) -> types::ObjectStatusData {
        let obj_id = self.read_i32();
        let pos = self.read_world_position();
        let stat_size = self.read_u16();
        let mut stats: HashMap<u8, types::StatData> = HashMap::new();
        for _ in 0..stat_size {
            let stat = self.read_stat_data();
            stats.insert(stat.stat_type, stat);
        }
        types::ObjectStatusData {
            object_id: obj_id,
            position: pos,
            stats: stats,
        }
    }
    pub fn read_stat_data(&mut self) -> types::StatData {
        let mut stat = types::StatData::new();
        stat.stat_type = self.read_u8();
        if stat.is_string_stat() == true {
            stat.str_stat_value = self.read_string();
        } else {
            stat.stat_value = self.read_i32();
        }
        stat
    }
    pub fn read_trade_item(&mut self) -> types::TradeItem {
        types::TradeItem {
            item: self.read_i32(),
            slot_type: self.read_i32(),
            tradeable: self.read_bool(),
            included: self.read_bool(),
        }
    }
}
