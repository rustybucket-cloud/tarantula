use crate::app::install;
use crate::app::run;
use crate::app::uninstall;
use crate::app::update;
use crate::app::update::UpdateOptions;
use crate::domain::app::App;
use crate::infra::app_data;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{Alignment, Length};

pub struct Ui {
    apps: Vec<crate::domain::app::App>,
    config: crate::app::config::Config,
    view_state: ViewState,
    form_state: FormState,
}

#[derive(Clone, Debug)]
enum ViewState {
    List,
    Install,
    Update(usize),
}

#[derive(Clone, Debug)]
struct FormState {
    name: String,
    url: String,
    icon: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenApp(usize),
    ShowInstallForm,
    ShowUpdateForm(usize),
    UninstallApp(usize),

    NameChanged(String),
    UrlChanged(String),
    IconChanged(String),

    SaveInstall,
    SaveUpdate,
    CancelForm,

    BackToList,
}

impl Ui {
    pub fn new(config: crate::app::config::Config) -> Self {
        let apps = app_data::get_apps(&config).unwrap();
        Self {
            apps,
            config,
            view_state: ViewState::List,
            form_state: FormState {
                name: String::new(),
                url: String::new(),
                icon: String::new(),
            },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenApp(index) => {
                let app = self.apps.get(index).unwrap();
                run::run(&app.name, &self.config).unwrap();
            }
            Message::ShowInstallForm => {
                self.view_state = ViewState::Install;
                self.form_state = FormState {
                    name: String::new(),
                    url: String::new(),
                    icon: String::new(),
                };
            }
            Message::ShowUpdateForm(index) => {
                if let Some(app) = self.apps.get(index) {
                    self.view_state = ViewState::Update(index);
                    self.form_state = FormState {
                        name: app.name.clone(),
                        url: app.url.clone(),
                        icon: app.icon.clone().unwrap_or_default(),
                    };
                }
            }
            Message::UninstallApp(index) => {
                let app = &self.apps.get(index).unwrap();
                uninstall::uninstall(&app.name, &self.config).unwrap();
                self.apps = self
                    .apps
                    .iter()
                    .filter(|a| a.name != app.name)
                    .cloned()
                    .collect();
            }
            Message::NameChanged(name) => {
                self.form_state.name = name;
            }
            Message::UrlChanged(url) => {
                self.form_state.url = url;
            }
            Message::IconChanged(icon) => {
                self.form_state.icon = icon;
            }
            Message::SaveInstall => {
                install::install(&self.form_state.name, &self.form_state.url, &self.config)
                    .unwrap();
                self.apps.push(App {
                    name: self.form_state.name.clone(),
                    url: self.form_state.url.clone(),
                    icon: None,
                });
                self.view_state = ViewState::List;
            }
            Message::SaveUpdate => {
                let app_name = match &self.view_state {
                    ViewState::Update(index) => self.apps.get(*index).unwrap().name.clone(),
                    _ => panic!("Invalid view state in edit save"),
                };
                let update_options = UpdateOptions {
                    name: Some(self.form_state.name.clone()),
                    url: Some(self.form_state.url.clone()),
                };

                update::update(&app_name, &update_options, &self.config).unwrap();
                for app in &mut self.apps {
                    if app.name == app_name {
                        app.name = self.form_state.name.clone();
                        app.url = self.form_state.url.clone();
                    }
                }
                self.view_state = ViewState::List;
            }
            Message::CancelForm => {
                self.view_state = ViewState::List;
            }
            Message::BackToList => {
                self.view_state = ViewState::List;
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        match self.view_state {
            ViewState::List => self.view_list(),
            ViewState::Install => self.view_add_form(),
            ViewState::Update(_index) => self.view_edit_form(),
        }
    }

    fn view_list(&self) -> Column<'_, Message> {
        let mut app_widgets = Column::new().spacing(10).padding(20);

        for (index, app) in self.apps.iter().enumerate() {
            let app_row = Container::new(
                Row::new()
                    .push(Text::new(&app.name).size(18))
                    .push(iced::widget::horizontal_space())
                    .push(Button::new(Text::new("Update")).on_press(Message::ShowUpdateForm(index)))
                    .push(
                        Button::new(Text::new("Uninstall")).on_press(Message::UninstallApp(index)),
                    )
                    .push(Button::new(Text::new("Open")).on_press(Message::OpenApp(index)))
                    .spacing(10)
                    .align_y(Alignment::Center),
            )
            .padding(15)
            .width(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.95, 0.95, 0.95,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.85, 0.85, 0.85),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            });

            app_widgets = app_widgets.push(app_row);
        }

        Column::new()
            .push(
                Row::new()
                    .push(Text::new("Applications").size(24))
                    .push(iced::widget::horizontal_space())
                    .push(
                        Button::new(Text::new("+ Install App")).on_press(Message::ShowInstallForm),
                    )
                    .align_y(Alignment::Center),
            )
            .push(app_widgets)
            .spacing(20)
            .padding(20)
    }

    fn view_add_form(&self) -> Column<'_, Message> {
        Column::new()
            .push(Text::new("Install New Application").size(24))
            .push(Text::new("Name:"))
            .push(
                TextInput::new("App name", &self.form_state.name)
                    .on_input(Message::NameChanged)
                    .padding(10),
            )
            .push(Text::new("URL:"))
            .push(
                TextInput::new("App URL", &self.form_state.url)
                    .on_input(Message::UrlChanged)
                    .padding(10),
            )
            .push(Text::new("Icon:"))
            .push(
                TextInput::new("Icon path", &self.form_state.icon)
                    .on_input(Message::IconChanged)
                    .padding(10),
            )
            .push(
                Row::new()
                    .push(Button::new(Text::new("Install")).on_press(Message::SaveInstall))
                    .push(Button::new(Text::new("Cancel")).on_press(Message::BackToList))
                    .spacing(10),
            )
            .spacing(15)
            .padding(20)
    }

    fn view_edit_form(&self) -> Column<'_, Message> {
        Column::new()
            .push(Text::new("Update Application").size(24))
            .push(Text::new("Name:"))
            .push(
                TextInput::new("App name", &self.form_state.name)
                    .on_input(Message::NameChanged)
                    .padding(10),
            )
            .push(Text::new("URL:"))
            .push(
                TextInput::new("App URL", &self.form_state.url)
                    .on_input(Message::UrlChanged)
                    .padding(10),
            )
            .push(Text::new("Icon:"))
            .push(
                TextInput::new("Icon path", &self.form_state.icon)
                    .on_input(Message::IconChanged)
                    .padding(10),
            )
            .push(
                Row::new()
                    .push(Button::new(Text::new("Save")).on_press(Message::SaveUpdate))
                    .push(Button::new(Text::new("Cancel")).on_press(Message::BackToList))
                    .spacing(10),
            )
            .spacing(15)
            .padding(20)
    }
}
