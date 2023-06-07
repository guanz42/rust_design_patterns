mod parking;

#[cfg(test)]
mod tests {
    use super::parking::car::car_type::Body::*;
    use super::parking::car::car_type::Colour::*;
    use super::parking::Parking;

    #[test]
    fn it_works() {
        let mut parking = Parking::new();
        parking.add_car("MW2017", 15, Truck, Yellow);
        parking.add_car("KING", 64, Coupe, Red);
        parking.add_car("DARK01", 24, Sedan, Black);

        parking.print();
    }
}
