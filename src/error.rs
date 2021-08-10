/// Possible custom errors for the library
#[derive(Debug)]
pub enum Error {
    /// Error related to libusb
    LibusbError(libusb::Error),
    /// For text printing, the replaced sequence could not be found
    CP437Error(String),
    /// Error regarding image treatment
    ImageError(image::ImageError),
    /// This means no bulk endpoint could be found
    NoBulkEndpoint,
    /// No replacement string for an instruction was found
    NoReplacementFound(String),
    /// PrintData should've been supplied.
    NoPrintData,
    PrinterError(String),
    WrongMarkdown,
    NoTables,
    NoTableFound(String),
    NoWidth,
    NoQrContent(String),
    NoQrContents,
    Encoding
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::LibusbError(e) => format!("Libusb error: {}", e),
            Error::CP437Error(detail) => format!("CP437 error: {}", detail),
            Error::ImageError(e) => format!("Image error: {}", e),
            Error::NoBulkEndpoint => "No bulk endpoint could be found".to_string(),
            Error::NoReplacementFound(replacement) => format!("Could not find replacement for tag {{{}}}", replacement),
            Error::NoPrintData => "Print data must be supplied for this instruction".to_string(),
            Error::PrinterError(detail) => format!("An error occured while printing, {}", detail),
            Error::WrongMarkdown => "Incorrect markdown structure".to_string(),
            Error::NoTables => "Not a single table was found in the PrintData structure".to_string(),
            Error::NoTableFound(table) => format!("No table was found for id {{{}}}", table),
            Error::NoWidth => "No width was found for the selected font".to_string(),
            Error::NoQrContent(name) => format!("Could not find qr code content for \"{}\"", name),
            Error::NoQrContents => "Could not find qr contents".to_string(),
            Error::Encoding => "An unsupported utf-8 character was found when passing to cp437".to_string()
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error{}