mod freight_train;
mod passenger_train;

use super::train_station::Mediator;

pub use freight_train::FreightTrain;
pub use passenger_train::PassengerTrain;

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}
