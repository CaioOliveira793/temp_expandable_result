#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct CollaboratorResult {
    id: String,
    name: String,
}

fn get_collaborator_result_somehow(ids: &[String]) -> Vec<CollaboratorResult> {
    let mut unique_ids = HashSet::<String>::with_capacity(ids.len());
    let mut results = Vec::<CollaboratorResult>::new();

    for id in ids.iter() {
        if unique_ids.contains(id) {
            continue;
        }

        unique_ids.insert(id.clone());
        results.push(CollaboratorResult {
            id: id.clone(),
            name: "fake".into(),
        });
    }

    results
}

// Expansion system

trait CollaboratorExpandableResult<'a> {
    fn get_collaborator_ids(&self) -> Vec<String>;
    fn set_collaborators(&mut self, collaborators: &'a [CollaboratorResult]) -> ();
}

// Expansion Integration

#[derive(Debug, Clone)]
struct ObservationResult {
    id: String,
    message: String,
    contract_id: String,
    owner_id: String,
    referenced_id: String,

    // Expaned:
    owner: Option<CollaboratorResult>,
    referenced: Option<CollaboratorResult>,
}

impl<'a> ObservationResult {
    fn new() -> ObservationResult {
        ObservationResult {
            id: "observation:1".into(),
            message: "contrato averbado".into(),
            contract_id: "contract".into(),
            owner_id: "owner_id:2".into(),
            referenced_id: "referenced_id:3".into(),
            owner: None,
            referenced: None,
        }
    }

    fn delete_message(&mut self) {
        self.message = "".into();
    }
}

impl<'a> CollaboratorExpandableResult<'a> for ObservationResult {
    fn get_collaborator_ids(&self) -> Vec<String> {
        vec![self.owner_id.clone(), self.referenced_id.clone()]
    }

    fn set_collaborators(&mut self, collaborators: &'a [CollaboratorResult]) {
        for collaborator in collaborators.iter() {
            if self.owner_id == collaborator.id {
                self.owner = Some(collaborator.clone());
            }
            if self.referenced_id == collaborator.id {
                self.referenced = Some(collaborator.clone());
            }
        }
    }
}

fn expand_obs(obss: &mut [ObservationResult]) {
    let collaborators = get_collaborator_result_somehow(
        &obss
            .iter()
            .flat_map(|x| x.get_collaborator_ids())
            .collect::<Vec<String>>(),
    );

    for obs in obss.iter_mut() {
        obs.set_collaborators(&collaborators);
    }
}

fn main() {
    let mut obss = [
        ObservationResult::new(),
        ObservationResult::new(),
        ObservationResult::new(),
        ObservationResult::new(),
    ];

    expand_obs(&mut obss);

    println!("{:?}", &obss);
}
