#[macro_use] extern crate rocket;

use rocket::form::Form;

#[derive(FromForm)]
struct Wizard {
    name: String,
    level: i32,
}

#[post("/add_wizard", data = "<wizard>")]
fn add_wizard(wizard: Form<Wizard>) -> String {
    let name = &wizard.name;
    let level = wizard.level;

    // Logic to add the wizard to your system/database goes here

    format!("Added Wizard - Name: {}, Level: {}", name, level)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![add_wizard])
}
