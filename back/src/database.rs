use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use postgres;
use postgres::params::{ConnectParams, Host};
use postgres::Connection;
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn connect() -> Pool {
  let url = format!(
    "postgres://{}:{}@{}/{}",
    env::var("POSTGRES_USER").unwrap(),
    env::var("POSTGRES_PASSWORD").unwrap(),
    env::var("POSTGRES_HOST").unwrap(),
    env::var("POSTGRES_DB").unwrap()
  );
  let manager = PostgresConnectionManager::new(url, TlsMode::None).unwrap();
  Pool::new(manager).expect("Failed to create pool")
}

pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
    let pool = request.guard::<State<Pool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for DbConn {
  // If using Sqlite
  // type Target = SqliteConnection;
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
