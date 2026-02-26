mod coffee;
mod storage;

fn main() {
    let path = dirs::data_dir()
        .expect("could not find data directory")
        .join("Dialed In");
}
