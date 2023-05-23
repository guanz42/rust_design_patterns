pub mod gui;
pub mod html_gui;
pub mod windows_gui;

use crate::gui::Dialog;
use crate::html_gui::HtmlDialog;
use crate::windows_gui::WindowsDialog;

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating HTML GUI --");
        &HtmlDialog
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_method() {
        let dialog = initialize();
        dialog.render();
        dialog.refresh();
    }
}
