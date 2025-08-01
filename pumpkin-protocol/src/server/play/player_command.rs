use std::io::Read;

use pumpkin_data::packet::serverbound::PLAY_PLAYER_COMMAND;
use pumpkin_macros::packet;

use crate::{
    ServerPacket, VarInt,
    ser::{NetworkReadExt, ReadingError},
};

#[packet(PLAY_PLAYER_COMMAND)]
pub struct SPlayerCommand {
    pub entity_id: VarInt,
    pub action: VarInt,
    pub jump_boost: VarInt,
}

pub enum Action {
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartHorseJump,
    StopHorseJump,
    OpenVehicleInventory,
    StartFlyingElytra,
}

pub struct InvalidAction;

impl TryFrom<i32> for Action {
    type Error = InvalidAction;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LeaveBed),
            1 => Ok(Self::StartSprinting),
            2 => Ok(Self::StopSprinting),
            3 => Ok(Self::StartHorseJump),
            4 => Ok(Self::StopHorseJump),
            5 => Ok(Self::OpenVehicleInventory),
            6 => Ok(Self::StartFlyingElytra),
            _ => Err(InvalidAction),
        }
    }
}

impl ServerPacket for SPlayerCommand {
    fn read(read: impl Read) -> Result<Self, ReadingError> {
        let mut read = read;

        Ok(Self {
            entity_id: read.get_var_int()?,
            action: read.get_var_int()?,
            jump_boost: read.get_var_int()?,
        })
    }
}
