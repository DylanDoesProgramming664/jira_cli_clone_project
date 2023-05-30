#![allow(dead_code, unused_imports)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: usize },
    NavigateToStoryDetail { epic_id: usize, story_id: usize },
    NavigateToPreviousPage,
    GetEpicName { epic_id: usize },
    GetEpicDescription { epic_id: usize },
    GetStoryName { story_id: usize },
    GetStoryDescription { story_id: usize },
    CreateEpic,
    CloseEpic { epic_id: usize },
    ReopenEpic { epic_id: usize },
    DeleteEpic { epic_id: usize },
    CreateStory { epic_id: usize },
    UpdateStoryStatus { epic_id: usize, story_id: usize },
    DeleteStory { epic_id: usize, story_id: usize },
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Open => {
                write!(f, "OPEN")
            }
            Self::InProgress => {
                write!(f, "IN PROGRESS")
            }
            Self::Resolved => {
                write!(f, "RESOLVED")
            }
            Self::Closed => {
                write!(f, "CLOSED")
            }
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub stories: Vec<usize>,
    pub status: Status,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        return Epic {
            name,
            description,
            stories: vec![],
            status: Status::Open,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        return Story {
            name,
            description,
            status: Status::Open,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DBState {
    pub last_item_id: usize,
    pub epics: HashMap<usize, Epic>,
    pub stories: HashMap<usize, Story>,
}

impl DBState {
    pub fn new() -> Self {
        return Self {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new(),
        };
    }

    pub fn update_epic_status(&mut self, epic_id: usize) {
        let current_status = self.epics[&epic_id].status.clone();
        let mut closed_count: usize = 0;
        let mut resolved_count: usize = 0;

        if self.epics[&epic_id].stories.is_empty() {
            self.epics.get_mut(&epic_id).unwrap().status = Status::InProgress;
            return;
        }

        for story_id in &self.epics[&epic_id].stories {
            let story = &self.stories[&story_id];
            if story.status == current_status {
                continue;
            }

            match story.status {
                Status::InProgress => {
                    self.epics.get_mut(&epic_id).unwrap().status = Status::InProgress;
                    break;
                }
                Status::Closed => closed_count += 1,
                Status::Resolved => resolved_count += 1,
                _ => continue,
            }
        }

        if (resolved_count + closed_count) as usize == self.epics[&epic_id].stories.len() {
            self.epics.get_mut(&epic_id).unwrap().status = Status::Resolved;
            return;
        }

        if closed_count as usize == self.epics[&epic_id].stories.len() {
            self.epics.get_mut(&epic_id).unwrap().status = Status::Closed;
        }
    }
}
