extern crate notification;
#[macro_use]
extern crate rocket;

pub mod e2e;

#[launch]
pub fn rocket() -> _ {
    use notification::notification_controller::{get_notifications_by_user_id, index};
    // Configures and launches the Rocket web server.
    rocket::build().mount("/", routes![index, get_notifications_by_user_id])
}
