use std::path::Path;

use crate::coffee::Coffee;
use crate::storage::{load, store};

fn add(coffee: Coffee, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut coffees = load(path)?;
    coffees.push(coffee);
    store(coffees, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use uuid::Uuid;

    use super::*;
    use crate::test_utils::pour_coffee;

    #[test]
    fn test_add() {
        let id = Uuid::new_v4();
        let coffee = pour_coffee(id);
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        add(coffee, path).expect("could not add coffee");
        let coffees = load(path);

        assert_eq!(
            coffees
                .expect("no beans in grinder")
                .last()
                .expect("last bag is empty"),
            &pour_coffee(id)
        );
    }
}
