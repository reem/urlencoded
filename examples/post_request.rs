// This example is almost the same as get_request.rs, but uses UrlEncodedBody
extern crate iron;
extern crate urlencoded;

use iron::{Iron, Request, Response, IronResult, Plugin, Set, status};
use iron::response::modifiers::{Status, Body};
use urlencoded::UrlEncodedBody;

fn log_post_data(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedBody>() {
        Some(ref hashmap) => println!("Parsed POST request body:\n {}", hashmap),
        None => println!("Error, no body found.")
    };

    Ok(Response::new().set(Status(status::Ok)).set(Body("Hello!")))
}

// Test with `curl -i -X POST "http://localhost:3000/" --data "fruit=apple&name=iron&fruit=pear"`
fn main() {
    Iron::new(log_post_data).listen("localhost:3000").unwrap();
}
