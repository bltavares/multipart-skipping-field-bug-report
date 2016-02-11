extern crate hyper;
extern crate multipart;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;
use multipart::server::Multipart;

const INDEX: &'static str = include_str!("index.html");

fn index(req: Request, res: Response) {
    res.send(INDEX.as_bytes());
}

fn handle(req: Request, res: Response) {
    if let Ok(mut multipart_request) = Multipart::from_request(req) {
        multipart_request.foreach_entry(|x| {
            println!("Entry name: {:?}", x.name);
        }).expect("Could not read multipart entries");
    }
}

fn router(req: Request, res: Response) {
    if let RequestUri::AbsolutePath(request_path) = req.uri.clone() {
        if request_path == "/go" {
            handle(req, res);
            return;
        }

        index(req, res);
    } else {
        index(req, res);
    }
}

fn main() {
    println!("Running on 0.0.0.0:7000");

    let port = 7000;
    Server::http(("0.0.0.0", port)).unwrap().handle(router).unwrap();
}
