extern crate futures;
extern crate hyper;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use futures::{Future, future};
use hyper::{
    service::service_fn, 
    Body, 
    Request,
    Method,
    Response, 
    Server,
    StatusCode,
    Error
};

const INDEX: &'static str = r#"
 <!doctype html>
 <html>
     <head>
         <title>Rust Microservice</title>
     </head>
     <body>
         <h3>Rust Microservice...in progress</h3>
     </body>
 </html>
 "#;



fn microservice_handler(req: Request<Body>)
    -> impl Future<Item=Response<Body>, Error=Error>
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        },
        _ => {
            let response = Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())
                            .unwrap();
            future::ok(response)
        }
    }
}
fn main() {
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    let builder = Server::bind(&addr);

    let server = builder.serve(|| service_fn(microservice_handler));

    let server = server.map_err(drop);

    hyper::rt::run(server);
}
