use std::collections::HashMap;

#[derive(Clone)]
pub(crate) enum Action {
    View,
    Add,
    Update,
    Delete
}

impl Action {
    pub(crate) fn from(index: &i32) -> Result<Action, String> {
        match Action::values().get(index) {
            Some(action) => Ok(action.clone()),
            None => Err("Invalid action".to_owned())
        }
    }

    fn as_string(&self) -> &'static str {
        match self {
            Action::View => "View Bills",
            Action::Add => "Add New Bill",
            Action::Update => "Update Bill",
            Action::Delete => "Delete Bill",
        }
    }

    fn values() -> HashMap<i32, Action> {
        HashMap::from([
            (1, Action::View),
            (2, Action::Add),
            (3, Action::Update),
            (4, Action::Delete),
        ])
    }

    pub(crate) fn display_values() {
        let values: HashMap<i32, Action> = Self::values();
        for i in 1..=4  {
            println!("{}. {}", i, values.get(&i).unwrap().as_string())
        }
    }
}