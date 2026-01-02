use iced::widget::{column, row, text, Column, Space, text_input, button};
use iced::{Application, window, Fill};
use polars::prelude::*;
use std::ptr;

mod csv_reader;

fn main() -> iced::Result {

    iced::application(Scanprinter::new, Scanprinter::update_barcode_input, Scanprinter::view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run()
}

fn load_data() -> PolarsResult<DataFrame> {
    let df = csv_reader::read_from_path("assets/doktorfoodmart-products-list.csv")?;
    Ok(df)
}

//debug function
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(Default)]
struct Scanprinter {
	content: String,
	df: DataFrame,
}

#[derive(Debug, Clone)]
pub enum Message {
	BarcodeInputContentChanged(String),
	BarcodeInputSubmit,
}

impl Scanprinter {
    fn new() -> Self {
        Scanprinter::default()
    }
    pub fn update_barcode_input(&mut self, message:Message) {
		let dfq = load_data();
		match message {
		    Message::BarcodeInputContentChanged(content) => {
		        self.content = content;				
			}
			Message::BarcodeInputSubmit => {
				println!("Submitted Barcode: {}", self.content);

				let barcode: i64 = self.content.parse().unwrap();
				//let barcode: i64 = 1231023;
				//println!("The type of x is: {}", type_of(&self.content));

				let filtered = dfq.expect("Expecto Patronum").lazy()
					.filter(col("Barcode").eq(lit(barcode)))
					.collect();
				
				
				println!("{:?}", filtered);

				self.content = String::new();
			}
		}	
    }
    pub fn view(&self) -> Column<Message> {
        column![
            text("Scanprinter V1.0").size(50).width(Fill).center(),
			row![
				Space::new().width(20),
				text("Insert Barcode:").size(18),
			],
            row![
            	Space::new().width(20),
            	text_input("Type Barcode Here...", &self.content.clone()).width(Fill).on_input(Message::BarcodeInputContentChanged).on_submit(Message::BarcodeInputSubmit),
            	Space::new().width(20),
            ],
            row![
            	Space::new().width(Fill),
            	button("Print").on_press(Message::BarcodeInputSubmit),
            	Space::new().width(Fill)
            ]
        ].spacing(10)
    }
}
