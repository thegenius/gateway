use std::ffi::OsString;
use std::path::Path;
use std::result::Result;
use std::sync::Arc;
use serde_json;
use serde::{Serialize, Deserialize};
use actix_web::actix::{Actor, SyncContext, Message, Handler};
use rocksdb::{DB, Options};

pub struct DBExecutor(pub Arc<RocksRepository>);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}


pub struct RepositoryContainer {
    repo: Arc<RocksRepository>,
}

impl RepositoryContainer {
    pub fn new(repo: RocksRepository) -> RepositoryContainer {
        return RepositoryContainer { repo: Arc::new(repo)  };
    }
    pub fn get_repository(&self) -> Arc<RocksRepository> {
        return self.repo.clone();
    }
}


#[derive(Clone)]
pub struct RocksRepository {
    db: Arc<DB>,
}

impl RocksRepository {
    pub fn new(path: &AsRef<Path>) -> RocksRepository {
        let db = DB::open_default(path).unwrap();
        RocksRepository {
            db: Arc::new(db),
        }
    }


    pub fn save(&self, key: &String, val: &String) -> bool {
        self.db.put(key.as_bytes(), val.as_bytes().to_vec());
        return true;
    }

    pub fn fetch(&self, key: &String)-> String {
        return self.db.get(key).unwrap().unwrap().to_utf8().unwrap().to_string();
//        let content = String::from_utf8(val_vec).unwrap();
//        return content;
    }

}

