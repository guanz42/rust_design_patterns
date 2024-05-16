#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Read};

    #[test]
    fn it_works() {
        let mut buf = [0u8; 10];

        let mut input = BufReader::new(Cursor::new("Input Data"));

        input.read_exact(&mut buf).ok();

        print!("Read from a buffered reader: ");

        for byte in buf {
            print!("{}", char::from(byte));
        }

        println!();
    }
}
