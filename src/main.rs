mod districts;
mod io;

fn main() {
    for district in districts::load().iter() {
        println!("{:?}", district);
    }
}
