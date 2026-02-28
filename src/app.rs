use std::path::Path;

use uuid::Uuid;

use crate::coffee::Coffee;
use crate::storage::{load, store};

fn add(coffee: Coffee, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut coffees = load(path)?;
    coffees.push(coffee);
    store(coffees, path)?;
    Ok(())
}

fn update(coffee: Coffee, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut coffees = load(path)?;
    for cup in coffees.iter_mut() {
        if cup.id == coffee.id {
            *cup = coffee;
            break;
        }
    }
    store(coffees, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

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

    #[test]
    fn test_update() {
        let id = Uuid::new_v4();
        let initial_coffee = pour_coffee(id);
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        add(initial_coffee, path).expect("could not add coffee");

        let mut updated_coffee = pour_coffee(id);
        updated_coffee.notes = Some("Wauw!".to_string());
        update(updated_coffee, path).expect("could not update coffee");
        let coffees = load(path);

        let mut expected_coffee = pour_coffee(id);
        expected_coffee.notes = Some("Wauw!".to_string());

        assert_eq!(
            coffees
                .expect("no beans in grinder")
                .first()
                .expect("first bag is empty"),
            &expected_coffee
        );
    }
}
