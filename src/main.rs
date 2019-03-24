
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod dal;

use serde_json;
use serde::{Serialize, Deserialize};

use rocksdb::{DB, Options};
use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error, Json
};
use listenfd::ListenFd;

use futures::{future, Future, Stream};
use actix_web::error::ParseError::Header;
use actix_web::actix::SyncArbiter;
use actix_web::actix::Addr;

use dal::repository;
use dal::repository::{DBExecutor, RepositoryContainer};
use dal::user::{User, CreateUser};
struct AppState {
    db: Addr<DBExecutor>,
}

fn index((name, state): (actix_web::Path<String>, State<AppState>) ) -> FutureResponse<HttpResponse> {
    return state.db
        .send( CreateUser{login_name: name.into_inner(), } )
        .from_err()

        .and_then(|res| match res {
                Ok(user) => {
                    //mail::send_mail("Wang <thegenius@vip.qq.com>", "test");
                    Ok(HttpResponse::Ok().json(user))
                },
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
        .responder();
}
fn static_index(req: &HttpRequest ) -> &'static str {
    return "hello";
}


fn main() {




    let sys = actix::System::new("diesel-example");
    let repo_container = RepositoryContainer::new(
        repository::RocksRepository::new(&std::path::Path::new("test.db"))
    );
    let addr = SyncArbiter::start(1, move || DBExecutor(repo_container.get_repository()));


    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(move ||
        App::with_state(AppState{db: addr.clone()})
        .resource("/user/{name}", |r| r.method(http::Method::GET).with_async(index)));
        //App::new()
        //    .resource("/hello", |r| r.f(static_index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("0.0.0.0:3000").unwrap()
    };
    println!("listen on {}", 3000);
    server.start();
    let _ = sys.run();






// NB: db is automatically closed at end of lifetime
  //  let path = "rocks_data";
  //  {
  //      let db = DB::open_default(path).unwrap();
  //      db.put(b"my key", b"my value").unwrap();
  //      match db.get(b"my key") {
  //          Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
  //          Ok(None) => println!("value not found"),
  //          Err(e) => println!("operational problem encountered: {}", e),
  //      }
  //      db.delete(b"my key").unwrap();
  //  }
  //  let _ = DB::destroy(&Options::default(), path);
  //  println!("Hello, world!");
}
