use std::{env,fs::File, io::{copy, Cursor}};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // parse args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3{
        eprintln!("Usage: wget_clone <URL> <OUTPUT_PATH>");
        std::process::exit(1);
    }
    
    let url = &args[1];
    let output_path = &args[2];

    // http request func
    let client = Client::new();
    let response = client.get(url).send().await?;

    if response.status().is_success(){
        let mut file = File::create(output_path)?;

        let content = response.bytes().await?;
        let mut content_cursor = Cursor::new(content);
        copy(&mut content_cursor, &mut file)?;
        println!("Downloaded content from url: {}, to file: {}", url, output_path);
    } else {
        eprintln!("The response was unsuccessful: {}", response.status());
    }

    Ok(())

}
