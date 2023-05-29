#![allow(dead_code)]

pub mod apaptee;
pub mod target;

use self::{apaptee::SpecificTarget, target::Target};

pub struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

fn call(target: impl Target) {
    println!("'{}'", target.request());
}

#[cfg(test)]
mod tests {
    use crate::adapter::{apaptee::SpecificTarget, call, target::OrdinaryTarget, TargetAdapter};

    #[test]
    fn it_works() {
        let target = OrdinaryTarget;
        print!("A compatible target can be directly called: ");
        call(target);

        let adaptee = SpecificTarget;
        println!(
            "Adaptee is incompatible with client: '{}'",
            adaptee.specific_request()
        );

        let adapter = TargetAdapter::new(adaptee);

        print!("But with adapter client can call its method: ");
        call(adapter);
    }
}
