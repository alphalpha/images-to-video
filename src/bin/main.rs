use async_std::task;
use images_to_video;

fn main() {
    match images_to_video::build_config(
        "/path/to/ffmpeg",
        "/path/to/images",
        4,
        images_to_video::Codec::H264,
    )
    .and_then(|config| {
        let task = task::spawn(async { images_to_video::run(config).await });
        task::block_on(task)
    }) {
        Ok(message) => println!("Done: {}", message),
        Err(e) => println!("Error: {}", e),
    }
}
