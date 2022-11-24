mod sql_connectivity;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket::{Build, Rocket};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use crate::sql_connectivity::{SQL, User};
use rocket_db_pools::{Database, Connection};

#[get("/")]
async fn index(mut db: Connection<SQL>) -> Template {
	let users_vec = User::get_all(&mut *db).await;
	Template::render("index", context! {
		title: "Title",
		users: users_vec
	})
}

#[get("/add/<username>/<password>")]
async fn add_new(mut db: Connection<SQL>, username: String, password: String) -> Redirect {
	User::new(username, password).insert(&mut *db).await;
	Redirect::to(uri!(index))
}

#[launch]
fn rocket() -> Rocket<Build> {
	let figment = rocket::Config::figment()
		.merge(("address", "0.0.0.0"))
		.merge(("databases.Database", rocket_db_pools::Config{
			url: "Database.db".to_string(),
			min_connections: None,
			max_connections: 1024,
			connect_timeout: 5,
			idle_timeout: None
		}));

	rocket::custom(figment)
		.attach(static_resources_initializer!(
			"favicon" => "img/favicon.png",
		))
		.mount("/", routes![favicon, index, add_new])
		.attach(Template::fairing())
		.attach(SQL::init())
}

static_response_handler! {
    "/favicon.png" => favicon => "favicon",
}
