use crate::database::establish_connection;
use crate::models::Notification;

pub fn find_by_user_id(user_id: &str) -> String {
    use crate::schema::notification::dsl::*;
    use diesel::prelude::*;
    let connection = &mut establish_connection();
    let results = notification
        .limit(5)
        .select(Notification::as_select())
        .load(connection)
        .expect("Error loading notifications by user id.");
    println!("User id: {:?}", results);
    user_id.to_owned()
}

pub fn update() {
    println!("Hi now.");
}

pub fn create() {
    println!("Hi later.")
}
