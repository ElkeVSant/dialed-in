use std::collections::HashSet;
use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::coffee::Coffee;
use crate::storage::{load, store};

pub fn add_coffee(mut coffee: Coffee, path: &Path) -> Result<Uuid, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();
    coffee.id = id;
    let timestamp = Utc::now();
    coffee.created_at = timestamp;
    coffee.updated_at = timestamp;
    let mut coffees = load(path)?;
    coffees.push(coffee);
    store(coffees, path)?;
    Ok(id)
}

pub fn update_coffee(mut coffee: Coffee, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    coffee.updated_at = Utc::now();
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

pub fn delete_coffee(id: Uuid, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut coffees = load(path)?;
    coffees.retain(|c| c.id != id);
    store(coffees, path)?;
    Ok(())
}

pub fn list_coffees(path: &Path) -> Result<Vec<Coffee>, Box<dyn std::error::Error>> {
    let coffees = load(path)?;
    Ok(coffees)
}

pub fn list_origins(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    list_string_fields(path, |c| c.origin.clone())
}

pub fn list_varieties(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let coffees = list_coffees(path)?;
    let varieties: HashSet<String> = coffees
        .iter()
        .filter_map(|c| c.varieties.as_ref())
        .flatten()
        .cloned()
        .collect();
    let mut varieties: Vec<String> = varieties.into_iter().collect();
    varieties.sort();
    Ok(varieties)
}

pub fn list_processes(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    list_string_fields(path, |c| c.process.clone())
}

pub fn list_decaffeination_processes(
    path: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    list_string_fields(path, |c| c.decaffeination_process.clone())
}

pub fn list_roasters(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    list_string_fields(path, |c| c.roaster.clone())
}

fn list_string_fields(
    path: &Path,
    extractor: impl Fn(&Coffee) -> Option<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let coffees = list_coffees(path)?;
    let values: HashSet<String> = coffees.iter().filter_map(extractor).collect();
    let mut values: Vec<String> = values.into_iter().collect();
    values.sort();
    Ok(values)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::test_utils::{pour_coffee, pour_decaf};

    #[test]
    fn test_add_coffee() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let id = add_coffee(coffee, path).expect("could not add coffee");
        let coffees = list_coffees(path).expect("no beans in grinder");
        let created_coffee = coffees.first().expect("bag is empty");

        let mut expected_coffee = pour_coffee();
        expected_coffee.id = id;
        expected_coffee.created_at = created_coffee.created_at;
        expected_coffee.updated_at = created_coffee.updated_at;

        assert_eq!(created_coffee, &expected_coffee);
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
        let coffees = list_coffees(path).expect("no beans in grinder");
        let updated_coffee = coffees.first().expect("bag is empty");

        let mut expected_coffee = pour_coffee();
        expected_coffee.id = id;
        expected_coffee.notes = Some("Wauw!".to_string());
        expected_coffee.created_at = updated_coffee.created_at;
        expected_coffee.updated_at = updated_coffee.updated_at;

        assert_eq!(updated_coffee, &expected_coffee);
    }

    #[test]
    fn test_delete_coffee() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let id = add_coffee(coffee, path).expect("could not add coffee");

        delete_coffee(id, path).expect("could not purge grinder");

        let coffees = list_coffees(path);

        assert!(coffees.expect("no beans in grinder").is_empty());
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
            vec![origin.expect("coffee comes from nowhere")],
            origins.expect("could not retrace coffee origins")
        );
    }

    #[test]
    fn test_list_varieties() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let expected_varieties = coffee.varieties.clone().expect("coffee varieties unknown");
        add_coffee(coffee, path).expect("could not add coffee");

        let listed_varieties = list_varieties(path);

        assert_eq!(
            expected_varieties,
            listed_varieties.expect("coffee not find coffee varieties")
        );
    }

    #[test]
    fn test_list_processes() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let process = coffee.process.clone();
        add_coffee(coffee, path).expect("could not add coffee");

        let processes = list_processes(path);

        assert_eq!(
            vec![process.expect("is coffee unprocessed?")],
            processes.expect("no coffees were processed")
        );
    }

    #[test]
    fn test_list_decaffeination_processes() {
        let coffee = pour_decaf();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let decaffeination_process = coffee.decaffeination_process.clone();
        add_coffee(coffee, path).expect("could not add coffee");

        let decaffeination_processes = list_decaffeination_processes(path);

        assert_eq!(
            vec![decaffeination_process.expect("is coffee not processed for decaffeination?")],
            decaffeination_processes.expect("no coffees were processed for decaffeination")
        );
    }

    #[test]
    fn test_list_roasters() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();

        let roaster = coffee.roaster.clone();
        add_coffee(coffee, path).expect("could not add coffee");

        let roasters = list_roasters(path);

        assert_eq!(
            vec![roaster.expect("is coffee unroasted?")],
            roasters.expect("no one is roasting coffee")
        );
    }
}
