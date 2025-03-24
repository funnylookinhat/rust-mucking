use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

type Counter = Arc<Mutex<u32>>;

async fn handle_request(
    req: Request<Body>,
    counter: Counter,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/incr") => {
            let mut count = counter.lock().unwrap();
            *count += 1;
            Ok(Response::new(Body::from("Incremented")))
        }
        (&Method::GET, "/count") => {
            let count = counter.lock().unwrap();
            Ok(Response::new(Body::from(format!(
                "Current count: {}",
                *count
            ))))
        }
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let counter: Counter = Arc::new(Mutex::new(0));
    // Create a service that properly captures the counter
    
    let make_svc: = make_service_fn(move |_conn| {
        let counter = counter.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let counter = counter.clone();
                handle_request(req, counter)
            }))
        }
    });

    // Create the server and run it
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
