mod calculate;
mod http_service;
use http_service::*;
use search_core::{TFIndex, read_xml_dir_and_write};
use std::error::Error;
use std::fs::File;
use tiny_http::Server;

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
