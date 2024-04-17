use url::Url;
use worker::*;
use color_thief::*;
use serde::ser::{Serialize, Serializer};
use serde_json::json;

use console_error_panic_hook::set_once as set_panic_hook;


#[derive(Debug)]
struct SerializableRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl From<rgb::RGB<u8>> for SerializableRGB {
    fn from(rgb: rgb::RGB<u8>) -> Self {
        SerializableRGB {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

impl Serialize for SerializableRGB {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{},{},{}", self.r, self.g, self.b);
        serializer.serialize_str(&s)
    }
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
    let image_path = req.path()[1..].to_string();

    set_panic_hook();
    
    match handle_render(image_path).await {
        Err(err) => {
            println!("error: {:?}", err);
            Response::error(format!("an unexpected error occurred: {}", err), 500)
        }
        Ok(res) => Ok(res),
    }
}

async fn handle_render(image_url: String) -> Result<Response> {
    console_log!("image URL: {}", image_url);
    let url = Url::parse(&image_url)
        .map_err(|err| format!("failed to parse URL: {}", err))?;

    let mut res = Fetch::Url(url)
        .send()
        .await
        .map_err(|err| format!("failed to request remote image: {}", err))?;
    if res.status_code() != 200 {
        let body = res.text().await?;
        return Response::error(
            format!("upstream image returned: {}: {}", res.status_code(), body),
            500,
        );
    }
    let image_data = res.bytes().await?;

    // color-thief extract color from image
    let img = image::load_from_memory(&image_data).unwrap();

    let (buffer, color_type) = get_image_buffer(img);
    let colors = color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();
    let response_colors: Vec<SerializableRGB> = colors.into_iter().map(SerializableRGB::from).collect();

    //convert to hex values list
    let hex_colors: Vec<String> = response_colors.iter().map(|color| format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)).collect();
    //return both rgb and hex values in response as rgb: array, hex: array
    let response_data = json!({
        "rgb": response_colors,
        "hex": hex_colors
    });
    let mut headers = Headers::new();
    headers.set("content-type", "application/json").unwrap();
    
    Ok(Response::from_json(&response_data).unwrap().with_headers(headers))
}

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img {
        image::DynamicImage::ImageRgb8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgb)
        }
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}
