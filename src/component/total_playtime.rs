use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::total_playtime;
use time::formatter::{Days, Regular, TimeFormatter};
use std::borrow::Cow;
use settings::{Color, Field, Gradient, SettingsDescription, Value};
use super::DEFAULT_INFO_TEXT_GRADIENT;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub show_days: bool,
    pub label_color: Option<Color>,
    pub value_color: Option<Color>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: DEFAULT_INFO_TEXT_GRADIENT,
            show_days: true,
            label_color: None,
            value_color: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub label_color: Option<Color>,
    pub value_color: Option<Color>,
    pub text: String,
    pub time: String,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn name(&self) -> Cow<str> {
        "Total Playtime".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let total_playtime = total_playtime::calculate(timer);

        let time = if self.settings.show_days {
            Days::new().format(total_playtime).to_string()
        } else {
            Regular::new().format(total_playtime).to_string()
        };

        State {
            background: self.settings.background,
            label_color: self.settings.label_color,
            value_color: self.settings.value_color,
            text: String::from("Total Playtime"),
            time,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Show Days (>24h)".into(), self.settings.show_days.into()),
            Field::new("Label Color".into(), self.settings.label_color.into()),
            Field::new("Value Color".into(), self.settings.value_color.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.show_days = value.into(),
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
