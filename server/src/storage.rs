use crate::schema;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound,
    // Internal(String),
}

pub trait Storage {
    fn save(&mut self, session: schema::Session) -> Result<(), &'static str>;
    fn find(&self, session_id: &String) -> Result<&schema::Session, Error>;
}

mod memory {
    use crate::schema;
    use crate::storage::{Storage, Error};
    use std::collections::HashMap;

    pub struct MemoryStorage {
        session_map: HashMap<String, schema::Session>,
    }

    impl MemoryStorage {
        pub fn new() -> Self {
            MemoryStorage {
                session_map: HashMap::new()
            }
        }
    }

    impl Storage for MemoryStorage {
        fn save(&mut self, session: schema::Session) -> Result<(), &'static str> {
            if session.id.len() > 0 {
                self.session_map.insert(session.id.clone(), session);
                Result::Ok(())
            } else {
                Result::Err("id can not be empty")
            }
        }

        fn find(&self, session_id: &String) -> Result<&schema::Session, Error> {
            let session = self.session_map.get(session_id);
            if session == None {
                return Result::Err(Error::NotFound);
            } else {
                return Result::Ok(session.unwrap());
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::MemoryStorage;
        use crate::schema;
        use crate::storage::{Storage, Error};

        #[test]
        fn save_returns_ok() {
            let mut storage = MemoryStorage::new();
            let session = schema::Session{
                title: "my test session".to_string(),
                id: "id1".to_string(),
                cards: vec!["1".to_string(), "2".to_string()]
            };
            let result = storage.save(session);
            assert_eq!(result, Result::Ok(()))
        }

        #[test]
        fn save_returns_error_when_id_is_empty() {
            let mut storage = MemoryStorage::new();
            let session = schema::Session{
                title: "empty id session".to_string(),
                id: "".to_string(),
                cards: vec!["1".to_string(), "2".to_string()]
            };
            let result = storage.save(session);
            assert_eq!(result, Result::Err("id can not be empty"))
        }

        #[test]
        fn find_returns_not_found() {
            let storage = MemoryStorage::new();
            let result = storage.find(&String::from("not-existing"));
            assert_eq!(result, Result::Err(Error::NotFound));
        }

        #[test]
        fn find_returns_session() {
            let mut storage = MemoryStorage::new();
            let session = schema::Session{
                title: "my test session".to_string(),
                id: "id1".to_string(),
                cards: vec!["1".to_string(), "2".to_string()]
            };
            let result = storage.save(session);
            assert_eq!(result, Result::Ok(()));

            let session = storage.find(&"id1".to_string());
            assert_eq!(session, Result::Ok(&schema::Session {
                title: "my test session".to_string(),
                id: "id1".to_string(),
                cards: vec!["1".to_string(), "2".to_string()]
            }));
        }
    }
}
