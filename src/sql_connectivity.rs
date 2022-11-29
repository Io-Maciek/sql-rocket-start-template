use sqlx::pool::PoolConnection;
use rocket::serde::Serialize;
use rocket_db_pools::{Database};

use rocket_db_pools::sqlx::Sqlite;
//use sqlx::Mssql;


#[derive(Database)]
#[database("Database")]
pub struct SQL(sqlx::SqlitePool);		//SQLITE
//pub struct SQL(sqlx::MssqlPool);		//MSSQL

//CREATE TABLE Users(ID INTEGER PRIMARY KEY AUTOINCREMENT, 	username TEXT NOT NULL UNIQUE, 			password TEXT NOT NULL); 			SQLITE
//CREATE TABLE Users(ID INT PRIMARY KEY IDENTITY(0,1), 		username NVARCHAR(25) NOT NULL UNIQUE, 	password NVARCHAR(25) NOT NULL); 	MSSQL
#[derive(Debug)]
#[derive(Serialize)]
#[derive(sqlx::FromRow)]
pub struct User{
	#[sqlx(rename  = "ID")]
	id: i32,
	username: String,
	password: String,
}

impl User{
	pub fn new(username: String, password: String) -> User {
		User{
			id: -1,
			username,
			password
		}
	}

	pub async fn get_all(db: &mut PoolConnection<Sqlite>) -> Vec<User> {
		sqlx::query_as::<_, User>("SELECT * FROM Users")
			.fetch_all(db).await.ok().unwrap()
	}

	pub async fn insert(self, db: &mut PoolConnection<Sqlite>) -> User {
		let query = &format!("INSERT INTO Users(username, password) values('{}', '{}') RETURNING *", &self.username, &self.password);	//SQLITE
		//let query = format!("INSERT INTO Users OUTPUT inserted.* VALUES('{}', '{}')", self.username, self.password);							//MSSQL

		sqlx::query_as::<_, User>(&query)
			.fetch_one(db).await.ok().unwrap()
	}
}
