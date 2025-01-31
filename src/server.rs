use tiny_http::{Server, Response, Header};
use std::fs;
use std::path::Path;

pub fn start_server(port: u16) -> std::io::Result<()> {
    let address = format!("0.0.0.0:{}", port);
    let server = Server::http(&address)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    println!("Serving at http://{}", address);

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        let path = if url.is_empty() {
            // If no file is specified, serve index.html
            "public/index.html".to_string()
        } else {
            // Serve requested file
            format!("public/{}", url)
        };

        let path_obj = Path::new(&path);
        if path_obj.is_file() {
            match fs::read(&path) {
                Ok(contents) => {
                    let mut response = Response::from_data(contents);

                    // Basic MIME type detection
                    if path.ends_with(".html") {
                        response = response.with_header(
                            "Content-Type: text/html"
                                .parse::<Header>()
                                .unwrap()
                        );
                    } else if path.ends_with(".css") {
                        response = response.with_header(
                            "Content-Type: text/css"
                                .parse::<Header>()
                                .unwrap()
                        );
                    } else if path.ends_with(".js") {
                        response = response.with_header(
                            "Content-Type: application/javascript"
                                .parse::<Header>()
                                .unwrap()
                        );
                    }

                    request.respond(response)?;
                }
                Err(_) => {
                    let not_found = Response::from_string("404 Not Found").with_status_code(404);
                    request.respond(not_found)?;
                }
            }
        } else {
            let not_found = Response::from_string("404 Not Found").with_status_code(404);
            request.respond(not_found)?;
        }
    }

    Ok(())
}
