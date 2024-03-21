#[macro_use] extern crate rocket;
enum ApiUrl {
    GetNotificationsByUserId(String),
    DeleteNotificationById(String)
}

fn get_api_url(get_url: ApiUrl) -> String {
    match get_url {
        ApiUrl::GetNotificationsByUserId(_) => "/api/v1/notification/<user_id>".to_owned(),
        ApiUrl::DeleteNotificationById(_) => "/api/v1/notification/delete/<user_id>".to_owned()
    }
}


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