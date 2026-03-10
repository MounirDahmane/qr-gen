/// Export format selected by the user in the settings panel.
#[derive(PartialEq, Clone)]
pub enum SaveFormat {
    PNG,
    SVG,
}

/// Visual style of each QR module (individual square unit of the QR code).
/// Affects only rendering — does not change scannability.
#[derive(Debug, PartialEq)]
pub enum ModuleShape {
    SQUARE,
    ROUNDED,
}
