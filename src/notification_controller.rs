use rocket::get;

/// Get user notifications by user id
#[get("/api/v1/notification/user/<user_id>")]
pub fn get_notifications_by_user_id(user_id: &str) -> String {
    crate::notification_service::find_by_user_id_notifications(user_id)
}

#[get("/api/v1/notification")]
pub fn index() -> &'static str {
    "Hello, world!"
}
