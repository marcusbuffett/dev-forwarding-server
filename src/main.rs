use clap::Clap;
use futures::future::{self, Future};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn debug_request(req: Request<Body>) -> BoxFut {
    let body_str = format!("{:?}", req);
    let response = Response::new(Body::from(body_str));
    Box::new(future::ok(response))
}

#[derive(Clap, Clone)]
#[clap(version = "1.0", author = "Marcus Buffett")]
struct Opts {
    #[clap(short, long)]
    api_port: String,
    #[clap(short, long)]
    web_port: String,
    #[clap(short, long)]
    port: u16,
}

fn main() {
    let opts: Opts = Opts::parse();
    // This is our socket address...
    let addr = ([127, 0, 0, 1], opts.port).into();

    // A `Service` is needed for every connection.
    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| {
            let opts: Opts = Opts::parse();
            // returns BoxFut

            if req.uri().path().starts_with("/api") {
                // will forward requests to port 13901
                return hyper_reverse_proxy::call(
                    remote_addr.ip(),
                    &format!("http://127.0.0.1:{}", opts.api_port),
                    req,
                );
            } else {
                // will forward requests to port 13902
                return hyper_reverse_proxy::call(
                    remote_addr.ip(),
                    &format!("http://127.0.0.1:{}", opts.web_port),
                    req,
                );
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server on http://{:?}", addr);

    // Run this server for... forever!
    hyper::rt::run(server);
}
