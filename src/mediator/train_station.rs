pub trait Mediator {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool;
    fn notify_about_departure(&mut self, train_name: &str);
}
