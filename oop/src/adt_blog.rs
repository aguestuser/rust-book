pub enum Post {
    Draft(String),
    PendingReview(String, i32),
    Published(String),
}

use self::Post::{Draft, PendingReview, Published};

impl Post {
    pub fn new() -> Post {
        Draft(String::new())
    }

    pub fn add_text(&mut self, text: &str) {
        match self {
            Draft(content) => content.push_str(text),
            _ => (),
        }
    }

    pub fn content(&self) -> Option<&str> {
        match self {
            Published(content) => Some(content),
            _ => None,
        }
    }

    pub fn request_review(self) -> Post {
        match self {
            Draft(content) => PendingReview(content, 0),
            _ => self,
        }
    }

    pub fn approve(self) -> Post {
        match self {
            PendingReview(content, approvals) => {
                if approvals >= 1 {
                    Published(content)
                } else {
                    PendingReview(content, approvals + 1)
                }
            }
            _ => self,
        }
    }

    pub fn reject(self) -> Post {
        match self {
            PendingReview(content, _) => Draft(content),
            _ => self,
        }
    }
}
