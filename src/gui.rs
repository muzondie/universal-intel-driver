use iced::{
    Application, Command, Element, Settings, Theme,
    widget::{button, column, progress_bar, row, text, vertical_space},
};

pub struct App {
    hardware: Vec<crate::hardware::HardwareComponent>,
    progress: f32,
    current_action: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScanPressed,
    InstallPressed,
    UpdateProgress(f32, String),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            hardware: Vec::new(),
            progress: 0.0,
            current_action: String::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Universal Intel Driver")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ScanPressed => Command::perform(
                async { crate::hardware::HardwareScanner::detect_intel_hardware() },
                |hw| Message::UpdateProgress(1.0, format!("Found {} components", hw.len())),
            ),
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Universal Intel Driver").size(24),
            vertical_space(20),
            button("Scan System").on_press(Message::ScanPressed),
            vertical_space(10),
            progress_bar(0.0..=1.0, self.progress),
            text(&self.current_action),
        ].padding(20).into()
    }
}