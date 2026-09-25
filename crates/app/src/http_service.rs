use crate::calculate::search;
use search_core::Model;
use std::{error::Error, fs::File, time::Instant};
use tiny_http::{Header, Method, Request, Response, StatusCode};

pub fn serve_request(model: &Model, mut req: Request) -> Result<(), Box<dyn Error>> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        req.method(),
        req.url()
    );
    let begin_time = Instant::now();
    match req.method() {
        Method::Get => match req.url() {
            "/" | "/index.html" => {
                serve_static_file(req, "crates/app/resources/index.html")
                    .expect("Unable to serve static files:{index.html}");
            }
            "/index.js" => {
                serve_static_file(req, "crates/app/resources/index.js")
                    .expect("Unable to serve static files:js");
            }
            _ => req
                .respond(Response::new_empty(StatusCode(404)))
                .unwrap_or_else(|err| {
                    eprintln!("ERROR: failed to respond: {}", err);
                }),
        },
        Method::Post => match req.url() {
            "/api/search" => {
                let mut buf = Vec::new();
                req.as_reader().read_to_end(&mut buf)?;
                let body = String::from_utf8(buf).map_err(|err| {
                    eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
                });
                let body = body.unwrap();
                // TODO empty String should be forbidden
                println!("Search: {body}");
                let paths = search(&body, model)
                    .into_iter()
                    .map(|t| t.0)
                    .collect::<Vec<_>>();
                let response_body = serde_json::to_string(&paths)?;
                let content_type =
                    Header::from_bytes("Content-Type", "application/json; charset=utf-8")
                        .expect("Content-Type header should be valid");
                req.respond(Response::from_string(response_body).with_header(content_type))
                    .unwrap_or_else(|err| {
                        eprintln!("ERROR: {err}");
                    });
            }
            _ => not_support_service(req)?,
        },
        _ => not_support_service(req)?,
    }
    //print the performance of the whole process
    println!(
        "search service cost time: {}ms",
        begin_time.elapsed().as_millis()
    );
    Ok(())
}
fn serve_static_file(req: Request, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_suffix = file_path
        .split_at(file_path.find('.').and_then(|i| Some(i + 1)).unwrap_or(0))
        .1;
    let resp = Response::from_file(File::open(file_path)?).with_header(
        Header::from_bytes(
            "Content-Type",
            format!("text/{}; charset=utf-8", file_suffix),
        )
        .expect("So we didn't put any garbage in the headers"),
    );
    req.respond(resp).unwrap_or_else(|err| {
        eprintln!("ERROR: failed to respond to response: {}", err);
    });
    Ok(())
}
fn not_support_service(req: Request) -> Result<(), Box<dyn Error>> {
    if let Ok(_) = req.respond(
        Response::from_string("ERROR: unsupported service").with_status_code(StatusCode::from(404)),
    ) {
        println!("INFO: unsupported service");
        Ok(())
    } else {
        eprintln!("ERROR: return status code:404");
        Ok(())
    }
}
pub fn print_usage_and_error(subcommand: &str) -> Box<dyn Error> {
    //rewrite by GPT-5.6 sol
    let program = env!("CARGO_PKG_NAME");

    if matches!(subcommand, "index" | "search" | "serve") {
        eprintln!("error: invalid invocation of command '{subcommand}'");
    } else {
        eprintln!("error: unknown command '{subcommand}'");
    }

    eprintln!(
        "\
Usage:
  {program} index <INPUT_DIR> [OUTPUT_FILE]
  {program} search <QUERY> <INDEX_FILE>
  {program} serve <INDEX_FILE> [ADDRESS]

Commands:
  index   Build an index from XML files in INPUT_DIR
  search  Search INDEX_FILE and print the top 10 matching paths and scores
  serve   Start the HTTP search service using INDEX_FILE

Defaults:
  OUTPUT_FILE  index.json
  ADDRESS      127.0.0.1:8080"
    );

    std::process::exit(2);
}
