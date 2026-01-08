use crate::domain::users::User;

pub async fn repo_get_user_by_id(id: i32) -> Result<User, String> {
    if id > 0 && id < 100 {
        Ok(User {
            id,
            name: "Sud Lore".to_string(),
        })
    } else {
        Err(format!("User with id {} not found", id))
    }
}

pub async fn repo_get_all_users(
    is_exit: bool,
) -> Result<Option<Vec<User>>, Box<dyn std::error::Error>> {
    if is_exit {
        let users = vec![
            User {
                id: 1,
                name: "Sud Lore".to_string(),
            },
            User {
                id: 2,
                name: "Ada Lovelace".to_string(),
            },
        ];
        Ok(Some(users))
    } else {
        Ok(None)
    }
}
