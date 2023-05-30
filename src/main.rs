use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new("data/db.json".to_owned()));
    let mut nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        let current_page_optional = nav.get_current_page();
        if let None = current_page_optional {
            println!("Error: No current page found!");
            break;
        }

        let current_page = current_page_optional.unwrap();

        if let Err(error) = current_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress Enter to continue...",
                error
            );
            wait_for_key_press();
        }

        let input = get_user_input();

        match current_page.handle_input(input.trim()) {
            Err(error) => {
                println!(
                    "Error handling input: {}\nPress Enter to continue...",
                    error
                );
                wait_for_key_press();
            }
            Ok(potential_action) => {
                if let Some(action) = potential_action {
                    if let Err(error) = nav.handle_action(action) {
                        println!(
                            "Error occurred handling action: {}\nPress Enter to continue...",
                            error
                        );
                        wait_for_key_press();
                    }
                }
            }
        };
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
