#![allow(dead_code)]

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone)]
struct CollaboratorResult {
    id: String,
    name: String,
}

#[derive(Debug, Clone)]
struct ContractResult {
    id: String,
    name: String,
    contract_type: String,
}

fn get_collaborator_result_somehow(ids: &[String]) -> Vec<CollaboratorResult> {
    let mut results = Vec::<CollaboratorResult>::new();
    let mut unique_ids = HashSet::<String>::with_capacity(ids.len());
    unique_ids.extend(ids.into_iter().map(Clone::clone));

    for id in unique_ids {
        results.push(CollaboratorResult {
            id,
            name: "fake".into(),
        });
    }

    results
}

fn get_contract_result_somehow(ids: &[String]) -> Vec<ContractResult> {
    let mut results = Vec::<ContractResult>::new();
    let mut unique_ids = HashSet::<String>::with_capacity(ids.len());
    unique_ids.extend(ids.into_iter().map(Clone::clone));

    for id in unique_ids {
        results.push(ContractResult {
            id,
            name: "fake".into(),
            contract_type: "new".into(),
        });
    }

    results
}

// Expansion system

trait ExpandableResult<T> {
    fn get_expand_ids(&self) -> Vec<&String>;
    fn set_expand_results(&mut self, collaborators: &[T]) -> ();
}

fn expand_result<T, F, R>(expandables: &mut [T], f: F)
where
    T: ExpandableResult<Rc<R>>,
    F: FnOnce(Vec<String>) -> Vec<Rc<R>>,
{
    let ids = expandables
        .iter()
        .flat_map(|x| x.get_expand_ids())
        .cloned()
        .collect::<Vec<String>>();
    let results = f(ids);
    for exp in expandables.iter_mut() {
        exp.set_expand_results(&results);
    }
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
    owner: Option<Rc<CollaboratorResult>>,
    referenced: Option<Rc<CollaboratorResult>>,
    contract: Option<Rc<ContractResult>>,
}

impl ObservationResult {
    fn new() -> ObservationResult {
        ObservationResult {
            id: "observation:1".into(),
            message: "contrato averbado".into(),
            contract_id: "contract".into(),
            owner_id: "owner_id:2".into(),
            referenced_id: "referenced_id:3".into(),
            owner: None,
            referenced: None,
            contract: None,
        }
    }

    fn delete_message(&mut self) {
        self.message = "".into();
    }
}

impl ExpandableResult<Rc<CollaboratorResult>> for ObservationResult {
    fn get_expand_ids(&self) -> Vec<&String> {
        vec![&self.owner_id, &self.referenced_id]
    }

    fn set_expand_results(&mut self, collaborators: &[Rc<CollaboratorResult>]) {
        for collaborator in collaborators.iter() {
            if self.owner_id == collaborator.id {
                self.owner = Some(Rc::clone(collaborator));
            }
            if self.referenced_id == collaborator.id {
                self.referenced = Some(Rc::clone(collaborator));
            }
        }
    }
}

impl ExpandableResult<Rc<ContractResult>> for ObservationResult {
    fn get_expand_ids(&self) -> Vec<&String> {
        vec![&self.contract_id]
    }

    fn set_expand_results(&mut self, contracts: &[Rc<ContractResult>]) {
        for contract in contracts.iter() {
            if self.contract_id == contract.id {
                self.contract = Some(Rc::clone(contract));
            }
        }
    }
}

fn main() {
    let mut obss = [ObservationResult::new()];
    println!("initial: {:#?}", &obss);
    for obs in obss.iter() {
        assert!(obs.contract.is_none());
        assert!(obs.owner.is_none());
        assert!(obs.referenced.is_none());
    }

    expand_result(&mut obss, |ids| {
        get_collaborator_result_somehow(&ids)
            .into_iter()
            .map(Rc::new)
            .collect()
    });
    println!("after collaborator: {:#?}", &obss);
    for obs in obss.iter() {
        assert!(obs.contract.is_none());
        assert!(obs.owner.is_some());
        assert!(obs.referenced.is_some());
    }

    expand_result(&mut obss, |ids| {
        get_contract_result_somehow(&ids)
            .into_iter()
            .map(Rc::new)
            .collect()
    });
    println!("after contract: {:#?}", &obss);
    for obs in obss.iter() {
        assert!(obs.contract.is_some());
        assert!(obs.owner.is_some());
        assert!(obs.referenced.is_some());
    }
}
