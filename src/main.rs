use clap::Clap;
use futures::future::Future;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};

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
    let addr = ([127, 0, 0, 1], opts.port).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| {
            let opts: Opts = Opts::parse();

            if req.uri().path().starts_with("/api") {
                return hyper_reverse_proxy::call(
                    remote_addr.ip(),
                    &format!("http://127.0.0.1:{}", opts.api_port),
                    req,
                );
            } else {
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

    hyper::rt::run(server);
}
