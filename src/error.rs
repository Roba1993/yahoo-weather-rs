use std::{io, num};
use curl;
use serde_json;
use chrono;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Curl(err: curl::Error) {
            from()
            description("curl error")
            display("Curl error: {}", err)
            cause(err)
        }
        Json(err: serde_json::Error) {
            from()
            description("json error")
            display("Json error: {}", err)
            cause(err)
        }
        Chrono(err: chrono::format::ParseError) {
            from()
            description("chrono error")
            display("Chrono error: {}", err)
            cause(err)
        }
        ParseInt(err: num::ParseIntError) {
            from()
            description("Parse integer error")
            display("Parse integer: {}", err)
            cause(err)
        }
        ParseFloat(err: num::ParseFloatError) {
            from()
            description("Parse float error")
            display("Parse float: {}", err)
            cause(err)
        }
        NoData {
            description("No data are returned")
            display("No data are returned")
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}
