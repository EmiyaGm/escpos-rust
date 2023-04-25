# escpos-rust: 基于 escpos-rs 优化扩展

## 新增内容
1. 打印一维条形码
2. 打印 GB18030 文本

## 未来新增功能
- [ ] 四行表格打印

# escpos-rs: A Rust crate for thermal printers

**Work in progress. Not ready for production.**

Escpos-rs builds a bit on top of `escpospp`, which aims to bring relatively easy communication to thermal printers that understand the ESC/POS protocol. Here is an example of a simple print with `escpos-rs`

```rust
use escpos_rs::{Printer, PrinterProfile};

fn main() {
    // We create the printer details
    let mut printer_details = PrinterProfile::usb_builder(0x0001, 0x0001).build();
    // We pass it to the printer
    let printer = match Printer::new(printer_details) {
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
```

## Connecting to a printer

In order to connect to a printer, you need to know both the vendor id, and the product id of your printer. Commonly known printers have these details published online, while for the least common you will have at least the following two options

* Some printers do print their information on test prints (often holding the feed button)
* If you are on linux, the `lsusb` command shows you this information.

With this information, you can start a connection to the printer

```rust
// Here goes the vendor id, and the product it (in that order)
let mut printer_details = PrinterProfile::usb_builder(0x0001, 0x0001).build();
// We pass it to the printer
let printer = match Printer::new(printer_details) {
    Ok(maybe_printer) => match maybe_printer {
        Some(printer) => printer,
        None => panic!("No printer was found :(")
    },
    Err(e) => panic!("Error: {}", e)
};
```

## Sending raw information

The printer has the `raw` method, which allows you to send raw bytes to the printer. Pretty straightforward if you need to operate on the low-level.

```rust
use escpos_rs::{
    Printer, PrinterModel,
    command::Command
};

fn main() {
    let printer = match Printer::new(PrinterModel::ZKTeco.usb_profile()) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };
    match printer.raw(b"Hello, world!\n") {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }

    match printer.raw(Command::Cut.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
}
```

You can take a look at the `Command` enum to see which commands are implemented (the list will grow).

## Printing images

You can also send images to the printer (assuming it is supported) through the `EscposImage` structure.

```rust
use escpos_rs::{
    EscposImage, Printer, PrinterProfile, Justification
};

fn main() {
    let printer_profile = PrinterProfile::usb_builder(0x0001, 0x0001).build();
    let printer = match Printer::new(printer_profile) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };
    let img = image::open("logo.jpg").unwrap();
    let escpos_image = EscposImage::new(img, 128, Justification::Center).unwrap();
    match printer.image(escpos_image) {
        Ok(_) => (), // Image should be printed
        Err(e) => println!("Error: {}", e)
    };
}
```

The `EscposImage`'s constructor takes as first argument a `DynamicImage`, as second a width scale (from 0 to 255), and as third, a justification.

## Network functionality

To be added soon.

## The Instruction structure

The Instruction structure has as primary goal the construction of a __template__, which can be used to print multiple documents with dynamic data.

```rust
use escpos_rs::{Printer, PrintData, PrinterProfile, Instruction, Justification, command::Font};

fn main() {
    // Printer profile...
    let printer_profile = PrinterProfile::usb_builder(0x0001, 0x0001)
        .with_font_width(Font::FontA, 32)
        .build();
    // We pass it to the printer
    let printer = match Printer::new(printer_profile) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };
    // We create a simple instruction with a single substitution
    let instruction = Instruction::text(
        "Hello, %name%!",
        Font::FontA,
        Justification::Center,
        // Words that will be replaced in this specific instruction
        Some(vec!["%name%".into()].into_iter().collect())
    );
    // We create custom information for the instruction
    let print_data_1 = PrintData::builder()
        .replacement("%name%", "Carlos")
        .build();
    // And a second set...
    let print_data_2 = PrintData::builder()
        .replacement("%name%", "John")
        .build();
    // We send the instruction to the printer, along with the custom data
    // for this particular print
    match printer.instruction(&instruction, Some(&print_data_1)) {
        Ok(_) => (), // "Hello, Carlos!" should've been printed.
        Err(e) => println!("Error: {}", e)
    }
    // Now we print the second data
    match printer.instruction(&instruction, Some(&print_data_2)) {
        Ok(_) => (), // "Hello, John!" should've been printed.
        Err(e) => println!("Error: {}", e)
    }
}
```

Instructions can be added up to form a complex instruction. Moreover, you can use [serde](https://docs.rs/serde) to serialize and deserialize these instructions so you can save your templates.

## Running the examples

You can run the examples contained in the examples folder, invoking them through cargo.

```
cargo run --example basic
```

# About building this library on Windows

Lib usb is needed for the compilation. Go to https://github.com/libusb/libusb/releases and download the compiled binaries, and put them in your include and bin folders for mingw. You will also need a pkg config file.

* Execute the command `pkg-config.exe --variable pc_path pkg-config` to know where `pkg-config` looks up `pc` files

* Add, in any of those routes, the file `libusb-1.0.pc` with the following content

```pc
prefix=c:/mingw-w64/x86_64-8.1.0-posix-seh-rt_v6-rev0/mingw64
exec_prefix=${prefix}
libdir=${prefix}/lib
includedir=${prefix}/include

Name: libusb-1.0
Description: C API for USB device access from Linux, Mac OS X, Windows, OpenBSD/NetBSD and Solaris userspace
Version: 1.0.23
Libs: -L${libdir} -lusb-1.0
Libs.private: -ludev -pthread
Cflags: -I${includedir}/libusb-1.0
```

**Note**: The version must match your libusb version, and the prefix must also match your main include and lib folders for MinGW.

The following steps are based on [this](https://stackoverflow.com/questions/1710922/how-to-install-pkg-config-in-windows) stackoverflow post.

We assume your mingw installation has its binaries in `C:\MinGW\bin`

* Go to [gnome](http://ftp.gnome.org/pub/gnome/binaries/win32/dependencies/), and download the `pkg-config_0.26-1_win32.zip` package
* Extract the file `bin/pkg-config.exe` to `C:\MinGW\bin`
* Download the file [gettext-runtime_0.18.1.1-2_win32.zip](http://ftp.gnome.org/pub/gnome/binaries/win32/dependencies/gettext-runtime_0.18.1.1-2_win32.zip)
* Extract the file bin/intl.dll to `C:\MinGW\bin`
go to http://ftp.gnome.org/pub/gnome/binaries/win32/glib/2.28
* Download the file `glib_2.28.8-1_win32.zip` from [here](http://ftp.gnome.org/pub/gnome/binaries/win32/glib/2.28) (gnome's website, again).
* Extract the file `bin/libglib-2.0-0.dll` to `C:\MinGW\bin`

# Using the library on Windows

I've only been able to use this library when WinUSB driver is in use for the chosen printer. You can use a tool like [Zadig](https://zadig.akeo.ie/) to change your printer's driver. Just bear in mind that this driver change might make the printer invisible to other tools ;).