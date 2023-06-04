use std::fs::File;
use std::io::Read;
use tts_rust::{ languages::Languages };
use tts_rust::tts::GTTSClient;
use csv::Reader;

mod make_files;
fn main() {
    let english = read_csv_file_to_string("./assets/english.csv").unwrap();
    let korean = read_csv_file_to_string("./assets/korean.csv").unwrap();
    let filtered_indexes = filter_indexes(&korean);

       //todo : 쓰레드 8개 만들어서 작업
       for number in &filtered_indexes {
           let eng_filepath = format!("./audio/eng_{}.mp3",*number);
           let kor_filepath = format!("./audio/kor_{}.mp3",*number);


           let narrator_eng: GTTSClient = GTTSClient {
               volume: 0.9,
               language: Languages::Spanish,
               tld: "com",
           };
           let narrator_kor: GTTSClient = GTTSClient {
               volume: 1.0,
               language: Languages::Korean,
               tld: "com",
           };
           let eng = &english[*number];
           let kor = korean[*number].clone();

           narrator_eng.save_to_file(eng, &eng_filepath).unwrap();

           narrator_kor.save_to_file(&kor, &kor_filepath).unwrap();

           println!("korean len : {:?}", kor.len());

           make_files::save_image(&english[*number], &korean[*number],*number);

           make_files::make_mp4(*number);

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