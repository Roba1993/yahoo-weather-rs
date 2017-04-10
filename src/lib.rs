#[macro_use] extern crate quick_error;
extern crate serde_json;
extern crate chrono;
extern crate measurements;
extern crate curl;

mod error;

use chrono::{NaiveDate, NaiveTime};
use measurements::Temperature;
use curl::easy::Easy;
use std::str;
use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;
use error::Error;
use std::str::FromStr;


pub fn get_weather<L: Into<&'static str>>(location: L) -> Result<Weather, Error> {
    // get the json values
    let json: Value = serde_json::from_str(&get_raw_data(location)?)?;
    // define the root data point
    let json = json.pointer("/query/results/channel").ok_or(Error::NoData)?;

    // get the temperature unit
    let temp_unit = json.pointer("/units/temperature").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?;
    if temp_unit != "F" {
        return Err(Error::Other("Only F from the API is supported right now"));
    }

    // set the weather
    let mut weather = Weather {
        temp: Temperature::from_fahrenheit(f64::from_str(json.pointer("/item/condition/temp").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?)?),
        condition_code:  usize::from_str(json.pointer("/item/condition/code").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?)?,
        condition: json.pointer("/item/condition/text").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?.to_string(),
        sunrise: NaiveTime::parse_from_str(json.pointer("/astronomy/sunrise").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?, "%l:%M %P")?,
        sunset: NaiveTime::parse_from_str(json.pointer("/astronomy/sunset").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?, "%l:%M %P")?,
        forecast: vec!()
    };

    // fill the forecast list with the data from the json
    for point in json.pointer("/item/forecast").ok_or(Error::NoData)?.as_array().ok_or(Error::NoData)? {
        weather.forecast.push(DataPoint {
            date: NaiveDate::parse_from_str(point.get("date").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?, "%d %b %Y")?,
            temp_high: Temperature::from_fahrenheit(f64::from_str(point.get("high").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?)?),
            temp_low: Temperature::from_fahrenheit(f64::from_str(point.get("low").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?)?),
            condition_code: usize::from_str(point.get("code").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?)?,
            condition: point.get("text").ok_or(Error::NoData)?.as_str().ok_or(Error::NoData)?.to_string()
        });
    }

    Ok(weather)
}

/// Request the data fromt the yahoo api and return the
/// result as String.
fn get_raw_data<L: Into<&'static str>>(location: L) -> Result<String, Error> {
    // define the empty contet string
    let content = Rc::new(RefCell::new("".to_string()));

    // prepare curl to load the data
    let mut curl = Easy::new();
    // define the url
    curl.url(format!("https://query.yahooapis.com/v1/public/yql?q=select%20*%20from%20weather.forecast%20where%20woeid%20in%20(select%20woeid%20from%20geo.places(1)%20where%20text%3D%22{}%2C%20de%22)&format=json&env=store%3A%2F%2Fdatatables.org%2Falltableswithkeys", location.into()).as_str()).unwrap();
    // define how to handle the content
    let mut transfer = curl.transfer();

    // define the value return
    transfer.write_function(|data| {
        match str::from_utf8(data) {
            Ok(v) => {
                content.borrow_mut().push_str(v);
                return Ok(data.len());
            },
            Err(_) => Ok(data.len())
        }
    })?;

    // execute
    transfer.perform()?;

    // return the content
    let x = content.borrow_mut().clone();
    Ok(x)
}

#[derive(Debug, Clone)]
pub struct Weather {
    temp: Temperature,
    condition_code: usize,
    condition: String,
    sunrise: NaiveTime,
    sunset: NaiveTime,
    forecast: Vec<DataPoint>
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    date: NaiveDate,
    temp_high: Temperature,
    temp_low: Temperature,
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
