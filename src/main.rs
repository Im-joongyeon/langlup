use std::fmt::format;
use std::fs::File;
use std::io::Read;
use tts_rust::{ languages::Languages };
use tts_rust::tts::GTTSClient;
use csv::Reader;
use std::{error::Error, io, process};

mod make_files;
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


    let english = read_csv_file_to_string("./assets/english.csv").unwrap();
    let korean = read_csv_file_to_string("./assets/korean.csv").unwrap();
    let length = english.len();

    //todo : 쓰레드 8개 만들어서 작업
    for number in 0..length {
        let eng_filepath = format!("./audio/eng_{}.mp3",number);
        narrator_eng.save_to_file(&english[number], &eng_filepath).unwrap();
        let kor_filepath = format!("./audio/kor_{}.mp3",number);
        narrator_kor.save_to_file(&korean[number], &kor_filepath).unwrap();

        make_files::save_image(&english[number], &korean[number],number);

        make_files::make_mp4(number);

    }
   make_files::concat_video(length).unwrap();
}



fn read_csv_file_to_string(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    // 파일 열기
    let mut file = File::open(file_path)?;

    // 파일 내용을 읽어서 문자열로 저장
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // CSV 데이터 파싱
    let mut reader = Reader::from_reader(contents.as_bytes());
    let mut result = Vec::new();
    for record in reader.records() {
        if let Ok(record) = record {
            if let Some(field) = record.get(0) {
                result.push(field.to_string());
            }
        }
    }


    for word in &result {
        println!("senetence : {:?}", word);
    }
    Ok(result)
}