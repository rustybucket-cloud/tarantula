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
    Add,
    Edit(usize),
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
    ShowAddForm,
    ShowEditForm(usize),
    DeleteApp(usize),

    NameChanged(String),
    UrlChanged(String),
    IconChanged(String),

    SaveAdd,
    SaveEdit,
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
        todo!("Implementation pending")
    }

    pub fn view(&self) -> Column<'_, Message> {
        match self.view_state {
            ViewState::List => self.view_list(),
            ViewState::Add => self.view_add_form(),
            ViewState::Edit(index) => self.view_edit_form(index),
        }
    }

    fn view_list(&self) -> Column<'_, Message> {
        let mut app_widgets = Column::new()
            .spacing(10)
            .padding(20);

        for (index, app) in self.apps.iter().enumerate() {
            let app_row = Container::new(
                Row::new()
                    .push(Text::new(&app.name).size(18))
                    .push(iced::widget::horizontal_space())
                    .push(Button::new(Text::new("Open")).on_press(Message::OpenApp(index)))
                    .spacing(10)
                    .align_y(Alignment::Center)
            )
            .padding(15)
            .width(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
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
            .push(Text::new("Applications").size(24))
            .push(app_widgets)
            .spacing(20)
            .padding(20)
    }

    fn view_add_form(&self) -> Column<'_, Message> {
        todo!("Implementation pending")
    }

    fn view_edit_form(&self, index: usize) -> Column<'_, Message> {
        todo!("Implementation pending")
    }
}
