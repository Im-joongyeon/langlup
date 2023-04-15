use std::fmt::format;
use tts_rust::{ languages::Languages };
use tts_rust::tts::GTTSClient;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Scale, Font, point};

use std::process::Command;
use std::io::Result;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use mp3_metadata::read_from_file;

const width: i32 = 1920;
const height: i32 = 1080;

fn main() {
    let narrator_eng: GTTSClient = GTTSClient {
        volume: 0.9,
        language: Languages::English,
        tld: "com",
    };
    let narrator_kor: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::Korean,
        tld: "com",
    };
    let english = vec!["Hello".to_string(), "World".to_string(), "video".to_string()];
    let korean = vec!["안녕하세요 좋은아침입니다.".to_string(), "세상".to_string(), "비디오".to_string()];
    let length = english.len();



    let mut index = 0;
    for number in 0..length {
        let eng_filepath = format!("./audio/eng_{}.mp3",index);
        narrator_eng.save_to_file(&english[number], &eng_filepath).unwrap();
        let kor_filepath = format!("./audio/kor_{}.mp3",index);
        narrator_kor.save_to_file(&korean[number], &kor_filepath).unwrap();

        save_image(&english[number], &korean[number],index);

        make_MP4(index);

        index += 1;
    }

}

fn save_image(eng: &str, kor: &str, index: i32) {
    let bg_image = image::open("./assets/wall.png").expect("Failed to open image");

    const gap : i32 = 100;

    let text_color = Rgba([255, 255, 255, 255]); // 텍스트 색상 (흰색)
    let kor_color = Rgba([0, 0, 0, 0]);

    let font_data: &[u8] = include_bytes!("../fonts/baemin_yensung.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let eng_scale = Scale::uniform(200.0); // 텍스트 크기 설정
    let kor_scale = Scale::uniform(96.0); // 텍스트 크기 설정


    let eng_glyphs: Vec<_> = font.layout(eng, eng_scale, point(0.0, 0.0)).collect(); // Glyph들을 생성하고, Vec 형태로 수집
    let eng_width: f32 = eng_glyphs.last().map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0);

    let kor_glyphs: Vec<_> = font.layout(kor, kor_scale, point(0.0, 0.0)).collect(); // Glyph들을 생성하고, Vec 형태로 수집
    let kor_width: f32 = kor_glyphs.last().map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0);


    let eng_pos_x = (width - eng_width as i32 ) / 2;
    let eng_pos_y = height / 2 - gap;
    let kor_pos_x = (width - kor_width as i32 ) / 2;
    let kor_pos_y = height / 2 + gap;

    let mut image  = DynamicImage::ImageRgba8(bg_image.to_rgba8());


    draw_text_mut(
        &mut image,
        text_color,
        eng_pos_x,
        eng_pos_y,
        eng_scale,
        &font,
        eng,
    );


    draw_text_mut(
        &mut image,
        kor_color,
        kor_pos_x,
        kor_pos_y,
        kor_scale,
        &font,
        kor,
    );


    // 이미지 저장
    let filepath = format!("./screen/{}.jpg",index);
    image.save(filepath).expect("Failed to save image");
    // image.as_bytes()
}

fn make_MP4(index: i32) {

    let eng_audio_file_path = format!("./audio/eng_{}.mp3",index);
    let kor_audio_file_path = format!("./audio/kor_{}.mp3",index);
    let img_file_path = format!("./screen/{}.jpg",index);
    let output_file_path = format!("./result/{}.mp4",index);

    let eng_meta = mp3_metadata::read_from_file(&eng_audio_file_path).unwrap();

    let kor_meta = mp3_metadata::read_from_file(&kor_audio_file_path).unwrap();

    let audio  = kor_meta.duration.as_millis() as f64 + eng_meta.duration.as_millis() as f64 + 9000.0;
    let duration = format!("{:.1}", (audio / 1000.0).ceil() as u128);
    println!("audio : {:?}, duration : {:?}", audio, duration);

    let output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-r", "24",
            "-loop", "1", "-i", &img_file_path,
            "-i", &kor_audio_file_path,
            "-f", "lavfi", "-t", "3", "-i", "anullsrc",
            "-i", &eng_audio_file_path,
            "-f", "lavfi", "-t", "3", "-i", "anullsrc",
            "-i", &eng_audio_file_path,
            "-f", "lavfi", "-t", "3", "-i", "anullsrc",
            "-i", &eng_audio_file_path,
            "-f", "lavfi", "-t", "3", "-i", "anullsrc",
            "-filter_complex", "[1:a][2:a][3:a][4:a][5:a][6:a][7:a]concat=n=8:v=0:a=1[a]",
            "-map", "0:v",
            "-map", "[a]",
            "-c:v", "libx264", "-tune", "stillimage",
            "-c:a", "aac", "-b:a", "192k",
            "-pix_fmt", "yuv420p",
            "-t", &duration.to_string(),
            "-shortest",&output_file_path
        ])
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}


