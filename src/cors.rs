use std::future::Future;
use std::pin::Pin;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Method, http::Status, Request, Response};

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    fn on_response<'r, 'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self, 
        request: &'r Request<'life1>, 
        response: &'life2 mut Response<'r>
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        Self: 'async_trait,
        'r: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
    {
        // Add CORS headers to allow all origins to all outgoing requests
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));

        // Respond to all `OPTIONS` requests with a `204` (no content) status
        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
        Box::pin(async {})
    }
}