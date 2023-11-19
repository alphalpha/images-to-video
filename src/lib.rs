pub mod utils;
use crate::utils::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Codec {
    ProRes,
    H264,
    None,
}

impl Codec {
    fn to_string(&self) -> Result<String, Error> {
        match self {
            Codec::ProRes => Ok("prores".to_owned()),
            Codec::H264 => Ok("libx264".to_owned()),
            Codec::None => Err(Error::Custom("Codec Not Set".to_owned())),
        }
    }

    fn pixel_format(&self) -> Result<String, Error> {
        match self {
            Codec::ProRes => Ok("yuv422p10le".to_owned()),
            Codec::H264 => Ok("yuv420p".to_owned()),
            Codec::None => Err(Error::Custom("Codec Not Set".to_owned())),
        }
    }
}

pub fn build_config(
    ffmpeg_path_str: &str,
    images_path_str: &str,
    frame_rate: u32,
    codec: Codec,
) -> Result<Config, Error> {
    if codec == Codec::None {
        return Err(Error::Custom("Codec Not Set".to_owned()));
    }
    let ffmpeg_path = utils::ffmpeg_path(ffmpeg_path_str)?;
    let (images_path, image_paths) = utils::images_path(images_path_str)?;
    let output_path = images_path.join("_Video.mov");
    Ok(Config {
        ffmpeg_path: ffmpeg_path,
        images_path: image_paths,
        output_path: output_path,
        frame_rate: frame_rate,
        codec: codec,
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    ffmpeg_path: PathBuf,
    images_path: PathBuf,
    output_path: PathBuf,
    frame_rate: u32,
    codec: Codec,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ffmpeg_path: PathBuf::new(),
            images_path: PathBuf::new(),
            output_path: PathBuf::new(),
            frame_rate: 0,
            codec: Codec::None,
        }
    }
}

pub async fn run(config: Config) -> Result<String, Error> {
    let codec = config.codec.to_string()?;
    let pixel_format = config.codec.pixel_format()?;
    match Command::new(config.ffmpeg_path.as_os_str())
        .arg("-y")
        .arg("-pattern_type")
        .arg("glob")
        .arg("-framerate")
        .arg(config.frame_rate.to_string())
        .arg("-i")
        .arg(config.images_path.to_str().unwrap())
        .arg("-c:v")
        .arg(codec)
        .arg("-pix_fmt")
        .arg(pixel_format)
        .arg(config.output_path.to_str().unwrap())
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8(output.stdout).map_err(|e| Error::Utf8(e))
            } else {
                String::from_utf8(output.stderr)
                    .map_err(|e| Error::Utf8(e))
                    .and_then(|message| Ok(message))
            }
        }
        Err(error) => Err(Error::Io(error)),
    }
}
