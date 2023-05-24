use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Status>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("{:-<29}", "");
    println!("Epic Name:");
    let name = get_user_input();
    println!("Epic Description:");
    let description = get_user_input();

    return Epic::new(name, description);
}

fn create_story_prompt() -> Story {
    println!("{:-<29}", "");
    println!("Story Name:");
    let name = get_user_input();
    println!("Story Description:");
    let description = get_user_input();

    return Story::new(name, description);
}

fn delete_epic_prompt() -> bool {
    println!("{:-<29}", "");
    loop {
        println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted (Y/n):");
        match get_user_input().as_str() {
            "Y" | "y" => return true,
            "N" | "n" => return false,
            _ => {
                println!("Invalid input! Please try again.");
                continue;
            }
        };
    }
}

fn delete_story_prompt() -> bool {
    println!("{:-<29}", "");
    loop {
        println!("Are you sure you want to delete this story? (Y/n):");
        match get_user_input().as_str() {
            "Y" | "y" => return true,
            "N" | "n" => return false,
            _ => {
                println!("Invalid input! Please try again.");
                continue;
            }
        };
    }
}

fn update_status_prompt() -> Status {
    println!("{:-<29}", "");
    loop {
        println!("Please enter new status. (In [P]rogress/[C]losed/[R]esolved):");
        match get_user_input().as_str() {
            "P" | "p" => return Status::InProgress,
            "C" | "c" => return Status::Closed,
            "R" | "r" => return Status::Resolved,
            _ => {
                println!("Invalid input! Please try again.");
                continue;
            }
        };
    }
}
