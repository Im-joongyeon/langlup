use std::fs::File;
use std::io::Read;
use tts_rust::{ languages::Languages };
use tts_rust::tts::GTTSClient;
use csv::Reader;
use std::sync::{Arc, Mutex};
use std::thread;

mod make_files;
fn main() {
    let english = read_csv_file_to_string("./assets/english.csv").unwrap();
    let korean = read_csv_file_to_string("./assets/korean.csv").unwrap();
    let filtered_indexes = filter_indexes(&korean);

    // 쓰레드를 저장할 벡터
    let mut handles = vec![];

    // 작업을 8개의 청크로 나누기
    let chunk_size = (filtered_indexes.len() + 7) / 8; 
    for chunk in filtered_indexes.chunks(chunk_size) {
        // 필요한 데이터 클론
        let english_chunk = english.clone();
        let korean_chunk = korean.clone();
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            for &number in &chunk {
                let eng_filepath = format!("./audio/eng_{}.mp3", number);
                let kor_filepath = format!("./audio/kor_{}.mp3", number);

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
                let eng = &english_chunk[number];
                let kor = korean_chunk[number].clone();

                narrator_eng.save_to_file(eng, &eng_filepath).unwrap();
                narrator_kor.save_to_file(&kor, &kor_filepath).unwrap();

                make_files::save_image(&english_chunk[number], &korean_chunk[number], number);
                make_files::make_mp4(number);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    make_files::make_text(&filtered_indexes).unwrap();
    make_files::concat_video(&filtered_indexes).unwrap();
}

fn filter_indexes(word : &Vec<String>) -> Vec<usize> {
    let filtered_indexes: Vec<usize> = word
        .iter()
        .enumerate()
        .filter(|(_, s)| s.len() <= 100)
        .map(|(index, _)| index)
        .collect();
    filtered_indexes
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


    // for word in &result {
    //     println!("senetence : {:?}", word);
    // }
    Ok(result)
}