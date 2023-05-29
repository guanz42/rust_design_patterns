#![allow(dead_code)]

use super::{HasMutableDevice, Remote};
use crate::bridge::device::Device;

pub struct AdvacedRemote<D: Device> {
    device: D,
}

impl<D: Device> AdvacedRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }

    pub fn mute(&mut self) {
        println!("Remote: mute");
        self.device.set_volume(0);
    }
}

impl<D: Device> HasMutableDevice<D> for AdvacedRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for AdvacedRemote<D> {}
