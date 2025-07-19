mod sheet; // brings in your spreadsheet logic

use iced::{
    Sandbox, Element, Settings, widget::{Column, Row, Button, Text, TextInput},
};
use sheet::{Sheet, CellValue};

#[derive(Default)]
struct SpreadsheetApp {
    sheet: Sheet,
    input: String,
    selected: Option<(usize, usize)>, // Track selected cell
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    CellSelected(usize, usize),
}

impl Sandbox for SpreadsheetApp {
    type Message = Message;

    fn new() -> Self {
        let mut sheet = Sheet::new();
        sheet.set_cell(0, 0, CellValue::Text("".to_string()));
        Self {
            sheet,
            input: String::new(),
            selected: None, // or Some((0, 0)) if you want a default selection
        }
    }

    fn title(&self) -> String {
        String::from("Rust Spreadsheet UI")
    }

    fn update(&mut self, message: Message) {
    match message {
        Message::InputChanged(val) => {
            self.input = val.clone();
            if let Some((row, col)) = self.selected {
                self.sheet.set_cell(row, col, CellValue::Text(val));
            }
        }
        Message::CellSelected(row, col) => {
            self.selected = Some((row, col));
            let value = self.sheet.get_cell(row, col)
                .map(|c| match &c.value {
                    CellValue::Text(t) => t.clone(),
                    CellValue::Number(n) => n.to_string(),
                    CellValue::Formula(f) => f.clone(),
                    CellValue::Empty => String::new(),
                })
                .unwrap_or_default();
            self.input = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut grid = Column::new().spacing(5);

        for row in 0..5 {
            let mut row_ui = Row::new().spacing(5);
            for col in 0..5 {
                let label = match self.sheet.get_cell(row, col) {
                    Some(cell) => match &cell.value {
                        CellValue::Text(t) => t.clone(),
                        CellValue::Number(n) => n.to_string(),
                        CellValue::Formula(f) => format!("={}", f),
                        CellValue::Empty => String::new(),
                    },
                    None => String::new(),
                };

                let button = Button::new(Text::new(label))
                    .on_press(Message::CellSelected(row, col));

                row_ui = row_ui.push(button);
            }
            grid = grid.push(row_ui);
        }

        Column::new()
            .spacing(10)
            .push(Text::new("Selected Cell Input:"))
            .push(TextInput::new("Type here...", &self.input).on_input(Message::InputChanged))
            .push(grid)
            .into()
    }
}

fn main() -> iced::Result {
    SpreadsheetApp::run(Settings::default())
}