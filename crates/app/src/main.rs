mod calculate;
mod http_service;
use http_service::*;
use search_core::{Model, re_index, read_xml_dir_and_write, write_to_disk};
use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use std::time::Instant;
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
            let content = args
                .next()
                .ok_or_else(|| print_usage_and_error(subcommand.as_str()))?;
            let (model, _) = load_index(&mut args, subcommand)?;
            let mut cnt = 1u32;
            println!("Here is the top 10 relativest files:");
            for ret_tuple in calculate::search(&content, &model) {
                println!(
                    "TOP {:02} => {:<50} with scores:{:>.5}",
                    cnt, ret_tuple.0, ret_tuple.1
                );
                cnt += 1;
            }
        }
        "serve" => {
            let begin_time = Instant::now();
            let (model, path) = load_index(&mut args, subcommand)?;
            let model = Arc::new(RwLock::new(model));
            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            let server = Server::http(&address).map_err(|_| {
                eprintln!("ERROR: unable to start server: {}", address);
                std::process::exit(1);
            })?;
            println!(
                "Listening on http://{address},cost {:.2} ms",
                begin_time.elapsed().as_millis()
            );
            // spawn a sync thread
            let clone = Arc::clone(&model);
            std::thread::spawn(move || {
                const THREAD_NAME: &str = "[SYNC THREAD]";
                println!("DEBUG: {THREAD_NAME} acquire the read lock");
                let model = clone.read().unwrap();
                let bulk = re_index(&model);
                // 显式释放读锁，防止变量遮蔽产生死锁
                drop(model);
                if bulk.is_err() {
                    eprintln!("ERROR: re_index failed,err={:?}", bulk.err());
                    return;
                }
                let bulk = bulk.unwrap();
                if bulk.is_none() {
                    println!("DEBUG: {THREAD_NAME} no need to apply bulk");
                    return;
                }
                println!("DEBUG: {THREAD_NAME} acquire the write lock");
                let mut model = clone.write().unwrap();
                let begin_time = Instant::now();
                model.apply(bulk.unwrap());
                //释放写锁
                drop(model);
                println!(
                    "DEBUG: apply bulk cost:[{}] ms",
                    begin_time.elapsed().as_millis()
                );
                let model = clone.read().unwrap();
                println!("DEBUG: {THREAD_NAME} sync index completed,ready to write to the disk");
                let _ = write_to_disk(&model, path)
                    .map_err(|err| eprintln!("ERROR: write to disk during the sync,err={err}"));
            });
            // main thread handle the requests
            for req in server.incoming_requests() {
                serve_request(Arc::clone(&model), req)?;
            }
        }
        _ => {
            print_usage_and_error(subcommand.as_str());
        }
    }
    Ok(())
}
/// 根据arg加载索引
fn load_index(args: &mut Args, subcommand: String) -> Result<(Model, String), Box<dyn Error>> {
    let index_path = args.next().ok_or_else(|| {
        eprintln!("ERROR: no path to index is provide for {subcommand} subcommand");
        print_usage_and_error(subcommand.as_str())
    })?;
    let index_file = File::open(&index_path).map_err(|err| {
        eprintln!("ERROR: could not open index file {index_path}:{err}");
        print_usage_and_error(subcommand.as_str())
    })?;
    let model: Model = serde_json::from_reader(BufReader::new(&index_file)).map_err(|_| {
        eprintln!("ERROR: could not parse the index_file:{index_file:?}");
        print_usage_and_error(subcommand.as_str())
    })?;
    Ok((model, index_path))
}
