# BoxBerry Scraper
Multithreaded Boxberry Package Info Scraper in Rust

## What is this?
Boxberry (https://boxberry.ru) is a courier service in Russia. Their package tracking service has several critical security holes which allow the bypassing of captcha and enumeration of sensitive package information. This is a program which enumerates all packages currently being handled and outputs them in a CSV-like (Pipe serapated) format to the console.

## What are the security holes?
- No rate limiting.
  The program runs very quickly, using 24 threads by default and very quickly can get all packages without speed restrictions.
- Captcha Bypass.
  It's possible to bypass the captcha by setting the query parameter `check_captcha` to `1` and `g_recaptcha_response` to `0`.
- Super simple, enumerable tracking codes.
  Codes start with a number, which can be between 0 and around 100,000, and are followed by an uppercase letter, either A, B, C or D. Enumeration of this format is straightforward and simple, meaning it's easy to spy on other people's deliveries.
  
## How to use?
- Install rust
- `cargo build --release`
- `./target/release/boxberry`

You should see a steady stream of package info being output to the console.
