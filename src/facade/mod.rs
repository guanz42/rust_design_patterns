mod account;
mod ledger;
mod notification;
mod security_code;
mod wallet;
mod wallet_facade;

#[cfg(test)]
mod tests {
    use super::wallet_facade::WalletFacade;

    #[test]
    fn it_works() {
        let mut wallet = WalletFacade::new("abc".into(), 1234);
        println!();

        let r1 = wallet.add_money_to_wallet(&"abc".into(), 1234, 10);
        println!("{:?}", r1);

        let r2 = wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5);
        println!("{:?}", r2);
    }
}
