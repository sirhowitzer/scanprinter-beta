use iced::widget::{column, row, text, Column, Space, text_input, button};
use iced::{Application, window, Fill};
use polars::prelude::*;
use std::ptr;

mod csv_reader;

fn main() -> iced::Result {
    println!("[=====================================]");
    iced::application(Scanprinter::new, Scanprinter::update_barcode_input, Scanprinter::view)
        .window_size(iced::Size::new(800.0, 600.0))
        .run()
}

fn load_data_table() -> PolarsResult<DataFrame> {
    let df = csv_reader::read_from_path("assets/nobrandmart-products-list.csv")?;
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
		let dfq = load_data_table();
		match message {
		    Message::BarcodeInputContentChanged(content) => {
		        self.content = content;				
			}
			Message::BarcodeInputSubmit => {
				let product_barcode: i64 = self.content.parse().unwrap();

                let result = dfq.expect("CANNOT LOAD DATA TABLE DURING THE BarcodeInputSubmit").lazy()
                    .filter(col("Barcode").eq(lit(product_barcode)))
                    .collect()
                    .expect("INVALID BARCODE!");

                // Extract product_name and product_price as String
                let product_name = result.column("Product Name")
                    .expect("Column 'Product Name' not found")
                    .get(0)
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let product_price = result.column("Product Price")
                    .expect("Column 'Price' not found")
                    .get(0)
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                println!("Submitted Barcode: {}", self.content);
                println!("Product Name: {}", product_name);
                println!("Product Price: {}", product_price);
                println!("[=====================================]");

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
