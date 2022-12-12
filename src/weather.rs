use crate::rgba_threshold;

pub async fn check_weather(
    source: String,
    x: u32,
    y: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let img_byte: Vec<u8> = reqwest::get(source)
        .await?
        .bytes()
        .await?
        .into_iter()
        .collect();
    let img = image::load_from_memory(&img_byte)?.into_rgba8();
    Ok(rgba_threshold::rgba8_threshold(
        img.get_pixel(x, y),
        image::Rgba::from([255, 0, 180, 255]),
        255 / 2,
    ))
}

pub fn himawari_se3_format() -> String {
    let now = chrono::Utc::now() - chrono::Duration::minutes(10);
    let minutes = now.minute();
    let minutes_rounded = minutes - (minutes % 10);
    format!(
        "https://www.data.jma.go.jp/mscweb/data/himawari/img/se3/se3_hrp_{:0>2}{:0>2}.jpg",
        now.hour(),
        minutes_rounded
    )
}

use chrono::Timelike;
pub async fn check_latest_weather(x: u32, y: u32) -> Result<bool, Box<dyn std::error::Error>> {
    check_weather(himawari_se3_format(), x, y).await
}
