use crate::gif::convert_image_gif;
use crate::webp::convert_image_webp;
use actix_utils::future::{err, ok, Ready};
use actix_web::dev::Payload;
use actix_web::http::header::{ToStrError, ACCEPT};
use actix_web::http::{header, StatusCode};
use actix_web::web::{Bytes};
use actix_web::{post, App, FromRequest, HttpRequest, HttpResponse, HttpServer, ResponseError};
use header::CONTENT_TYPE;
use lottieconv::Animation;

use thiserror::Error;

mod gif;
mod webp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(convert))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

enum ConversionFormat {
    Webp,
    Gif,
}

impl ConversionFormat {
    fn content_type(&self) -> &str {
        match self {
            ConversionFormat::Webp => "image/webp",
            ConversionFormat::Gif => "image/gif",
        }
    }
}

impl TryFrom<&str> for ConversionFormat {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let format = match value {
            "image/webp" => ConversionFormat::Webp,
            "image/gif" => ConversionFormat::Gif,
            _ => return Err(ConversionError::InvalidAcceptHeader),
        };
        Ok(format)
    }
}

impl FromRequest for ConversionFormat {
    type Error = ConversionError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let accept = match req
            .headers()
            .get(ACCEPT)
            .ok_or(ConversionError::AcceptHeaderMissing)
        {
            Ok(value) => value,
            Err(e) => return err(e),
        };
        let accept = match accept.to_str() {
            Ok(header) => header,
            Err(e) => return err(e.into()),
        };
        match accept.try_into() {
            Ok(format) => ok(format),
            Err(e) => err(e),
        }
    }
}

#[post("/convert")]
async fn convert(body: Bytes, format: ConversionFormat) -> Result<HttpResponse, ConversionError> {
    let bytes = body.to_vec();
    let cache_key = sha256::digest_bytes(&bytes);
    let animation = Animation::from_data(bytes, cache_key, "/dev/null")
        .ok_or(ConversionError::AnimationReadFailed)?;
    let image = convert_animation(animation, &format)?;
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, format.content_type()))
        .body(image))
}

fn convert_animation(
    animation: Animation,
    format: &ConversionFormat,
) -> Result<Vec<u8>, ConversionError> {
    match format {
        ConversionFormat::Webp => convert_image_webp(animation),
        ConversionFormat::Gif => convert_image_gif(animation),
    }
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Conversion failed.")]
    ConversionFailed,
    #[error("Unable to read animation.")]
    AnimationReadFailed,
    #[error("Accept Header missing")]
    AcceptHeaderMissing,
    #[error("Accept Header is invalid")]
    InvalidAcceptHeader,
    #[error("Unable to convert Accept header to string")]
    ToStrError(#[from] ToStrError),
}

impl ResponseError for ConversionError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
