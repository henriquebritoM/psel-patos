use std::str::FromStr;
use http_parser::HttpRequest;

fn main() {
    println!("Hello, world!");

    //  Manda umas requests para testar a lib

    // let request = "GET / HTTP/1.1\r\nheader\r\n\r\nbody\r\n\r\n";
    // let request = "POST /test/demo_form.php HTTP/1.1\r\nHost: w3schools.com\r\n\r\nname1=value1&name2=value2\r\n\r\n";
    let request = "POST  HTTP/1.1\r\nHost: w3schools.com\r\n\r\nname1=value1&name2=value2\r\n\r\n";

    let http = HttpRequest::from_str(request).unwrap();
    println!("{:?}", http);

}
