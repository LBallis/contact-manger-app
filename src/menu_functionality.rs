use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::ptr::null;
use serde::Deserialize;
use crate::{Contact, ContactBook, Search};

pub fn import_contacts_menu(my_agenda: &mut ContactBook) -> Result<(), Box<dyn Error>>{
    println!("Enter filename: ");
    let filename = match get_input() {
        Some(input) => input,
        None => { "".to_owned() }
    };
    let mut rdr = csv::Reader::from_path(filename);
    for contact in rdr.unwrap().deserialize(){
        let contact:Contact = contact?;
        my_agenda.add(contact);
    }
    Ok(())
}

pub fn view_contacts_menu(contact_book: &ContactBook){
    for contact in contact_book.get_all(){
        println!("{:?}", contact);
    }
}

pub fn add_contact_menu(contact_book: &mut ContactBook){
    println!("Contact name: ");
    let name = match get_input(){
        Some(name) => name,
        None => return
    };
    println!("Contact email: ");
    let email = match get_input() {
        Some(email) => email,
        None => return
    };
     contact_book.add(Contact::new(contact_book.next_id(), name, email));
    println!("Contact added");

}

pub fn update_contacts_menu(contact_book: &mut ContactBook) {
    if contact_book.inner.is_empty(){
        println!("There are no contacts to update");
        return;
    }

    view_contacts_menu(contact_book);
    println!("Which contact you want to change?");
    let contact_id = match get_contact_id() {
        Some(c_id) => c_id,
        None => return
    };
    println!("Name: ");
    let name = match get_input() {
        Some(name) => name,
        None => return
    };
    println!("Email: ");
    let email = match get_input() {
        Some(email) => email,
        None => return
    };
    if contact_book.update(&contact_id, &name, &email) {
        println!("Contact updated");
    }else{
        println!("Contact not found");
    }
}

pub fn remove_contact_menu(contact_book:&mut ContactBook){
    if contact_book.inner.is_empty(){
        println!("There are no contacts to remove");
        return;
    }

    view_contacts_menu(contact_book);
    println!("Which contact you want to remove?");
    let contact_id = match get_contact_id() {
        Some(c_id) => c_id,
        None => return
    };
    if contact_book.remove(&contact_id) {
        println!("Contact removed");
    }else {
        println!("Contact not found");
    }
}

pub fn search_contact_menu(contact_book: &ContactBook, search_type: Search) {
    match search_type {
        Search::Name => {
            let name_id_map:HashMap<String,i32> = fill_name_id_map(contact_book);
            println!("Name: ");
            let name = match get_input() {
                Some(name) => match name_id_map.get(&name){
                    Some(id)=>println!("{:?}",contact_book.inner.get(id)),
                    None => return
                },
                None => return
            };
        }
        Search::Email => {
            let email_id_map = fill_email_id_map(contact_book);
            println!("Email: ");
            let email = match get_input() {
                Some(email) => match email_id_map.get(&email) {
                    Some(id) => println!("{:?}",contact_book.inner.get(id)),
                    None => return
                },
                None => return
            };
        }
        Search::Id => {
            let id = match get_contact_id() {
                Some(id) => println!("{:?}",contact_book.inner.get(&id)),
                None => return
            };
        }
    };

}



pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    };
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn get_contact_id() -> Option<i32> {
    println!("Id: ");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        let parsed_id: Result<i32, _> = input.parse();
        match parsed_id {
            Ok(id, ..) => return Some(id),
            Err(_) => println!("Please enter a number")
        }
    }
}

fn fill_name_id_map(contact_book: &ContactBook) -> HashMap<String, i32>{
    let mut map:HashMap<String,i32> = HashMap::new();
    for contact in contact_book.inner.values(){
        map.insert(contact.get_name(), contact.get_id());
    }
    map
}

fn fill_email_id_map(contact_book: &ContactBook) -> HashMap<String, i32>{
    let mut map:HashMap<String,i32> = HashMap::new();
    for contact in contact_book.inner.values(){
        map.insert(contact.get_email(), contact.get_id());
    }
    map
}



