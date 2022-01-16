use crate::ContactBook;
use crate::menu_functionality::*;

pub fn start_application(){
    println!("**************************");
    println!("****** Contact Book ******");
    println!("**************************");
}

pub fn main_menu() {
    fn show(){
        println!("******* Main Menu ********");
        println!("     1 - Import Contacts");
        println!("     2 - View Contacts");
        println!("     3 - Add new Contact");
        println!("     4 - Edit Contact");
        println!("     5 - Remove Contact");
        println!("     6 - Search Contact");
        println!("     7 - Exit");
        println!("**************************");
        println!("   Enter your selection: ");
    }
    println!("new Agenda created");
    let mut contact_book = ContactBook::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };

        match input.as_str() {
            "1" => match import_contacts_menu(&mut contact_book){
                Ok(()) => (),
                Err(e) => ()
            },
            "2" => view_contacts_menu(&contact_book),
            "3" => add_contact_menu(&mut contact_book),
            "4" => update_contacts_menu(&mut contact_book),
            "5" => remove_contact_menu(&mut contact_book),
            "6" => search_menu(&contact_book),
            "7" => break,
            _ => {}
        }
    }
}

fn search_menu(contact_book: &ContactBook){
    fn show(){
        println!("****** Search Menu *******");
        println!("   1 - Search with Name");
        println!("   2 - Search with Email");
        println!("   3 - Search with Id");
        println!("   4 - Exit");
        println!("**************************");
        println!("   Enter your selection: ");
    }

    loop{
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };

        match input.as_str() {
            "1" => search_contact_menu(contact_book,Search::Name),
            "2" => search_contact_menu(contact_book,Search::Email),
            "3" => search_contact_menu(contact_book,Search::Id),
            "4" => break,
            _ => {}
        }
    }
}

pub enum Search {
    Name,
    Email,
    Id
}