#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn should_get_notification_by_user_id() {
        use crate::rocket;
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api/v1/notification/user/XXX01")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "[]");
    }
}