use escpos_rust::{Printer, PrinterProfile, command::Font};

fn main() {
    let printer_profile = PrinterProfile::terminal_builder().with_font_width(Font::FontA, 20).build();
    // We pass it to the printer
    let printer = match Printer::new(printer_profile) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };
    
    // We print simple text
    match printer.println("Hello, world!") {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
}