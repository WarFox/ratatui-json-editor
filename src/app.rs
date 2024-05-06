use std::collections::HashMap;

pub enum CurrentScreen {
    Main,    // main summary screen
    Editing, // input key value pairs
    Exiting, // prompt user
}

pub enum CurrentlyEditing {
    // key value pair
    Key,
    Value,
}

// Application State
pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub key: String,
    pub value: String,
    pub pairs: HashMap<String, String>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            key: String::new(),
            value: String::new(),
            pairs: HashMap::new(),
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key.clone(), self.value.clone());
        self.key.clear();
        self.value.clear();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value);
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key);
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let json = serde_json::to_string_pretty(&self.pairs)?;
        println!("{}", json);
        Ok(())
    }
}
