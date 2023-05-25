#![allow(dead_code)]

#[derive(Clone)]
struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle1 = Circle {
            x: 10,
            y: 5,
            radius: 10,
        };

        let mut circle2 = circle1.clone();
        circle2.radius = 42;

        println!("Circle 1: {}, {}, {}", circle1.x, circle1.y, circle1.radius);
        println!("Circle 2: {}, {}, {}", circle2.x, circle2.y, circle2.radius);
    }
}