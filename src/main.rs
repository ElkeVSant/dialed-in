mod app;
mod coffee;
mod storage;

#[cfg(test)]
mod test_utils;

fn main() {
    let path = dirs::data_dir()
        .expect("could not find data directory")
        .join("Dialed In");
}
