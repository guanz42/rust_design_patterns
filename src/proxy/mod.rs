mod server;

#[cfg(test)]
mod tests {
    use super::server::{NginxServer, Server};

    #[test]
    fn it_works() {
        let app_status = &"/app/status".to_string();
        let create_user = &"/create/user".to_string();

        let mut nginx = NginxServer::new();

        let (code, body) = nginx.handle_request(app_status, "GET");
        println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

        let (code, body) = nginx.handle_request(app_status, "GET");
        println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

        let (code, body) = nginx.handle_request(app_status, "GET");
        println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

        let (code, body) = nginx.handle_request(create_user, "POST");
        println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

        let (code, body) = nginx.handle_request(create_user, "GET");
        println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
    }
}
