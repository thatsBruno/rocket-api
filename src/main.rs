#[macro_use] extern crate rocket;

struct Wizard {
    name: String,
    level: u8,
}

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}