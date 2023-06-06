pub struct Notification;

impl Notification {
    pub fn send_wallet_credit_notification(&self) {
        println!("Send wallet credit notification");
    }

    pub fn send_wallet_debit_notification(&self) {
        println!("Send debit credit notification");
    }
}
