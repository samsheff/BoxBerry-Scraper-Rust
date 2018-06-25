extern crate threadpool;
use threadpool::ThreadPool;

extern crate reqwest;

extern crate clap;
use clap::{Arg, App};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    track_id: String,
    NameIM: String,
    ProgramNumber: String,
    order_id: String,
    Weight: String,
    delivery_type: String,
    ForingParcel: String,
    point_city: String,
    point_address: String,
    point_phone: String,
    Code: String,
    store_date: String,
    delivery_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackingCode {
    status: i16,
    token: Option<String>,
    data: Option<Vec<Package>>,
}

pub static BASEURL: &str = "http://boxberry.ru/tracking.service/tracking.php?check_captcha=1&g_recaptcha_response=0";
pub static LETTERS: &'static [&str] = &["A", "B", "C", "D"];

fn main() {
    let n_workers = 24;
    let pool = ThreadPool::new(n_workers);

    for y in LETTERS.iter() {
        for x in 0..10000 {
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
            println!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}", item.track_id, item.NameIM, item.ProgramNumber, item.order_id, item.Weight, item.delivery_type, item.ForingParcel, item.point_city, item.point_address, item.point_phone, item.Code, item.store_date, item.delivery_date);
        }
    }
}
