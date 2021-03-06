use {component, Timer};
use super::{Component, GeneralSettings, LayoutSettings, LayoutState};

#[derive(Clone)]
pub struct Layout {
    pub components: Vec<Component>,
    settings: GeneralSettings,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            settings: GeneralSettings::default(),
        }
    }

    pub fn default_layout() -> Self {
        Self {
            components: vec![
                component::title::Component::new().into(),
                component::splits::Component::new().into(),
                component::timer::Component::new().into(),
                component::previous_segment::Component::new().into(),
            ],
            settings: GeneralSettings::default(),
        }
    }

    pub fn from_settings(layout_settings: LayoutSettings) -> Self {
        Self {
            components: layout_settings
                .components
                .into_iter()
                .map(Into::into)
                .collect(),
            settings: layout_settings.general,
        }
    }

    pub fn general_settings(&self) -> &GeneralSettings {
        &self.settings
    }

    pub fn general_settings_mut(&mut self) -> &mut GeneralSettings {
        &mut self.settings
    }

    pub fn push<C: Into<Component>>(&mut self, component: C) {
        self.components.push(component.into());
    }

    pub fn state(&mut self, timer: &Timer) -> LayoutState {
        let settings = &self.settings;
        LayoutState {
            components: self.components
                .iter_mut()
                .map(|c| c.state(timer, settings))
                .collect(),
            background: self.settings.background,
            thin_separators_color: self.settings.thin_separators_color,
            separators_color: self.settings.separators_color,
            text_color: self.settings.text_color,
        }
    }

    pub fn settings(&self) -> LayoutSettings {
        LayoutSettings {
            components: self.components.iter().map(|c| c.settings()).collect(),
            general: self.settings.clone(),
        }
    }

    pub fn scroll_up(&mut self) {
        for component in &mut self.components {
            component.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        for component in &mut self.components {
            component.scroll_down();
        }
    }

    pub fn remount(&mut self) {
        for component in &mut self.components {
            component.remount();
        }
    }
}
