use serde::{Serialize, Deserialize};
use actix_web::actix::{Actor, SyncContext, Message, Handler};
use super::repository::{DBExecutor, RocksRepository};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub login_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub login_name: String,
}

impl Message for CreateUser {
    type Result = Result<User, actix_web::Error>;
}

impl Handler<CreateUser> for DBExecutor {
    type Result = Result<User, actix_web::Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context)-> Self::Result {
        let sled_repo: &RocksRepository = &self.0;
	let id = 23;
        let user = User {
            id: 23,
            login_name: msg.login_name
        };

        let json_content = serde_json::to_string(&user).unwrap();
        let id_str: String = id.to_string();
        sled_repo.save(&id_str , &json_content);

        let saved_content = sled_repo.fetch(&id_str);
        let saved_user: User = serde_json::from_str(saved_content.as_str()).unwrap();
        let saved_user_content: String = serde_json::to_string(&saved_user).unwrap();
        return Ok(user);
    }
}

