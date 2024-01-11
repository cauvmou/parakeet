use iced::{Alignment, Application, Command, Element, Length, Renderer, Settings, Theme};
use iced::executor;
use iced::mouse::ScrollDelta::Pixels;
use iced::widget::{button, column, Column, pick_list, PickList, Row, text_input};
use log::{info};

use pcap::Device;

fn main() -> iced::Result {
    simple_logger::init().expect("Failed to initialize logging engine!");
    info!("Starting Application");
    Parakeet::run(Settings::default())
}

struct Parakeet {
    device: Option<Device>,
    active_filter: String,
    filter_text_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    DeviceSelected(Option<Device>),
    FilterChanged(String),
    FilterSubmitted,
}

impl Application for Parakeet {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                device: None,
                active_filter: "".to_string(),
                filter_text_input: "".to_string(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Parakeet Sniffer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::DeviceSelected(device) => {
                self.device = device
            }
            Message::FilterChanged(s) => {self.filter_text_input = s}
            Message::FilterSubmitted => {self.active_filter = self.filter_text_input.clone()}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let adapter_picker: PickList<String, Self::Message> = pick_list(Device::list().expect("Failed to get NICs!").iter().map(|d| d.name.clone()).collect::<Vec<_>>(), self.device.clone().map(|d| d.name), |v| Self::Message::DeviceSelected(Some(Device::from(v.as_str()))))
            .placeholder("Select Adapter".to_string()).width(Length::Fixed(200.0));

        let filter_input = text_input("Filter...", self.filter_text_input.as_str()).on_submit(Self::Message::FilterSubmitted).on_input(|i| Self::Message::FilterChanged(i))
            .width(Length::Fill);

        Row::with_children(vec![adapter_picker.into(), filter_input.into()]).padding(10).spacing(10).align_items(Alignment::Center).width(Length::Fill).into()
    }
}