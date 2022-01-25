//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("static");

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    let req_path = req.get_path();
    let file_path = &req_path[1..];

    let index_path = if file_path == "" {
        String::from("index.html")
    } else if file_path.chars().last() == Some('/') {
        format!("{file_path}index.html")
    } else {
        format!("{file_path}/index.html")
    };

    if let Some(file) = STATIC_DIR.get_file(file_path) {
        let body = file.contents();
        let mut resp = Response::from_status(StatusCode::OK).with_body(body);

        let guess = mime_guess::from_path(file_path);
        if let Some(mime) = guess.first() {
            resp.set_content_type(mime);
        }

        Ok(resp)
    } else if let Some(file) = STATIC_DIR.get_file(index_path) {
        Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML)
            .with_body(file.contents()))
    } else {
        Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n"))
    }
}
