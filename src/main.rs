use youtube_data::{playlist::Video, parsing::get_videos};

fn main() {
    let videos: Vec<Video> = get_videos("data/eps_page_2.json").unwrap();
    for video in videos {
        println!("{}", Video::filename(&video.title()));
        video.write_markdown_file();
    }
}
