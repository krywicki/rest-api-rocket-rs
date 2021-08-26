#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::serde::{Deserialize, json::Json};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize, Debug, Validate)]
#[serde(crate = "rocket::serde", rename_all="camelCase")]
struct Input {
    first_name: String,
    last_name: String,

    #[validate(range(min=1, max=10))]
    num: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket\n"
}

#[post("/user", data="<input>")]
fn create_user(input: Json<Input>) -> Result<(), ValidationErrors> {
    //println!("{:#?}", input);
    input.validate()?;
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(),rocket::Error> {
    rocket::build()
        .mount("/", routes![index, create_user])
        .launch()
        .await
}
