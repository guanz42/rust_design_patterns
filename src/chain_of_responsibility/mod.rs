mod department;
mod patient;

#[cfg(test)]
mod tests {
    use super::{department::*, patient::Patient};

    #[test]
    fn it_works() {
        let cashier = Cashier::default();
        let medical = Medical::new(cashier);
        let doctor = Doctor::new(medical);
        let mut reception = Reception::new(doctor);

        let mut patient = Patient {
            name: "Jack".into(),
            ..Patient::default()
        };

        reception.execute(&mut patient);

        println!("\nThe patient has been already handled:\n");

        reception.execute(&mut patient);
    }
}
