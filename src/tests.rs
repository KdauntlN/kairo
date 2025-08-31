use super::*;

#[test]
fn correct_method() {
    let request = Request::parse("GET / HTTP/1.1").unwrap();
    assert_eq!(request.method, "GET");
}

#[test]
fn correct_uri() {
    let request = Request::parse("GET /login HTTP/1.1").unwrap();
    assert_eq!(request.uri, "/login");
}

#[test]
#[should_panic]
fn invalid_header() {
    let request = "GET / HTTP/1.1\r\nContent-Length: fifty-five\r\n\r\n";
    let _request = Request::parse(request).unwrap();
}