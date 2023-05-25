use super::gui::{Button, CheckBox, GuiFactory, GuiFactoryDynamic};

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("Button pressed in Mac");
    }
}

pub struct MacCheckBox;

impl CheckBox for MacCheckBox {
    fn switch(&self) {
        println!("CheckBox switch in Mac");
    }
}

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;

    type C = MacCheckBox;

    fn create_button(&self) -> Self::B {
        MacButton {}
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckBox {}
    }
}

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacCheckBox)
    }
}
