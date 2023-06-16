mod freight_train;

use super::train_station::Mediator;

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}
