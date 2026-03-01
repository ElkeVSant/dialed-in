mod app;
mod coffee;
mod storage;
mod ui;

#[cfg(test)]
mod test_utils;

fn main() {
    let path = dirs::data_dir()
        .expect("could not find data directory")
        .join("Dialed In");

    ui::run().expect("failed to run application");
}
