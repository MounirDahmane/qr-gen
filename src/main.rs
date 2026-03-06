use image::Luma;
use qrcode::QrCode;

fn generate(data: &str) {
    // Encode some data into bits.
    let code = QrCode::new(data.as_bytes()).unwrap();

    // Render the bits into an image.
    let img = code.render::<Luma<u8>>().build();

    // Save the image.
    img.save("./qrcode.png").unwrap();
}


fn main(){
    generate("https://github.com/MounirDahmane");
}