#[macro_use] extern crate quick_error;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate hyper;

pub mod error;

use serde_json::Value;
use chrono::{NaiveDate, NaiveTime};
use std::str;
use std::str::FromStr;
use std::io::Read;
use error::Error;
use hyper::Client;
use hyper::status::StatusCode;



pub fn get_weather<L: Into<String>>(location: L) -> Result<Weather, Error> {
    // get the json values
    let json: Value = serde_json::from_str(&get_raw_data(location)?)?;
    // define the root data point
    let json = json.pointer("/query/results/channel").ok_or(Error::NoData)?;

    // set the weather
    let mut weather = Weather {
        temp: json.pointer("/item/condition/temp").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        temp_unit: json.pointer("/units/temperature").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        condition_code: usize::from_str(json.pointer("/item/condition/code").and_then(|v| v.as_str()).unwrap_or("")).unwrap_or(3200),
        condition: json.pointer("/item/condition/text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        sunrise: NaiveTime::parse_from_str(json.pointer("/astronomy/sunrise").and_then(|v| v.as_str()).unwrap_or(""), "%l:%M %P")?,
        sunset: NaiveTime::parse_from_str(json.pointer("/astronomy/sunset").and_then(|v| v.as_str()).unwrap_or(""), "%l:%M %P")?,
        forecast: vec!()
    };

    // fill the forecast list with the data from the json
    for point in json.pointer("/item/forecast").ok_or(Error::NoData)?.as_array().ok_or(Error::NoData)? {
        weather.forecast.push(DataPoint {
            date: NaiveDate::parse_from_str(point.get("date").and_then(|v| v.as_str()).unwrap_or(""), "%d %b %Y")?,
            temp_high: point.get("high").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            temp_low: point.get("low").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            condition_code: usize::from_str(point.get("code").and_then(|v| v.as_str()).unwrap_or("")).unwrap_or(3200),
            condition: point.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        });
    }

    Ok(weather)
}

/// Request the data fromt the yahoo api and return the
/// result as String.
fn get_raw_data<L: Into<String>>(location: L) -> Result<String, Error> {
    // request the data
    let client = Client::new();
    let mut res = client.get(
        format!("http://query.yahooapis.com/v1/public/yql?q=select%20*%20from%20weather.forecast%20where%20woeid%20in%20(select%20woeid%20from%20geo.places(1)%20where%20text%3D%22{}%2C%20de%22)&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys", location.into())
        .as_str())
        .send()?;

    // check the status code response
    if res.status != StatusCode::Ok {
        return Err(Error::Other("No status code ok, returned"));
    }

    // read the response body
    let mut buf = String::new();
    res.read_to_string(&mut buf)?;

    Ok(buf)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
    temp: String,
    temp_unit: String,
    condition_code: usize,
    condition: String,
    sunrise: NaiveTime,
    sunset: NaiveTime,
    forecast: Vec<DataPoint>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPoint {
    date: NaiveDate,
    temp_high: String,
    temp_low: String,
    condition_code: usize,
    condition: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Weather: {:?}", get_weather("Ransbach-Baumbach").unwrap());
    }
}
