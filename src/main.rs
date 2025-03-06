use std::error::Error;

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
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Error fetching data: {e}"),
    }
    
}
