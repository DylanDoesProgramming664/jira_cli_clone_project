#![allow(unused)]
use anyhow::{Ok, Result};
use std::rc::Rc;

use crate::{
    db::JiraDatabase,
    models::{Action, Status},
    ui::{EpicDetail, HomePage, Page, Prompts, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        return Self {
            pages: vec![Box::new(HomePage { db: Rc::clone(&db) })],
            prompts: Prompts::new(),
            db,
        };
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        // this should always return the last element in the pages vector
        return self.pages.last();
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                // create a new EpicDetail instance and add it to the pages vector
                let epic_page = EpicDetail {
                    epic_id,
                    db: self.db.clone(),
                };
                self.pages.push(Box::new(epic_page));
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                // create a new StoryDetail instance and add it to the pages vector
                let story_page = StoryDetail {
                    epic_id,
                    story_id,
                    db: self.db.clone(),
                };
                self.pages.push(Box::new(story_page));
            }
            Action::NavigateToPreviousPage => {
                // remove the last page from the pages vector
                self.pages.pop();
            }
            Action::CreateEpic => {
                // prompt the user to create a new epic and persist it in the database
                let new_epic = (self.prompts.create_epic)();
                self.db.create_epic(new_epic)?;
            }
            Action::CloseEpic { epic_id } => {
                // prompt the user to update status and persist it in the database
                if (self.prompts.close_epic)() {
                    self.db.close_epic(epic_id)?;
                    println!("Epic was closed!");
                } else {
                    println!("Cancelled!");
                }
            }
            Action::ReopenEpic { epic_id } => {
                if (self.prompts.reopen_epic)() {
                    self.db.update_epic_status(epic_id)?;
                    println!("Epic was reopened!");
                } else {
                    println!("Cancelled!");
                }
            }
            Action::DeleteEpic { epic_id } => {
                // prompt the user to delete the epic and persist it in the database
                if (self.prompts.delete_epic)() {
                    self.db.delete_epic(epic_id)?;
                    println!("Epic and attached stories were removed!");
                    self.pages.pop();
                } else {
                    println!("Cancelled!");
                }
            }
            Action::CreateStory { epic_id } => {
                // prompt the user to create a new story and persist it in the database
                let new_story = (self.prompts.create_story)();
                self.db.create_story(new_story, epic_id)?;
                println!("Story was created!");
            }
            Action::UpdateStoryStatus { epic_id, story_id } => {
                // prompt the user to update status and persist it in the database
                if self.db.read_db()?.epics[&epic_id].status == Status::Closed {
                    println!("Cannot change the status of a Story from a closed Epic!");
                } else {
                    let new_status = (self.prompts.update_status)();
                    self.db.update_story_status(story_id, new_status)?;
                    println!("Story status updated successfully!");
                }
            }
            Action::DeleteStory { epic_id, story_id } => {
                // prompt the user to delete the story and persist it in the database
                if (self.prompts.delete_story)() {
                    self.db.delete_story(epic_id, story_id)?;
                    self.pages.pop();
                }
            }
            Action::Exit => {
                // remove all pages from the pages vector
                self.pages.clear();
            }
        }

        Ok(())
    }

    // Private functions used for testing

    fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::test_utils::MockDB,
        models::{Epic, Status, Story},
    };

    #[test]
    fn should_start_on_home_page() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let nav = Navigator::new(db);

        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();

        assert_eq!(Some(()).is_some(), true);

        assert_eq!(home_page.is_some(), true);
    }

    #[test]
    fn handle_action_should_navigate_pages() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });

        let mut nav = Navigator::new(db);

        nav.handle_action(Action::NavigateToEpicDetail { epic_id: 1 })
            .ok()
            .unwrap();
        assert_eq!(nav.get_page_count(), 2);

        let current_page = nav.get_current_page().unwrap();
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToStoryDetail {
            epic_id: 1,
            story_id: 2,
        })
        .ok()
        .unwrap();
        assert_eq!(nav.get_page_count(), 3);

        let current_page = nav.get_current_page().unwrap();
        let story_detail_page = current_page.as_any().downcast_ref::<StoryDetail>();
        assert_eq!(story_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage)
            .ok()
            .unwrap();
        assert_eq!(nav.get_page_count(), 2);

        let current_page = nav.get_current_page().unwrap();
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage)
            .ok()
            .unwrap();
        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();
        assert_eq!(home_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage)
            .ok()
            .unwrap();
        assert_eq!(nav.get_page_count(), 0);

        nav.handle_action(Action::NavigateToPreviousPage)
            .ok()
            .unwrap();
        assert_eq!(nav.get_page_count(), 0);
    }

    #[test]
    fn handle_action_should_clear_pages_on_exit() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });

        let mut nav = Navigator::new(db);

        nav.handle_action(Action::NavigateToEpicDetail { epic_id: 1 })
            .ok()
            .unwrap();
        nav.handle_action(Action::NavigateToStoryDetail {
            epic_id: 1,
            story_id: 2,
        })
        .unwrap();
        nav.handle_action(Action::Exit).unwrap();

        assert_eq!(nav.get_page_count(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_epic() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.create_epic = Box::new(|| Epic::new("name".to_owned(), "description".to_owned()));

        nav.set_prompts(prompts);

        nav.handle_action(Action::CreateEpic).ok().unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.epics.len(), 1);

        let epic = db_state.epics.into_iter().next().unwrap().1;
        assert_eq!(epic.name, "name".to_owned());
        assert_eq!(epic.description, "description".to_owned());
    }

    #[test]
    fn handle_action_should_handle_close_epic() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });

        let epic_id = db
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .ok()
            .unwrap();

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.close_epic = Box::new(|| true);

        nav.set_prompts(prompts);

        nav.handle_action(Action::CloseEpic { epic_id })
            .ok()
            .unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.epics[&epic_id].status, Status::Closed);
    }

    #[test]
    fn handle_action_should_handle_delete_epic() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let epic_id = db
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .ok()
            .unwrap();

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.delete_epic = Box::new(|| true);

        nav.set_prompts(prompts);

        nav.handle_action(Action::DeleteEpic { epic_id })
            .ok()
            .unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.epics.len(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_story() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let epic_id = db
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .ok()
            .unwrap();

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.create_story = Box::new(|| Story::new("name".to_owned(), "description".to_owned()));

        nav.set_prompts(prompts);

        nav.handle_action(Action::CreateStory { epic_id })
            .ok()
            .unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.stories.len(), 1);

        let story = db_state.stories.into_iter().next().unwrap().1;
        assert_eq!(story.name, "name".to_owned());
        assert_eq!(story.description, "description".to_owned());
    }

    #[test]
    fn handle_action_should_handle_update_story() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let epic_id = db
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .ok()
            .unwrap();
        let story_id = db
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .ok()
            .unwrap();

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.update_status = Box::new(|| Status::InProgress);

        nav.set_prompts(prompts);

        nav.handle_action(Action::UpdateStoryStatus { epic_id, story_id })
            .ok()
            .unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.stories[&story_id].status, Status::InProgress);
    }

    #[test]
    fn handle_action_should_handle_delete_story() {
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let epic_id = db
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .ok()
            .unwrap();
        let story_id = db
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .ok()
            .unwrap();

        let mut nav = Navigator::new(Rc::clone(&db));

        let mut prompts = Prompts::new();
        prompts.delete_story = Box::new(|| true);

        nav.set_prompts(prompts);

        nav.handle_action(Action::DeleteStory { epic_id, story_id })
            .ok()
            .unwrap();

        let db_state = db.read_db().ok().unwrap();
        assert_eq!(db_state.stories.len(), 0);
    }
}
