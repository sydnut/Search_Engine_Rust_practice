use core::read_xml_dir_and_write;
use std::error::Error;
use std::fs::File;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let subcommand = args.next().unwrap_or_else(|| "index".to_string());

    match subcommand.as_str() {
        "index" => {
            let index_path = args
                .next()
                .ok_or_else(|| usage_and_error(subcommand.as_str()))?;
            let target_path = args.next().unwrap_or_else(|| String::from("index.json"));
            read_xml_dir_and_write(index_path, target_path)?;
        }
        "search" => {
            todo!("search")
        }
        "serve" => {
            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            let server = Server::http(&address).map_err(|_| {
                eprintln!("ERROR: unable to start server: {}", address);
                std::process::exit(1);
            })?;
            println!("Listening on http://{address}");
            for req in server.incoming_requests() {
                serve_request(req)?;
            }
        }
        _ => {
            usage_and_error(subcommand.as_str());
        }
    }
    Ok(())
}

fn serve_request(req: Request) -> Result<(), Box<dyn Error>> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        req.method(),
        req.url()
    );
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
        Method::Post =>{
            match req.url() {
                "/api/search" =>{

                }
                _ => todo!()
            }
        }
        _ => todo!(),
    }
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
fn usage_and_error(subcommand: &str) -> Box<dyn Error> {
    println!("Usage: [subcommand] [arg] [options]");
    println!(
        "subcommand index: index for the next arg as the input file and the second arg for output file if there is.Default it will output to 'index.json'"
    );
    println!("subcommand search: search the current directory if it does not exist");
    println!(
        "subcommand serve: init the http server for net,you can add a optional[arg] for the address"
    );
    println!();
    eprintln!("ERROR: unknown subcommand: {subcommand}");
    std::process::exit(1);
}
