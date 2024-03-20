#[macro_use] extern crate rocket;



#[get("/api/v1/notification/<user_id>")]
fn getNotificationsByUserId(user_id: &str) -> String {
    format!("Hello, {}!", user_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}