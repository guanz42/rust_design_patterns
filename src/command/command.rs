use cursive::Cursive;

mod copy;
mod cut;
mod paste;

pub use copy::CopyCommand;
pub use cut::CutCommand;
pub use paste::PasteCommand;

pub trait Command {
    fn execute(&mut self, app: &mut Cursive) -> bool;
    fn undo(&mut self, app: &mut Cursive);
}
