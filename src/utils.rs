use std::path::{Path, PathBuf};
use std::{fmt, fs, io, num, string};

pub fn ffmpeg_path(path_str: &str) -> Result<PathBuf, Error> {
    let ffmpeg_path = PathBuf::from(path_str);
    match ffmpeg_path.try_exists() {
        Ok(true) => {
            if let Some(name) = ffmpeg_path.file_name() {
                if name == "ffmpeg" {
                    Ok(ffmpeg_path)
                } else {
                    Err(Error::Custom("File Name is not \"ffmpeg\"".to_owned()))
                }
            } else {
                Err(Error::Custom(format!("{} does not exist", path_str)))
            }
        }
        Ok(false) => Err(Error::Custom(format!("{} does not exist", path_str))),
        Err(e) => Err(Error::Io(e)),
    }
}

pub fn images_path(path_str: &str) -> Result<(PathBuf, PathBuf), Error> {
    let images_path = PathBuf::from(path_str);
    if images_path.is_dir() {
        let paths = image_paths(&images_path)?;
        if paths.is_empty() {
            Err(Error::Custom("Empty Images Folder".to_owned()))
        } else {
            let extension = paths
                .get(0)
                .and_then(|file| file.extension())
                .ok_or(Error::Custom(
                    "Cannot Obtain Extension of Image File".to_owned(),
                ))?;
            let wildcard = images_path.join("*");
            Ok((images_path, wildcard.with_extension(extension)))
        }
    } else {
        Err(Error::Custom(format!("{} must be a directory", path_str)))
    }
}

fn image_paths(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some())
        .collect();
    paths.sort();
    Ok(paths)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    Custom(String),
    Utf8(string::FromUtf8Error),
    Else,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO Error: {}", err),
            Error::ParseFloat(ref err) => write!(f, "Parse Error: {}", err),
            Error::ParseInt(ref err) => write!(f, "Parse Error: {}", err),
            Error::Custom(ref err) => write!(f, "Error: {}", err),
            Error::Utf8(ref err) => write!(f, "Error: {}", err),
            Error::Else => write!(f, "Some Error"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Custom(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}
