use youtube_data::{playlist::Video, parsing::get_videos};

fn main() {
    let videos: Vec<Video> = get_videos("data/eps_page_1.json").unwrap();
    let first_video = &videos[0];
    let markdown = first_video.to_markdown();
    println!("{}", markdown);
}
