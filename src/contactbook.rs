use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ops::Deref;
use crate::Contact;

#[derive(Debug)]
pub struct ContactBook {
    pub inner: HashMap<i32, Contact>
}

impl ContactBook {
    pub fn new() -> Self { Self{inner: HashMap::new()} }

    pub fn add(&mut self, contact: Contact){
        self.inner.insert(contact.get_id(), contact);
    }

    pub fn get_all(&self) -> Vec<&Contact>{
        let mut contacts = vec![];
        for contact in self.inner.values() {
            contacts.push(contact);
        }
        println!("{:?}", contacts.len());
        contacts
    }

    pub fn update(&mut self, id: &i32, name: &str, email: &str) -> bool {
        match self.inner.get_mut(id) {
            Some(contact) => {
                contact.set_name(name);
                contact.set_email(email);
                true
            }
            None => false
        }
    }

    pub fn remove(&mut self, contact_id: &i32) -> bool {
        self.inner.remove(contact_id).is_some()
    }

    pub fn next_id(&self) -> i32 {
        // First we just get all the keys (ids).
        // This vector is just a copy of the ids, so we can throw it
        // away when done.
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        // "pop" removes the last entry from the vector, so we will have
        // the largest ID currently in use.
        // Adding 1 to the largest ID gives us the next ID, and if none
        // were found, we just start at 1.
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }
}