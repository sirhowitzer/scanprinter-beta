use iced::widget::{column, row, text, Column, Space, text_input, button};
use iced::{Application, window, Fill};
use polars::prelude::*;
use std::ptr;

mod csv_reader;
mod print_handler;

fn main() -> iced::Result {
    println!("[=====================================]");

    print_handler::save_template();

    iced::application(Scanprinter::new, Scanprinter::update, Scanprinter::view)
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
	input_barcode_content: String,
    product_name_content: String,
    product_price_content: String,
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
    pub fn update(&mut self, message:Message) {
		let dfq = load_data_table();
        let mut product_barcode: i64 = 0;
        let mut product_name: String = "".to_string();
        let mut product_price: String = "".to_string();
		match message {
		    Message::BarcodeInputContentChanged(input_barcode_content) => {
		        self.input_barcode_content = input_barcode_content;				
			}
			Message::BarcodeInputSubmit => {
				product_barcode = self.input_barcode_content.parse().unwrap();

                let result = dfq.expect("CANNOT LOAD DATA TABLE DURING THE BarcodeInputSubmit").lazy()
                    .filter(col("Barcode").eq(lit(product_barcode)))
                    .collect()
                    .expect("INVALID BARCODE!");

                // Extract product_name and product_price as String
                product_name = result.column("Product Name")
                    .expect("Column 'Product Name' not found")
                    .get(0)
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                product_price = result.column("Product Price")
                    .expect("Column 'Price' not found")
                    .get(0)
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                println!("Submitted Barcode: {}", self.input_barcode_content);
                println!("Product Name: {}", product_name);
                println!("Product Price: {}", product_price);
                println!("[=====================================]");

                self.product_name_content = product_name;

                self.product_price_content = product_price;

                self.input_barcode_content = String::new();
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
            	text_input("Type Barcode Here...", &self.input_barcode_content.clone()).width(Fill).on_input(Message::BarcodeInputContentChanged).on_submit(Message::BarcodeInputSubmit),
            	Space::new().width(20),
            ],
            row![
            	Space::new().width(Fill),
            	button("Print").on_press(Message::BarcodeInputSubmit),
            	Space::new().width(Fill)
            ],
            text("Product Name:").size(20).width(Fill).center(),
            text(&self.product_name_content).size(20).width(Fill).center(),

            text("Product Price:").size(20).width(Fill).center(),
            text(&self.product_price_content).size(20).width(Fill).center(),
        ].spacing(10)
    }
}
