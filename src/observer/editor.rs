#![allow(dead_code)]

use super::publisher::{Event, Publisher};

#[derive(Default)]
pub struct Editor {
    publisher: Publisher,
    file_path: String,
}

impl Editor {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path);
    }

    pub fn save(&mut self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}
