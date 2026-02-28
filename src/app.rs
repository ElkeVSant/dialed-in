use std::collections::HashSet;
use std::path::Path;

use uuid::Uuid;

use crate::coffee::Coffee;
use crate::storage::{load, store};

fn add_coffee(mut coffee: Coffee, path: &Path) -> Result<Uuid, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();
    coffee.id = id;
    let mut coffees = load(path)?;
    coffees.push(coffee);
    store(coffees, path)?;
    Ok(id)
}

fn update_coffee(coffee: Coffee, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

fn list_coffees(path: &Path) -> Result<Vec<Coffee>, Box<dyn std::error::Error>> {
    let coffees = load(path)?;
    Ok(coffees)
}

fn list_origins(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let coffees = list_coffees(path)?;
    let origins: HashSet<String> = coffees.iter().filter_map(|c| c.origin.clone()).collect();
    let mut origins: Vec<String> = origins.into_iter().collect();
    origins.sort();
    Ok(origins)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::test_utils::pour_coffee;

    #[test]
    fn test_add_coffee() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let id = add_coffee(coffee, path).expect("could not add coffee");
        let coffees = list_coffees(path);

        let mut expected_coffee = pour_coffee();
        expected_coffee.id = id;

        assert_eq!(
            coffees
                .expect("no beans in grinder")
                .last()
                .expect("last bag is empty"),
            &expected_coffee
        );
    }

    #[test]
    fn test_update_coffee() {
        let initial_coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let id = add_coffee(initial_coffee, path).expect("could not add coffee");

        let mut updated_coffee = pour_coffee();
        updated_coffee.id = id;
        updated_coffee.notes = Some("Wauw!".to_string());
        update_coffee(updated_coffee, path).expect("could not update coffee");
        let coffees = list_coffees(path);

        let mut expected_coffee = pour_coffee();
        expected_coffee.id = id;
        expected_coffee.notes = Some("Wauw!".to_string());

        assert_eq!(
            coffees
                .expect("no beans in grinder")
                .first()
                .expect("first bag is empty"),
            &expected_coffee
        );
    }

    #[test]
    fn test_list_origins() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let origin = coffee.origin.clone();
        add_coffee(coffee, path).expect("could not add coffee");

        let origins = list_origins(path);

        assert_eq!(
            origin.expect("coffee comes from nowhere"),
            origins
                .expect("coffee not retrace coffee origins")
                .into_iter()
                .next()
                .expect("coffee comes from nowhere")
        );
    }
}
