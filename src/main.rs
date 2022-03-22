#[macro_use]
extern crate serde_derive;

use std::env;

use crate::event::push::{process_gitlab_push_event, process_push_event};
use crate::event::release::process_release_event;
use crate::telegram::send_message_to_telegram;
use std::convert::Infallible;
use std::net::SocketAddr;
use futures::{future, TryStreamExt};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use tokio::stream::StreamExt;
use hyper::body::HttpBody;
use hyper::body::to_bytes;

pub mod event;
pub mod markdown;
pub mod telegram;

#[cfg(test)]
pub mod test;

const TOKEN: &str = "2127284370:AAHO9SCKCOsWyE7A7gOxnATk7enA8zB6-8s";
const CHAT_ID: &str = "588813729";

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            println!("hello");

            for h in _req.headers() {
                println!("{} = {}", h.0, h.1.to_str().unwrap());
            }

            let vec = _req.into_body().data().await.unwrap().unwrap().to_vec();
            let strBody = std::str::from_utf8(&vec).unwrap();

            let message = process_gitlab_push_event(strBody.to_string());
            send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), message).await;


            // *response.body_mut() = _req.into_body();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}
