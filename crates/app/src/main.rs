mod calculate;
use search_core::{TFIndex, read_xml_dir_and_write};
use std::{error::Error, path::PathBuf};
use std::fs::File;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

use crate::calculate::{idf, tf};
type Lexer<'a> = search_core::lexer::Lexer<'a>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let subcommand = args.next().unwrap_or_else(|| "index".to_string());

    match subcommand.as_str() {
        "index" => {
            let index_path = args
                .next()
                .ok_or_else(|| print_usage_and_error(subcommand.as_str()))?;
            let target_path = args.next().unwrap_or_else(|| String::from("index.json"));
            read_xml_dir_and_write(index_path, target_path)?;
        }
        "search" => {
            todo!("search")
        }
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                eprintln!("ERROR: no path to index is provide for {subcommand} subcommand");
                print_usage_and_error(subcommand.as_str())
            })?;
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}:{err}");
                print_usage_and_error(subcommand.as_str())
            })?;
            let tf_index: TFIndex = serde_json::from_reader(&index_file).map_err(|_| {
                eprintln!("ERROR: could not parse the index_file:{index_file:?}");
                print_usage_and_error(subcommand.as_str())
            })?;

            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            let server = Server::http(&address).map_err(|_| {
                eprintln!("ERROR: unable to start server: {}", address);
                std::process::exit(1);
            })?;
            println!("Listening on http://{address}");
            for req in server.incoming_requests() {
                serve_request(&tf_index, req)?;
            }
        }
        _ => {
            print_usage_and_error(subcommand.as_str());
        }
    }
    Ok(())
}

fn serve_request(tf_index: &TFIndex, mut req: Request) -> Result<(), Box<dyn Error>> {
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
        Method::Post => match req.url() {
            "/api/search" => {
                let mut buf = Vec::new();
                req.as_reader().read_to_end(&mut buf)?;
                let body = String::from_utf8(buf).unwrap_or_else(|err| {
                    eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
                    String::from("CONVERT ERROR")
                });
                println!("Search: {body}");
                let body = body.chars().collect::<Vec<_>>();
                let tokens:Vec<String>=Lexer::new(&body).collect();
                // record the rank of each doc's tf-idf score
                let mut res:Vec<(&PathBuf,f32)>=Vec::with_capacity(tf_index.len());
                for (path, tf_table) in tf_index {
                    let mut rank=0f32;
                    for token in &tokens{
                        rank += tf(token, tf_table) * idf(token, tf_index);
                    }
                    res.push((path,rank));
                }
                res.sort_by(|(_,r1),(_,r2)|r2.partial_cmp(r1).unwrap());
                //show the top 10
                for (path,rank) in res.iter().take(10){
                    println!("{path} => {rank}", path = path.display());
                }
                let _ = req
                    .respond(Response::from_string("ok"))
                    .unwrap_or_else(|err| {
                        eprintln!("ERROR: {err}");
                    });
            }
            _ => todo!(),
        },
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
fn print_usage_and_error(subcommand: &str) -> Box<dyn Error> {
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
