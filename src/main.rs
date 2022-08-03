#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;


#[get("/")]
fn index() -> &'static str {
    "Hello, worldy!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
