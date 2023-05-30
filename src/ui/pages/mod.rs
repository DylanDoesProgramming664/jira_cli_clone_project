#![allow(unused_imports, dead_code)]
use std::any::Any;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        // TODO: print out epics using get_column_string(). also make sure the epics are sorted by id
        let db = self.db.read_db()?;
        let epics = db.epics;
        for id in epics.keys().sorted() {
            let epic = &epics[id];
            println!(
                "{} | {} | {}",
                get_column_string(&id.to_string(), 11),
                get_column_string(&epic.name, 32),
                get_column_string(&epic.description, 17)
            )
        }

        println!();
        println!();

        println!("[q]uit | [c]reate epic | epic [:id:]");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epics = self.db.read_db()?.epics;
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        return match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            x => match x.parse::<usize>() {
                Ok(id) => {
                    if let None = epics.get(&id) {
                        Ok(None)
                    } else {
                        Ok(Some(Action::NavigateToEpicDetail { epic_id: (id) }))
                    }
                }
                Err(_) => Ok(None),
            },
        };
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

pub struct EpicDetail {
    pub epic_id: usize,
    pub db: Rc<JiraDatabase>,
}

impl EpicDetail {
    pub fn new(epic_id: usize, db: Rc<JiraDatabase>) -> Self {
        return Self { epic_id, db };
    }
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        // TODO: print out epic details using get_column_string()
        println!(
            "{:<5} | {} | {} | {}",
            &self.epic_id.to_string(),
            get_column_string(&epic.name, 12),
            get_column_string(&epic.description, 27),
            get_column_string(&epic.status.to_string(), 13)
        );

        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        // TODO: print out stories using get_column_string(). also make sure the stories are sorted by id
        let stories = &db_state.stories;
        for story_id in &epic.stories {
            println!(
                "{} | {} | {}",
                get_column_string(&story_id.to_string(), 11),
                get_column_string(&stories[&story_id].name, 32),
                get_column_string(&stories[&story_id].status.to_string(), 17)
            );
        }

        println!();
        println!();

        println!("[p]revious | [cl]ose epic | [r]eopen epic | [d]elete epic | [cr]eate story | [e]pic [n]ame | [e]pic [d]escription | story [:id:]");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epic_id = self.epic_id;
        let stories = &self.db.read_db()?.epics[&epic_id].stories;

        // match against the user input and return the corresponding action. If the user input was invalid return None.
        return match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "cl" => Ok(Some(Action::CloseEpic {
                epic_id: (self.epic_id),
            })),
            "r" => Ok(Some(Action::ReopenEpic {
                epic_id: self.epic_id,
            })),
            "d" => Ok(Some(Action::DeleteEpic {
                epic_id: (self.epic_id),
            })),
            "cr" => Ok(Some(Action::CreateStory {
                epic_id: (self.epic_id),
            })),
            "en" => Ok(Some(Action::GetEpicName {
                epic_id: self.epic_id,
            })),
            "ed" => Ok(Some(Action::GetEpicDescription {
                epic_id: self.epic_id,
            })),
            input => match input.parse::<usize>() {
                Ok(id) => match stories.contains(&id) {
                    true => Ok(Some(Action::NavigateToStoryDetail {
                        epic_id,
                        story_id: id,
                    })),
                    false => Ok(None),
                },
                Err(_) => Ok(None),
            },
        };
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

pub struct StoryDetail {
    pub epic_id: usize,
    pub story_id: usize,
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        // TODO: print out story details using get_column_string()
        println!(
            "{} | {} | {} | {}",
            get_column_string(&self.story_id.to_string(), 5),
            get_column_string(&story.name, 12),
            get_column_string(&story.description, 27),
            get_column_string(&story.status.to_string(), 13)
        );

        println!();
        println!();

        println!(
            "[p]revious | [u]pdate story | [s]tory [n]ame | [s]tory [d]escription | [d]elete story"
        );

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        return Ok(match input {
            "p" => Some(Action::NavigateToPreviousPage),
            "u" => Some(Action::UpdateStoryStatus {
                epic_id: self.epic_id,
                story_id: self.story_id,
            }),
            "sn" => Some(Action::GetStoryName {
                story_id: self.story_id,
            }),
            "sd" => Some(Action::GetStoryDescription {
                story_id: self.story_id,
            }),
            "d" => Some(Action::DeleteStory {
                epic_id: self.epic_id,
                story_id: self.story_id,
            }),
            _ => None,
        });
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_utils::MockDB;
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .ok()
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "cl";
            let d = "d";
            let c = "cr";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::CloseEpic { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteEpic { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(c).unwrap(),
                Some(Action::CreateStory { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail {
                    epic_id: 1,
                    story_id: 2
                })
            );
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod story_detail_page {
        use std::borrow::BorrowMut;

        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
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

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = (&db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap())
                .to_owned();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let _ = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id: 999,
                db,
            };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateStoryStatus { epic_id, story_id })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteStory { epic_id, story_id })
            );
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }
}
