use std::process::Command;
use std::io::{Result, Write};
use std::fs::File;
use image::{DynamicImage,  Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Scale, Font, point};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;


pub fn concat_video(length :&Vec<usize>) -> Result<()> {
    let file_path = "files.txt";
    let mut ffmpeg_cmd = Command::new("ffmpeg");
    // ffmpeg_cmd
    //     .arg("-hwaccel").arg("cuvid")
    //     .arg("-hwaccel_output_format").arg("cuda");
    // for i in length {
    //     let input_path = format!("./result/{}.mp4", i);
    //     ffmpeg_cmd.arg("-i").arg(input_path);
    // }

    let output = ffmpeg_cmd
    //     .arg("-filter_complex").arg(format!("concat=n={}:v=1:a=1[v][a]",length.len()))
    //     .arg("-map").arg("[v]")
    //     .arg("-map").arg("[a]")
    //     .arg("-c:v").arg("h264_nvenc")
    //     .arg("-c:a").arg("aac")
    //     .arg("-y")
    //     .arg("youtube.mp4")
    //     .output()
    //     .expect("failed..");
        .args(&[
            "-hwaccel", "cuvid",
            "-hwaccel_output_format", "cuda",
            "-safe", "0",
            "-f", "concat",
            "-i", &file_path,
            "-c:v", "av1_nvenc",
            "-c:a", "aac", "-b:a", "192k",
             "-y",
            "youtube.mp4"
        ])
        .output()
        .expect("failed..");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}


pub fn save_image(eng: &str, kor: &str, index : usize) {
    let bg_image = image::open("./assets/wall.png").expect("Failed to open image");

    const GAP : i32 = 150;

    let text_color = Rgba([255, 255, 255, 255]); // 텍스트 색상 (흰색)
    let kor_color = Rgba([0, 0, 0, 0]);

    let font_data: &[u8] = include_bytes!("../fonts/baemin_yensung.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    let eng_font_size_default = 124.0;

    let mut eng_scale = Scale::uniform(eng_font_size_default); // 텍스트 크기 설정
    let  kor_scale = Scale::uniform(96.0); // 텍스트 크기 설정

    let mut eng_glyphs: Vec<_> = font.layout(eng, eng_scale, point(0.0, 0.0)).collect(); // Glyph들을 생성하고, Vec 형태로 수집
    let mut eng_width: f32 = eng_glyphs.last().map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0);

    if eng_width > 1920.0{
        let mut font_scale : f32 = eng_font_size_default;
        while eng_width > 1920.0 {
            font_scale -= 10.0;
            eng_scale = Scale::uniform(font_scale); // 텍스트 크기 설정
            let new_glyphs: Vec<_> = font.layout(eng, eng_scale, point(0.0, 0.0)).collect(); // Glyph들을 생성하고, Vec 형태로 수집
            let new_width: f32 = new_glyphs.last().map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
                .unwrap_or(0.0);

            eng_glyphs = new_glyphs;
            eng_width = new_width;
        }
    }

    let kor_glyphs: Vec<_> = font.layout(kor, kor_scale, point(0.0, 0.0)).collect(); // Glyph들을 생성하고, Vec 형태로 수집
    let kor_width: f32 = kor_glyphs.last().map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0);


    let eng_pos_x = (WIDTH - eng_width as i32 ) / 2;
    let eng_pos_y = HEIGHT / 2 - GAP;
    let kor_pos_x = (WIDTH - kor_width as i32 ) / 2;
    let kor_pos_y = HEIGHT / 2 + GAP;



    while eng_pos_x < 0 {

    }


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


pub fn make_text(index: &Vec<usize>) -> Result<()>{
    let file_path = "files.txt";
    let mut file_content = String::new();

    for num in index {
        let file_name = format!("file './result/{}.mp4'\n", num);
        file_content.push_str(&file_name);
    }
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}

pub fn make_mp4(index: usize) {

    let eng_audio_file_path = format!("./audio/eng_{}.mp3",index);
    let kor_audio_file_path = format!("./audio/kor_{}.mp3",index);
    let img_file_path = format!("./screen/{}.jpg",index);
    let output_file_path = format!("./result/{}.mp4",index);

    let eng_meta = mp3_metadata::read_from_file(&eng_audio_file_path).unwrap();

    let kor_meta = mp3_metadata::read_from_file(&kor_audio_file_path).unwrap();

    let audio  = kor_meta.duration.as_millis() as f64 + eng_meta.duration.as_millis() as f64 + 9000.0;
    let duration = format!("{:.1}", (audio / 1000.0).ceil() as u128);
    println!("index: {}, audio : {:?}, duration : {:?}", index, audio, duration);

    let output = Command::new("ffmpeg")
        .args(&[
            "-hwaccel", "cuvid",
            "-hwaccel_output_format", "cuda",
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
            "-c:v", "av1_nvenc",
            "-c:a", "aac", "-b:a", "192k",
            "-y",
            "-r", "24",
            "-shortest",&output_file_path
        ])
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
