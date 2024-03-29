use crate::database::establish_connection;
use crate::models::Notification;

/// Find all the notifications by user id in the database.
pub fn find_by_user_id(id_search: &str) -> String {
    use crate::schema::notification::dsl::*;
    use diesel::prelude::*;
    let connection = &mut establish_connection();
    let results = notification
        .limit(5)
        .select(Notification::as_select())
        .load(connection)
        .expect("Error loading notifications by user id.");
    println!("User id: {:?}", results);
    id_search.to_string()
}

/// Creates a new notification for a user id.
pub fn create() {}

pub fn update() {
    println!("Hi now.");
}

