#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<span style=\"color: red;\">Hello, world!</span>"
}

#[get("/page")]
fn page() -> &'static str {
    "<span style=\"color: red;\">Hello, Sean!</span>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}