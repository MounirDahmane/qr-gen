#[derive(PartialEq, Clone)]
pub enum SaveFormat {
    PNG,
    SVG,
}

#[derive(Debug, PartialEq)]
pub enum ModuleShape {
    SQUARE,
    ROUNDED,
}
