mod app;
mod coffee;
mod storage;
mod ui;

#[cfg(test)]
mod test_utils;

fn main() {
    ui::run().expect("failed to run application");
}
