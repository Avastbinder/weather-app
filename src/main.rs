use std::error::Error;
use std::fs;
use std::io::prelude::*;
use serde_json;

fn get_json(api: &str, location: &str) -> Result<String, Box<dyn Error>>
{
    let key: &str = "c66fe44192444404b73205455252402";
    
    let url = format!("http://api.weatherapi.com/v1/{api}.json?key={key}&q={location}&days=7");

    let response = reqwest::blocking::get(&url)?.text()?;

    Ok(response)
}


fn main() {
    match get_json("current", "83440")
    {
        Ok(json) => fs::write("./src/output.json", json).expect("Unable to write file"),
        Err(e) => eprintln!("Error fetching data: {e}"),
    }
    
    let file = fs::File::open("./src/output.json")
        .expect("File could not be opened");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File could not be read");

    println!("{}", json["current"]["wind_mph"]);

}
