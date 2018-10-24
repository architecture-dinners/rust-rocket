#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::State;
use rocket::response::status;
use std::sync::atomic::{Ordering, AtomicUsize };

struct Counter {
    count: AtomicUsize
}


#[get("/hello")]
fn endpoint_hello() -> &'static str {
    "Hello, world!"
}


#[derive(FromForm)]
struct Echo {
    text: Option<String>
}

#[get("/echo?<echo>")]
fn endpoint_echo(echo: Echo) -> Result<String, status::BadRequest<String>> {
    if let Some(text) = echo.text {
        Ok(format!("{}\n", text))
    } else {
        Err(status::BadRequest(Some("Missing text param\n".to_string())))
    }
}

#[get("/boom")]
fn endpoint_boom() -> &'static str {
    panic!("Boom");
}

#[post("/incr")]
fn endpoint_incr(counter: State<Counter>) -> String {
    let prev = counter.count.load(Ordering::Relaxed);
    counter.count.store(prev + 1, Ordering::Relaxed);
    format!("{}\n", counter.count.load(Ordering::Relaxed))
}

fn main() {
    rocket::ignite()
        .manage(Counter { count: AtomicUsize::new(0) })
        .mount("/", routes![
            endpoint_hello,
            endpoint_echo,
            endpoint_boom,
            endpoint_incr,
        ])
        .launch();
}
