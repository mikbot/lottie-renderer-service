use crate::{Animation, ConversionError};
use lottieconv::Converter;

pub fn convert_image_webp(animation: Animation) -> Result<Vec<u8>, ConversionError> {
    let webp_data = Converter::new(animation).webp()?.convert()?;
    Ok(webp_data.to_vec())
}

impl From<webp_animation::Error> for ConversionError {
    fn from(_: webp_animation::Error) -> Self {
        ConversionError::ConversionFailed
    }
}
