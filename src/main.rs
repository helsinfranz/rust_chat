use rocket::{get, launch, routes};
use rocket_ws::{Stream, WebSocket};

#[get("/echo")]
fn echo_stream(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, echo_stream])
}
