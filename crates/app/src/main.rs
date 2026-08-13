use tiny_http::{Response, Server};
use core::read_xml_dir_and_write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args=std::env::args();
    args.next();
    let subcommand=args.next().unwrap_or_else(|| "index".to_string());

    match subcommand.as_str() {
        "index" => {
            let index_path=args.next().ok_or_else(||usage_and_error(subcommand.as_str()))?;
            let target_path=args.next().unwrap_or_else(||String::from("index.json"));
            read_xml_dir_and_write(index_path, target_path)?;
        }
        "search"=>{
            todo!("search")
        }
        "serve"=>{
            let address=args.next().unwrap_or("127.0.0.1:8080".to_string());
            let server=Server::http(&address).map_err(|_|{
                eprintln!("ERROR: unable to start server: {}", address);
                std::process::exit(1);
            })?;
            println!("Listening on http://{}", address);
            for req in server.incoming_requests() {
                println!("INFO: received request! method: {:?}, url: {:?}",req.method(), req.url());
                let resp=Response::from_string("hello");
                req.respond(resp).unwrap_or_else(|err|{
                    eprintln!("ERROR: failed to respond to response: {}", err);
                });
            }
            todo!("serve")
        }
        _=>{

        }
    }

    for file_path in std::env::args().skip(1).take(1) {
        read_xml_dir_and_write(file_path, "index.json")?;
    }
    Ok(())
}
fn usage_and_error(subcommand: &str)->Box<dyn std::error::Error>{
    println!("Usage: [subcommand] [arg] [options]");
    println!("subcommand index: index for the next arg as the input file and the second arg for output file if there is.Default it will output to 'index.json'");
    println!("subcommand search: search the current directory if it does not exist");
    println!("subcommand serve: init the http server for net,you can add a optional[arg] for the address");
    println!();
    eprintln!("ERROR: unknown subcommand: {subcommand}");
    std::process::exit(1);
}