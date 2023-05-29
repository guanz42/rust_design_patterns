mod device;
mod remotes;

#[cfg(test)]
mod tests {
    use crate::bridge::remotes::{
        advanced::AdvacedRemote, basic::BasicRemote, HasMutableDevice, Remote,
    };

    use super::device::{Device, Radio, TV};

    fn test_device<T>(device: T)
    where
        T: Device + Clone,
    {
        println!("Tests with basic remote.");
        let mut basci_remote = BasicRemote::new(device.clone());
        basci_remote.power();
        basci_remote.device().print_status();

        println!("Tests with advanced remote.");
        let mut advanced_remote = AdvacedRemote::new(device);
        advanced_remote.power();
        advanced_remote.mute();
        advanced_remote.device().print_status();
    }

    #[test]
    fn it_works() {
        test_device(TV::default());
        test_device(Radio::default());
    }
}
