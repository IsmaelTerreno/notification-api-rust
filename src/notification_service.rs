use crate::notification_repository::find_by_user_id;

pub fn find_by_user_id_notifications(user_id: &str) -> String {
    let result = find_by_user_id(user_id);
    format!("{:?}", result)
}