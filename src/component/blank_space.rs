use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use settings::{Field, Gradient, SettingsDescription, Value};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub height: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Gradient::Transparent,
            height: 24,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub height: u32,
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
        "Blank Space".into()
    }

    pub fn state(&self, _timer: &Timer) -> State {
        State {
            background: self.settings.background,
            height: self.settings.height,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Height".into(), (self.settings.height as u64).into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.height = value.into_uint().unwrap() as _,
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
