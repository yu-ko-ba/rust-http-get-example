fn main() {
    let body = reqwest::blocking::get("http://www.example.com");

    println!("body = {:?}", body);
}
