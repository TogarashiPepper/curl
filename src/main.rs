use clap::Parser;
use curl::{create_builder_request, ArgHandler};
use std::fs;
//create flags in this order https://reqbin.com/req/c-skhwmiil/curl-flags

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    URL: String,

    #[clap(short, long, min_values=0)]
    //optional argument and optional value (see https://github.com/clap-rs/clap/blob/63a36673e1df54baded338b9830aff78e60148ae/examples/derive_ref/README.md#arg-types)
    output: Option<Option<String>>,

    #[clap(short, long)]
    method: Option<String>,

    #[clap(short, long)]
    body: Option<String>,

    #[clap(short, long)]
    port: Option<u16>
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let client = reqwest::Client::new();
    let method = match &args.method {
        Some(m) => m,
        _ => "GET"
    };

    println!("{:?}", args.body);

    let url = &args.URL;
    let resp = create_builder_request(url, method, client, args.port)
        .optional_body(args.body)
        .send();

    let resp = resp.await?;
    
    let headers = resp.headers().clone();
    let text = resp
        .text()
        .await?;
    //check if Output argument is present
    if let Some(i) = args.output {
        //check if argument value is present
        if let Some(e) = i {
            fs::write(e, &text).expect("Unable to write file");
            return Ok(())
        }
        //if not then get the value from the Content-Disposition header
        //potentially throw an error if file name not found through path or header
        else {
            let mut value = "Unknown_File_Name";
            if headers.contains_key("Content-Disposition") {
              value = headers["Content-Disposition"].to_str()?;
              println!("{}", value);
            }
            fs::write(value, &text).expect("Unable to write file");
            return Ok(())
        }
    }
    //if not then print to stdout
    else {
        println!("{}", &text);
    }
    
    Ok(())
}