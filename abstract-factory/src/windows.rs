use crate::gui::{Button, CheckBox, GuiFactory, GuiFactoryDynamic};

pub struct WinButton;

impl Button for WinButton {
    fn press(&self) {
        println!("Button pressed in Windows");
    }
}

pub struct WinCheckBox;

impl CheckBox for WinCheckBox {
    fn switch(&self) {
        println!("CheckBox switch in Windows");
    }
}

pub struct WinFactory;

impl GuiFactory for WinFactory {
    type B = WinButton;

    type C = WinCheckBox;

    fn create_button(&self) -> Self::B {
        WinButton {}
    }

    fn create_checkbox(&self) -> Self::C {
        WinCheckBox {}
    }
}

impl GuiFactoryDynamic for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WinCheckBox)
    }
}
