use reqwest;
use std::io::stdin;

#[tokio::main]
async fn req(url:&String) -> Result<(),reqwest::Error> {

    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    println!("Body: {resp:#?}");
    
    Ok(())
}

fn main() {
    println!("Enter your url: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Not a string");
    let err_check = req(&input);
    match err_check {
        Ok(()) => println!("Request suceeded"),
        Err(error) => panic!("Error! {error:?}"),
    };
}