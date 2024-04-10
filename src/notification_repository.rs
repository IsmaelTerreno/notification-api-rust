use crate::database::establish_connection;
use crate::models::Notification;

/// Find all the notifications by user id in the database.
pub fn find_by_user_id(id_search: &str) -> Vec<Notification> {
    use crate::schema::notification::dsl::*;
    use diesel::prelude::*;
    let connection = &mut establish_connection();
    println!("Finding notifications by user id: {}", id_search);
    notification
        .limit(5)
        .select(Notification::as_select())
        .load(connection)
        .expect("Error loading notifications by user id.")
}

/// Creates a new notification for a user id.
pub fn create() {}

pub fn update() {}

