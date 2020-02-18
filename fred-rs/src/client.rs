
pub use reqwest::blocking::{Client, Response};

use std::time::Duration;
use std::env;

use crate::*;

const FRED_BASE_URL: &str = "https://api.stlouisfed.org/fred/";
const FRED_API_KEY: &str = "FRED_API_KEY";

pub struct FredClient {
    client: Client,
    url_base: &'static str,
    api_key: String,
}

impl FredClient {

    pub fn new() -> Result<FredClient, String> {

        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(c) => c,
            Err(msg) => return Err(msg.to_string()),
        };

        let api_key = match env::var(FRED_API_KEY) {
            Ok(val) => val,
            Err(_) => return Err(String::from("FRED_API_KEY not found.")),
        };

        let fred = FredClient {
            client,
            url_base: FRED_BASE_URL,
            api_key,
        };

        let url = format!("{}category?category_id=125&api_key={}&file_type=json", fred.url_base, fred.api_key);
        match fred.client.get(url.as_str()).send() {
            Ok(_) => (),
            Err(msg) => return Err(msg.to_string()),
        }

        return Ok(fred)

    }

    fn get_request(&mut self, url: &str) -> Result<Response, String> {
        match self.client.get(url).send() {
            Ok(r) => Ok(r),
            Err(msg) => Err(msg.to_string()),
        }
    }

    // ----------------------------------------------------------------------
    // Series

    pub fn series_observation(
        &mut self,
        series_id: &str,
        opt_builder: Option<series::ObservationBuilder>
    ) -> Result<series::ObservationResponse, String> {
        let mut url: String = format!(
            "{}series/observations?series_id={}&api_key={}&file_type=json",
            self.url_base,
            series_id,
            self.api_key
        );

        match opt_builder {
            Some(builder) => url.push_str(builder.options().as_str()),
            None => (),
        }

        match self.get_request(url.as_str()) {
            Ok(resp) => {
                match serde_json::from_str(&resp.text().unwrap()) {
                    Ok(val) => Ok(val),
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        match FredClient::new() {
            Ok(_) => assert_eq!(1, 1),
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1)
            },
        }
    }

    #[test]
    fn series_observation() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: series::ObservationResponse = match c.series_observation("GNPCA", None) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        /*for item in resp.observations {
            println!("{}: {}", item.date, item.value.parse::<f64>().unwrap());
        }*/
        assert_eq!(resp.observations[0].value, String::from("1120.076"));
    }

    #[test]
    fn series_observation_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut opt_builder = series::ObservationBuilder::new();
        opt_builder
            .observation_start("2000-01-01")
            .units(series::ObservationUnits::PCH)
            .frequency(series::ObservationFrequency::M);

        let resp: series::ObservationResponse = match c.series_observation("UNRATE", Some(opt_builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.observations {
            println!("{}: {}", item.date, item.value.parse::<f64>().unwrap());
        }
        //assert_eq!(resp.observations[0].value, String::from("1120.076"));
    } 
}