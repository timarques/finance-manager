#[derive(Debug, Clone, PartialEq)]
pub enum AlertButtonType {
    Ok,
    Cancel,
    Remove,
}

impl AlertButtonType {
    pub fn as_str(&self) -> &str {
        match self {
            AlertButtonType::Ok => "ok",
            AlertButtonType::Cancel => "cancel",
            AlertButtonType::Remove => "remove",
        }
    }

}

#[derive(Debug, Clone)]
pub struct AlertButton {
    pub label: String,
    pub button_type: AlertButtonType,
    pub appearance: adw::ResponseAppearance,
}

impl AlertButton {

    pub fn new<S: AsRef<str>>(label: S, button_type: AlertButtonType) -> Self {
        Self {
            label: label.as_ref().to_owned(),
            button_type,
            appearance: adw::ResponseAppearance::Default,
        }
    }

    pub fn tag(&self) -> &str {
        self.button_type.as_str()
    }

    pub const fn is_default(&self) -> bool {
        matches!(self.button_type, 
            AlertButtonType::Ok |
            AlertButtonType::Cancel
        )
    }

    pub fn is_destructive(&self) -> bool {
        self.appearance == adw::ResponseAppearance::Destructive
    }

    pub fn is_suggested(&self) -> bool {
        self.appearance == adw::ResponseAppearance::Suggested
    }

    pub fn suggested(mut self) -> Self {
        self.appearance = adw::ResponseAppearance::Suggested;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.appearance = adw::ResponseAppearance::Destructive;
        self
    }

    pub fn ok() -> Self {
        Self::new("OK", AlertButtonType::Ok).suggested()
    }

    pub fn cancel() -> Self {
        Self::new("Cancel", AlertButtonType::Cancel)
    }

    pub fn remove() -> Self {
        Self::new("Remove", AlertButtonType::Remove).destructive()
    }

}

impl Default for AlertButton {
    fn default() -> Self {
        Self::ok()
    }
}