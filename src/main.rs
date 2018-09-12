extern crate threadpool;
use threadpool::ThreadPool;

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::{Deserialize, Deserializer};


fn default_resource() -> String {
    "".to_string()
}

fn nullable_resource<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(default_resource))
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    track_id: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    NameIM: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    ProgramNumber: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    order_id: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    Weight: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    delivery_type: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    ForingParcel: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    point_city: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    point_address: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    point_phone: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    Code: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    store_date: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    delivery_date: String,
    #[serde(default = "default_resource", deserialize_with = "nullable_resource")]
    ref_track: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackingCode {
    status: i16,
    token: Option<String>,
    data: Option<Vec<Package>>,
}

pub static BASEURL: &str = "https://boxberry.ru/tracking.service/tracking_service.php?check_captcha=1&g_recaptcha_response=0";
pub static LETTERS: &'static [&str] = &["A", "B", "C", "D"];

fn main() {
    let n_workers = 24;
    let pool = ThreadPool::new(n_workers);

    for y in LETTERS.iter() {
        for x in 0..100000 {
            pool.execute(move|| {
                get_package(&format!("{}", x), y);
            });
        }
    }
    pool.join();
}

fn get_package(number: &str, letter: &str) {
    let client = reqwest::Client::new();
    let mut request = client.get(&format!("{}&id={}{}", BASEURL, number, letter));
    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    //println!("{}", resp.text().unwrap());

    print_packages(serde_json::from_str(&resp.text().unwrap()).unwrap());
}

fn print_packages(tracking_code: TrackingCode) {
    if tracking_code.status == 2 {
        let result = tracking_code.data.unwrap();

        for item in result {
            println!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}", item.track_id, item.NameIM, item.ProgramNumber, item.order_id, item.Weight, item.delivery_type, item.ForingParcel, item.point_city, item.point_address, item.point_phone, item.Code, item.store_date, item.delivery_date, item.ref_track);
        }
    }
}
