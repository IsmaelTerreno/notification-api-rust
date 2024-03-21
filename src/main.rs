#[macro_use] extern crate rocket;


#[get("/api/v1/notification/<user_id>")]
fn getNotificationsByUserId(user_id: &str) -> String {
    format!("Hello, {}!", user_id)
}

#[get("/api/v1/notification")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // Configures and launches the Rocket web server.
    rocket::build().mount("/", routes![index, getNotificationsByUserId])
}