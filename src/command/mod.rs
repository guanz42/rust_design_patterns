#![allow(dead_code)]

mod commands;

use commands::Command;
use cursive::Cursive;

#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>,
}

fn execute(app: &mut Cursive, mut command: impl Command + 'static) {
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command));
        });
    }
}

fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app);
    }
    app.set_user_data(context);
}

#[cfg(test)]
mod tests {
    use cursive::{
        view::Nameable,
        views::{Dialog, EditView},
    };

    use super::{
        commands::{CopyCommand, CutCommand, PasteCommand},
        execute, undo, AppContext,
    };

    #[test]
    #[ignore]
    fn it_works() {
        let mut app = cursive::default();

        app.set_user_data(AppContext::default());
        app.add_layer(
            Dialog::around(EditView::default().with_name("Editor"))
                .title("Type and use bottons")
                .button("Copy", |s| execute(s, CopyCommand::default()))
                .button("Cut", |s| execute(s, CutCommand::default()))
                .button("Paste", |s| execute(s, PasteCommand::default()))
                .button("Undo", undo)
                .button("Quit", |s| s.quit()),
        );

        app.run();
    }
}
