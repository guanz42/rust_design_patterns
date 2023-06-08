#![allow(dead_code)]

mod cohort;

#[cfg(test)]
mod tests {
    use super::cohort::{AttackCommand, Cohort, CohortCommander, RetreatCommand};

    #[test]
    fn it_works() {
        let mut commander = CohortCommander::new(Cohort {}, AttackCommand {}, RetreatCommand {});
        commander.attack();
        commander.stop();
        commander.retreat();
        commander.stop();
    }
}
