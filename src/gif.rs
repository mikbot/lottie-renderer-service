use crate::{Animation, ConversionError};
use lottieconv::{Converter, Rgba};
use std::io::Cursor;

pub fn convert_image_gif(animation: Animation) -> Result<Vec<u8>, ConversionError> {
    let mut buffer = Cursor::new(Vec::new());
    Converter::new(animation)
        .gif(Rgba::new_alpha(0, 0, 0, true), &mut buffer)?
        .convert()?;
    Ok(buffer.into_inner())
}

impl From<gif::EncodingError> for ConversionError {
    fn from(_: gif::EncodingError) -> Self {
        ConversionError::ConversionFailed
    }
}
