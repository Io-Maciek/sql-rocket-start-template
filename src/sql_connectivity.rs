use sqlx::pool::PoolConnection;
use rocket::serde::Serialize;
use rocket_db_pools::{sqlx::Sqlite, Database};

#[derive(Database)]
#[database("Database")]
pub struct SQL(sqlx::SqlitePool);

#[derive(Debug)]
#[derive(Serialize)]
#[derive(sqlx::FromRow)]
pub struct User{	//CREATE TABLE Users(ID integer PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, password TEXT NOT NULL);
	ID: i32,
	username: String,
	password: String,
}

impl User{
	pub fn new(username: String, password: String) -> User {
		User{
			ID: -1,
			username,
			password
		}
	}

	pub async fn get_all(db: &mut PoolConnection<Sqlite>) -> Vec<User> {
		sqlx::query_as::<_, User>("SELECT * FROM Users")
			.fetch_all(db).await.ok().unwrap()
	}

	pub async fn insert(self, db: &mut PoolConnection<Sqlite>) -> User {
		sqlx::query_as::<_, User>(&format!("INSERT INTO Users(username, password) values('{}', '{}') RETURNING *", &self.username, &self.password))
			.fetch_one(db).await.ok().unwrap()
	}
}
