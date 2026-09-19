use crate::calculate::search;
use search_core::TFIndex;
use std::{error::Error, fs::File, time::Instant};
use tiny_http::{Header, Method, Request, Response, StatusCode};

pub fn serve_request(tf_index: &TFIndex, mut req: Request) -> Result<(), Box<dyn Error>> {
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
                let body = String::from_utf8(buf).unwrap_or_else(|err| {
                    eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
                    String::from("CONVERT ERROR")
                });
                println!("Search: {body}");
                let paths = search(&body, tf_index);
                let response_body = serde_json::to_string(&paths)?;
                let content_type =
                    Header::from_bytes("Content-Type", "application/json; charset=utf-8")
                        .expect("Content-Type header should be valid");
                req.respond(Response::from_string(response_body).with_header(content_type))
                    .unwrap_or_else(|err| {
                        eprintln!("ERROR: {err}");
                    });
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
    //print the performance of the whole process
    println!("cost time: {}ms", begin_time.elapsed().as_millis());
    Ok(())
}
pub fn serve_static_file(req: Request, file_path: &str) -> Result<(), Box<dyn Error>> {
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
pub fn print_usage_and_error(subcommand: &str) -> Box<dyn Error> {
    println!("Usage: [subcommand] [arg] [options]");
    println!(
        "subcommand `index`: index for the next arg as the input file and the second arg for output file if there is.Default it will output to 'index.json'"
    );
    println!("subcommand `search`: search the current directory if it does not exist");
    println!(
        "subcommand `serve <index-file> <address>`: init the http server for net,you can add the index file and add a optional[arg] for the address"
    );
    println!();
    eprintln!("ERROR: unknown subcommand: {subcommand}");
    std::process::exit(1);
}
