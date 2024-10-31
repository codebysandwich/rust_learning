// File       : main.rs
// Time       ：2024/10/9 15:14
// Author     ：sandwich
// version    ：V1.0
// Description：

extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

use rust_pl::utils::fn_tools::gcd;

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Caculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let from_data = match request.get_ref
        ::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_numbers = match from_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut("from data has no 'n' parameter\n");
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(unparsed) {
            Err(e) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", e));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n) }
        }
    }
    let mut d = numbers[0];
    for &m in &numbers[1..] {
        d = gcd(d, m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of numbers: {:?} is <b>{}</b>", numbers, d)
    );
    Ok(response)
}

fn main() {
    let mut router = Router::new();

    router.get("/", get_from, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Server on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}
