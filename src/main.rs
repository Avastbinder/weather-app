use std::error::Error;
use std::fs;
use serde_json;
use iced::widget::{button, column, text, text_input, image};
use iced::Element;

#[derive(Default)]
struct State{
    location: String,
    weather_data: String,
    weather_icon: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    FetchWeather
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(location) => 
            {
                self.location = location;
            }

            Message::FetchWeather => {
                match get_weather(&self.location) {
                    Ok(data) => self.weather_data = data,
                    Err(_) => self.weather_data = "Error fetching data".to_string(),
                }

                match get_weather_icon(&self.location) {
                    Ok(icon_path) => self.weather_icon = icon_path,
                    Err(_) => self.weather_icon = "".to_string(),
                }
            }
        }
    }


    fn view(&self) -> Element<Message> {
        column![
            text_input("Type zip code, city name", &self.location)
                .on_input(Message::ContentChanged),
            button("Enter").on_press(Message::FetchWeather),
            image(self.weather_icon.clone()),
            text(&self.weather_data).size(25)
        ]
        .spacing(10)
        .padding(20)
        .into()
    }
}


fn get_json(api: &str, location: &str) -> Result<serde_json::Value, Box<dyn Error>>
{
    let key: &str = "c66fe44192444404b73205455252402";
    let url = format!("http://api.weatherapi.com/v1/{api}.json?key={key}&q={location}&days=7");
    let response = reqwest::blocking::get(&url)?.text()?;

    fs::write("./src/output.json", response).expect("Unable to write file");

    let file = fs::File::open("./src/output.json")
        .expect("File could not be opened");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File could not be read");

    Ok(json)
}


fn main() -> iced::Result {

    iced::run("Weather App - Aidan Vastbinder", State::update, State::view)

}


fn get_weather(location: &str) -> Result<String, Box<dyn Error>>
{
    let json = match get_json("current", location)
    {
        Ok(json) => json,
        Err(_e) => serde_json::Value::String("Error".to_string()),
    };

    let wind_speed = json["current"]["wind_mph"].to_string();
    let temp = json["current"]["temp_f"].to_string();
    let humidity = json["current"]["humidity"].to_string();
    let city = json["location"]["name"].as_str().unwrap_or("");
    let state = json["location"]["region"].as_str().unwrap_or("");
    let country = json["location"]["country"].as_str().unwrap_or("");
    let location = format!("{}, {}, {}\nLat: {}\nLong: {}", city, state, country, json["location"]["lat"], json["location"]["lon"]).to_string();

    Ok(format!("{location}\nCurrent Temperature: {temp}\nHumidity: {humidity}\nWind speed: {wind_speed}"))
}


fn get_weather_icon(location: &str) -> Result<String, Box<dyn Error>>
{
    let json = match get_json("current", location)
    {
        Ok(json) => json,
        Err(_e) => serde_json::Value::String("Error".to_string()),
    }.clone();

    let image = json["current"]["condition"]["icon"].as_str().unwrap_or("");

    let mut file = fs::File::create("./src/Weather_icon.png").unwrap();
    reqwest::blocking::get(format!("https:{image}"))
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    Ok("./src/Weather_icon.png".to_string())
}