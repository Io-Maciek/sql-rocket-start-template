## Rust Rocket template with Handlebars templating and SQLite database connection

sqlite3 is required for this repository, but can be changed to any kind of [sql pool](https://api.rocket.rs/master/rocket_db_pools/).

Starting database included in this repository *Database.db* has on table created with command:
 > CREATE TABLE Users(ID integer PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, password TEXT NOT NULL);
