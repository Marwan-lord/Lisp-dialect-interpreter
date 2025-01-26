use crate::lsymc::Lsymc;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Default, PartialEq)]
pub struct Store {
    parent: Option<Rc<RefCell<Store>>>,
    syms: HashMap<String, Lsymc>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, data: Rc<RefCell<Self>>) {
        self.syms.extend(
            data.borrow()
                .syms
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            syms: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<Lsymc> {
        match self.syms.get(name) {
            Some(v) => Some(v.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|s| s.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, v: Lsymc) {
        self.syms.insert(name.to_owned(), v);
    }
}
