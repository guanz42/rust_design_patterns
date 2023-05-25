pub mod builders;
pub mod cars;
pub mod components;
pub mod director;

#[cfg(test)]
mod tests {
    use crate::builder::{
        builders::{car::CarBuilder, car_manual::CarManualBuilder, Builder},
        cars::{Car, Manual},
        director::Director,
    };

    #[test]
    fn it_works() {
        let mut car_builder = CarBuilder::default();

        Director::construct_sports_car(&mut car_builder);

        let car: Car = car_builder.build();
        println!("Car built: {:?}\n", car.car_type());

        let mut manual_builder = CarManualBuilder::default();

        Director::construct_city_car(&mut manual_builder);

        let manual: Manual = manual_builder.build();
        println!("Car manual built:\n{}", manual);
    }
}
