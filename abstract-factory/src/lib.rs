use gui::{GuiFactory, GuiFactoryDynamic};

pub mod gui;
pub mod macos;
pub mod windows;

pub fn render(factory: &impl GuiFactory) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    use gui::{Button, CheckBox};

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}

pub fn render_dyn(factory: &dyn GuiFactoryDynamic) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}

#[cfg(test)]
mod tests {
    use super::gui::GuiFactoryDynamic;
    use super::macos::MacFactory;
    use super::windows::WinFactory;
    use super::*;

    #[test]
    fn factory() {
        let windows = false;

        if windows {
            render(&WinFactory)
        } else {
            render(&MacFactory)
        }
    }

    #[test]
    fn factory_dyn() {
        let windows = false;

        // Allocate a factory object in runtime depending on unpredictable input.
        let factory: &dyn GuiFactoryDynamic = if windows { &WinFactory } else { &MacFactory };

        let button = factory.create_button();
        button.press();

        render_dyn(factory);
    }
}
