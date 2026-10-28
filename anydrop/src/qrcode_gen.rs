use qrcode::{QrCode, render::unicode};

pub fn qr_from_str(str: &str) -> String {
    let code = QrCode::new(str).unwrap();

    let unicode_str = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    unicode_str
}