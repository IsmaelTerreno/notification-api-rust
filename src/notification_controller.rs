use rocket::get;

/// Get user notifications by user id
#[get("/api/v1/notification/<user_id>")]
pub fn get_notifications_by_user_id(user_id: &str) -> String {
    format!("Hello, {}!", user_id)
}

#[get("/api/v1/notification")]
pub fn index() -> &'static str {
    "Hello, world!"
}
