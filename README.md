# dev-forwarding-server

[![Crates.io](https://img.shields.io/crates/v/dev-forwarding-server)](https://crates.io/crates/dev-forwarding-server)

Often when working on a web app I'll need to route `/api/*` requests to an API
server, and the rest to a web server. In production this is accomplished w/
kubernetes, cloudfront, etc, but in development I use this tool.
`dev-api-server` takes a port to run on, an API port, and a web port.

## Installation

`cargo install dev-forwarding-server`

This will install the `dev-forwarding-server` binary.

## Usage

```bash
dev-forwarding-server --port 4300 --web-port 3000 --api-port 8000
```

This will start up a server on port 4300, that will forward api requests to port
8000, and other requests to port 3000.

## Helptext

```
USAGE:
    dev-forwarding-server --api-port <api-port> --web-port <web-port> --port <port>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --api-port <api-port>
    -p, --port <port>
    -w, --web-port <web-port>
```
