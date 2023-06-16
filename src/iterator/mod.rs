mod users;

#[cfg(test)]
mod test {
    use super::users::UserCollection;

    #[test]
    fn it_works() {
        let array = &[1, 2, 3];
        let iterator = array.iter();

        iterator.for_each(|x| print!("{} ", x));

        println!("\n");

        let users = UserCollection::new();
        let mut iterator = users.iter();

        println!("1nd element: {:?}", iterator.next());
        println!("2nd element: {:?}", iterator.next());
        println!("3nd element: {:?}", iterator.next());
        println!("4nd element: {:?}", iterator.next());

        print!("\nAll elements in user collection: ");
        iterator.for_each(|e| print!("{} ", e));

        println!();
    }
}
