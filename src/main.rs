mod districts;
mod io;
mod purposes;

fn main() {
    println!("{:?}", districts::load().unwrap());
    println!("{:?}", purposes::load().unwrap());
}
