use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::string::String;
use clap::{Command, arg, ArgGroup, value_parser};

/// 7 bytes for the ASCII string "CAMELOT" in each save's header.
const HEADER_CAMELOT_ASCII_STRING: &str = "CAMELOT";

/// Golden Sun/Golden Sun: The Lost Age build date
/// Source: Golden Sun Hacking Community Discord Server
///
/// Value: 0x1000 | 1024 * year + 64 * month + day
/// Note: Year is "1", "2", not "2001", "2002"
///
/// GS1 (J) = 0x159C -> 2001/06/28
/// GS1 (U) = 0x1652 -> 2001/09/18
/// GS1 (G) = 0x1849 -> 2002/01/09
/// GS1 (S) = 0x1885 -> 2002/02/05
/// GS1 (F) = 0x1713 -> 2001/12/19
/// GS1 (I) = 0x1886 -> 2002/02/06
///
/// GS2 (J) = 0x198A -> 2002/06/10
/// GS2 (U) = 0x1C85 -> 2003/02/05
/// GS2 (G) = 0x1D97 -> 2003/06/23
/// GS2 (S) = 0x1DC7 -> 2003/07/07
/// GS2 (F) = 0x1D98 -> 2003/06/24
/// GS2 (I) = 0x1DC8 -> 2003/07/08
const GS_BUILD_DATE: [[u16; 6]; 2] = [
  [0x159C, 0x1652, 0x1849, 0x1885, 0x1713, 0x1886],
  [0x198A, 0x1C85, 0x1D97, 0x1DC7, 0x1D98, 0x1DC8]
];

/// Main characters' default names in different languages.
///
/// Japanese:                                   "ロビン", "ジェラルド", "イワン", "メアリィ", "ガルシア", "ジャスミン", "シバ",  "ピカード"
/// English:                                    "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Sheba", "Piers"
/// German:                                     "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Cosma", "Aaron"
/// Spanish:                                    "Hans",   "Garet",      "Iván",   "Mia",      "Félix",    "Nadia",      "Sole",  "Piers"
/// French:                                     "Vlad",   "Garet",      "Ivan",   "Sofia",    "Pavel",    "Lina",       "Cylia", "Piers"
/// Italian:                                    "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Sara",  "Piers"
/// Chinese fan translation by Mobile/Soma Team for GS2 (SC and TC share same encoding)
/// ├── Simplified Chinese:                     "罗宾",   "杰拉德",     "伊万",   "米雅莉",   "加西亚",   "加斯敏",     "西芭",  "皮卡德"
/// └── Traditional Chinese:                    "羅賓",   "傑拉德",     "伊萬",   "米雅莉",   "加西亞",   "加斯敏",     "西芭",  "皮卡德"
/// Chinese fan translation by 2023 Team for GS1
/// └── Simplified Chinese:                     "罗宾",   "杰拉德",     "伊万",   "梅雅莉",   "加西亚",   "加斯敏",     "西芭",  "皮卡德"
///
/// Note: In the Korean fan translation version by pjs0493, the main characters' names are used from the Japanese version.
///       Translation patch source: [TBS](https://blog.naver.com/pjs0493/220449867529) and [TLA](https://blog.naver.com/pjs0493/220489958082)
///       In the Polish fan translation version by Rykuzushi, the main characters' names are used from the English version.
///       Translation patch source: [TBS](https://www.romhacking.net/translations/1078/)
const PC_NAME: [[[u8; 7]; 8]; 8] = [
  [[0xDB, 0xCB, 0xDE, 0xDD, 0x00, 0x00, 0x00], [0xBC, 0xDE, 0xAA, 0xD7, 0xD9, 0xC4, 0xDE], [0xB2, 0xDC, 0xDD, 0x00, 0x00, 0x00, 0x00], [0xD2, 0xB1, 0xD8, 0xA8, 0x00, 0x00, 0x00], [0xB6, 0xDE, 0xD9, 0xBC, 0xB1, 0x00, 0x00], [0xBC, 0xDE, 0xAC, 0xBD, 0xD0, 0xDD, 0x00], [0xBC, 0xCA, 0xDE, 0x00, 0x00, 0x00, 0x00], [0xCB, 0xDF, 0xB6, 0xB0, 0xC4, 0xDE, 0x00]],
  [[0x49, 0x73, 0x61, 0x61, 0x63, 0x00, 0x00], [0x47, 0x61, 0x72, 0x65, 0x74, 0x00, 0x00], [0x49, 0x76, 0x61, 0x6E, 0x00, 0x00, 0x00], [0x4D, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00], [0x46, 0x65, 0x6C, 0x69, 0x78, 0x00, 0x00], [0x4A, 0x65, 0x6E, 0x6E, 0x61, 0x00, 0x00], [0x53, 0x68, 0x65, 0x62, 0x61, 0x00, 0x00], [0x50, 0x69, 0x65, 0x72, 0x73, 0x00, 0x00]],
  [[0x49, 0x73, 0x61, 0x61, 0x63, 0x00, 0x00], [0x47, 0x61, 0x72, 0x65, 0x74, 0x00, 0x00], [0x49, 0x76, 0x61, 0x6E, 0x00, 0x00, 0x00], [0x4D, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00], [0x46, 0x65, 0x6C, 0x69, 0x78, 0x00, 0x00], [0x4A, 0x65, 0x6E, 0x6E, 0x61, 0x00, 0x00], [0x43, 0x6F, 0x73, 0x6D, 0x61, 0x00, 0x00], [0x41, 0x61, 0x72, 0x6F, 0x6E, 0x00, 0x00]],
  [[0x48, 0x61, 0x6E, 0x73, 0x00, 0x00, 0x00], [0x47, 0x61, 0x72, 0x65, 0x74, 0x00, 0x00], [0x49, 0x76, 0xE1, 0x6E, 0x00, 0x00, 0x00], [0x4D, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00], [0x46, 0xE9, 0x6C, 0x69, 0x78, 0x00, 0x00], [0x4E, 0x61, 0x64, 0x69, 0x61, 0x00, 0x00], [0x53, 0x6F, 0x6C, 0x65, 0x00, 0x00, 0x00], [0x50, 0x69, 0x65, 0x72, 0x73, 0x00, 0x00]],
  [[0x56, 0x6C, 0x61, 0x64, 0x00, 0x00, 0x00], [0x47, 0x61, 0x72, 0x65, 0x74, 0x00, 0x00], [0x49, 0x76, 0x61, 0x6E, 0x00, 0x00, 0x00], [0x53, 0x6F, 0x66, 0x69, 0x61, 0x00, 0x00], [0x50, 0x61, 0x76, 0x65, 0x6C, 0x00, 0x00], [0x4C, 0x69, 0x6E, 0x61, 0x00, 0x00, 0x00], [0x43, 0x79, 0x6C, 0x69, 0x61, 0x00, 0x00], [0x50, 0x69, 0x65, 0x72, 0x73, 0x00, 0x00]],
  [[0x49, 0x73, 0x61, 0x61, 0x63, 0x00, 0x00], [0x47, 0x61, 0x72, 0x65, 0x74, 0x00, 0x00], [0x49, 0x76, 0x61, 0x6E, 0x00, 0x00, 0x00], [0x4D, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00], [0x46, 0x65, 0x6C, 0x69, 0x78, 0x00, 0x00], [0x4A, 0x65, 0x6E, 0x6E, 0x61, 0x00, 0x00], [0x53, 0x61, 0x72, 0x61, 0x00, 0x00, 0x00], [0x50, 0x69, 0x65, 0x72, 0x73, 0x00, 0x00]],
  [[0x3F, 0x05, 0x81, 0x01, 0x00, 0x00, 0x00], [0x39, 0x04, 0xC1, 0x04, 0x4E, 0x02, 0x00], [0xDC, 0x08, 0xCF, 0x07, 0x00, 0x00, 0x00], [0x7A, 0x05, 0xA4, 0x08, 0xE7, 0x04, 0x00], [0xFC, 0x03, 0x14, 0x08, 0xA6, 0x08, 0x00], [0xFC, 0x03, 0x37, 0x07, 0x8B, 0x05, 0x00], [0x14, 0x08, 0x24, 0x01, 0x00, 0x00, 0x00], [0x07, 0x06, 0x8A, 0x04, 0x4E, 0x02, 0x00]],
  [[0x25, 0x23, 0x26, 0x23, 0x00, 0x00, 0x00], [0x27, 0x23, 0x28, 0x23, 0x29, 0x23, 0x00], [0x2A, 0x23, 0x2B, 0x23, 0x00, 0x00, 0x00], [0x2C, 0x23, 0x2D, 0x23, 0x2E, 0x23, 0x00], [0x2F, 0x23, 0x30, 0x23, 0x31, 0x23, 0x00], [0x2F, 0x23, 0x32, 0x23, 0x33, 0x23, 0x00], [0x30, 0x23, 0x34, 0x23, 0x00, 0x00, 0x00], [0x35, 0x23, 0x36, 0x23, 0x29, 0x23, 0x00]]
];

/// The 8th byte in save header is the slot number, it only show 3 active save data in game.
const HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX: usize = 0x07;

/// The values for 3 'active' save data are: 0x00, 0x01 and 0x02.
/// For more information, please see the comment for `convert_save`.
const MAX_VALID_SLOT_NUMBER: u8 = 0x02;

/// For TBS, the size of each save slot is 4KB.
/// For TLA, the size of each save slot is 12KB.
const SAVE_SLOT_SIZE: [usize; 2] = [0x1000, 0x3000];

/// TBS: 64KB / 4KB = 16
/// TLA: 64KB / 12KB = 5
const MAX_LOOP_COUNT: [usize; 2] = [16, 5];

/// In TBS, it should be 0 -> Robin (Isaac)
/// In TLA, it should be 4 -> Garcia (Felix)
const PARTY_MAIN_LEADER_INDEX: [usize; 2] = [0, 4];

/// In TBS, we only have four available members: Robin (Isaac), Gerald (Garet), Ivan and Mary/Mearī (Mia).
/// We should also include Garcia (Felix), Jasmine (Jenna) and Shiba (Sheba).
/// In TLA, well, We have Picard (Piers) in party now.
const PARTY_MEMBERS_COUNT: [usize; 2] = [7, 8];
const PC_NAME_LOCATION_INDEX: [usize; 2] = [0x510, 0x530];
const BUILD_DATE_LOCATION_INDEX: [[[usize; 2]; 3]; 2] = [
  [[0x36, 0x37], [0x250, 0x251], [0x508, 0x509]],
  [[0x36, 0x37], [0x250, 0x251], [0x528, 0x529]]
];

/// Save slot size - header size
/// Header size is 0x10.
/// TBS: 0x1000 - 0x10
/// TLA: 0x3000 - 0x10
const CHECKSUM_RANGE: [usize; 2] = [0xFF0, 0x2FF0];
const HEADER_CHECKSUM_LOCATION_INDEX: [usize; 2] = [0x08, 0x09];

#[derive(Clone, Copy)]
enum GameType {
  // GS1
  TheBrokenSeal,
  // GS2
  TheLostAge,
}

#[derive(Clone, Copy)]
enum NameType {
  Japanese,
  English,
  German,
  Spanish,
  French,
  Italian,
  ChineseFanTranslationMobileTeam,
  ChineseFanTranslation2023Team,
}

#[derive(Clone, Copy)]
enum BuildDateType {
  Japanese,
  English,
  German,
  Spanish,
  French,
  Italian,
}

fn main() {
  // "long_about" looks weird?
  let mut about_string = String::new();
  about_string.push_str("A simple tool for two GBA games, Golden Sun and Golden Sun: The Lost Age.\n\n");
  about_string.push_str("This tool can do two things by reading a save file:\n");
  about_string.push_str("1. Change the names of all playable characters to their default names in other languages.\n");
  about_string.push_str("2. Convert the save version by modifying the build date in the save file.\n\n");
  about_string.push_str("Note: 1. This tool also supports some other languages' fan translation version.\n");
  about_string.push_str("      2. If the build date in the save file does not match the build date the game ROM,\n");
  about_string.push_str("         the game will force the player to start the game from the sanctum.");

  let matches = Command::new("Golden Sun Save Converter")
    .version("v0.1.8")
    .author("Hambaka")
    .about(about_string)
    .allow_negative_numbers(true)
    .args(&[
      arg!(<INPUT_FILE> "Golden Sun/Golden Sun: The Lost Age save file").value_parser(value_parser!(PathBuf)).required(true),
      arg!(-n --name <VALUE> "The version of the names of playable characters"),
      arg!(-d --date <VALUE> "Build date version"),
      arg!(-o --output <OUTPUT_FILE> "Output save file location").value_parser(value_parser!(PathBuf))
    ])
    .group(ArgGroup::new("args")
      .args(["name", "date"])
      .required(true)
      .multiple(true)
    )
    .get_matches();

  let mut pc_name_type_option: Option<NameType> = None;
  if let Some(name) = matches.get_one::<String>("name") {
    pc_name_type_option = match name.as_str() {
      "j" | "k" => Some(NameType::Japanese),
      "e" | "p" => Some(NameType::English),
      "g" => Some(NameType::German),
      "s" => Some(NameType::Spanish),
      "f" => Some(NameType::French),
      "i" => Some(NameType::Italian),
      // oc -> Chinese fan translation (old, GS2 only)
      "oc" => Some(NameType::ChineseFanTranslationMobileTeam),
      // nc -> Chinese fan translation (new, GS1 only)
      "nc" => Some(NameType::ChineseFanTranslation2023Team),
      // Invalid value
      _ => {
        eprintln!("Please input a valid name type!");
        eprintln!("Available values: j, e, g, s, f, i, oc, nc, p, k");
        eprintln!(" j: Japanese, e: English, g: Germany,");
        eprintln!(" s: Spanish,  f: French,  i: Italy,");
        eprintln!("oc: Chinese (TLA only),  nc: Chinese (TBS only),");
        eprintln!(" p: English,  k: Japanese");
        eprintln!("Example: -n e");
        return;
      }
    }
  };

  let mut build_date_type_option: Option<BuildDateType> = None;
  if let Some(build_date_type) = matches.get_one::<String>("date") {
    build_date_type_option = match build_date_type.as_str() {
      // nc -> Chinese fan translation by 2023 Team (new, TBS only, based on Japanese version)
      // k -> Korean fan translation by pjs0493 (TBS & TLA, based on Japanese version).
      "j" | "nc" | "k" => Some(BuildDateType::Japanese),
      // oc -> Chinese fan translation by Mobile/Soma Team (old, TLA only, based on English version)
      // p -> Polish fan translation by Rykuzushi (TBS only, based on English version)
      "u" | "e" | "oc" | "p" => Some(BuildDateType::English),
      "g" => Some(BuildDateType::German),
      "s" => Some(BuildDateType::Spanish),
      "f" => Some(BuildDateType::French),
      "i" => Some(BuildDateType::Italian),
      // Invalid value
      _ => {
        eprintln!("Please input a valid build date type!");
        eprintln!("Available values: j, u, e, g, s, f, i, oc, nc, p, k");
        eprintln!("j:  Japan,   u: USA/Europe, e: USA/Europe");
        eprintln!("g:  Germany, s: Spanish,    f: France,    i: Italy");
        eprintln!("oc: USA/Europe (TLA only), nc: Japan (TBS only)");
        eprintln!("p:  USA/Europe,             k: Japan");
        eprintln!("Example: -d e");
        return;
      }
    };
  }

  // Read save file.
  let raw_input_path = matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
  let mut input_file = File::open(raw_input_path).expect("An error occurred while opening save file!");

  /* Check the size of save file.
     The size of save file should be 64KB,
     though the .SaveRAM file created by Bizhawk is 128KB.
     Even its size is 128KB, seems it only use first 64KB space to store save data. */
  let file_size = input_file.metadata().unwrap().len();
  if file_size != 0x10000 && file_size != 0x20000 {
    eprintln!("The size of save file is not valid!");
    return;
  }

  // Get raw save.
  let mut raw_save_file = Vec::new();
  input_file.read_to_end(&mut raw_save_file).unwrap();

  // Detect game/save type, also get loop start index.
  let (game_type_option, loop_start_index_option) = get_game_type_with_loop_start_index_option(&raw_save_file);
  if game_type_option.is_none() {
    eprintln!("It's not a valid Golden Sun/Golden Sun: The Lost age save file! Or there is no save data in save file!");
    return;
  }
  let loop_start_index = loop_start_index_option.unwrap();

  // Only for Chinese fan translations.
  if let Some(name_type) = pc_name_type_option {
    if let Some(game_type) = game_type_option {
      if (matches!(name_type, NameType::ChineseFanTranslationMobileTeam) && matches!(game_type, GameType::TheBrokenSeal)) || (matches!(name_type, NameType::ChineseFanTranslation2023Team) && matches!(game_type, GameType::TheLostAge)) {
        eprintln!("This combination is not supported!");
        return;
      }
    }
  }

  // Get output save data file.
  let output_save = convert_save(raw_save_file, game_type_option, loop_start_index, pc_name_type_option, build_date_type_option);
  // Start to create and write output save file.
  let output_path;
  let mut output_file;
  if let Some(raw_output) = matches.get_one::<PathBuf>("output") {
    output_path = PathBuf::from(raw_output);
    let output_dir = output_path.parent().unwrap();
    fs::create_dir_all(output_dir).expect("Failed to create directory!");
  } else {
    let path = Path::new(raw_input_path);
    let parent = path.parent().unwrap().to_str().unwrap();
    let file_stem = path.file_stem().unwrap().to_str().unwrap();

    let file_extension = if path.extension().is_none() {
      ""
    } else {
      path.extension().unwrap().to_str().unwrap()
    };

    let mut file_name_str = String::new();
    file_name_str.push_str(file_stem);
    file_name_str.push_str("_output");

    if !file_extension.is_empty() {
      file_name_str.push('.');
      file_name_str.push_str(file_extension);
    }

    output_path = PathBuf::from(parent).join(file_name_str);
  }
  output_file = File::create(output_path.clone()).unwrap_or_else(|_| panic!("Failed to create \"{}\"!", output_path.to_str().unwrap()));
  output_file.write_all(&output_save).unwrap_or_else(|_| panic!("Failed to create \"{}\"!", output_path.to_str().unwrap()));
}

fn get_game_type_with_loop_start_index_option(raw_save_file: &[u8]) -> (Option<GameType>, Option<usize>) {
  for i in 0..MAX_LOOP_COUNT[0] {
    let Ok(header_string) = std::str::from_utf8(&raw_save_file[(i * SAVE_SLOT_SIZE[0])..(i * SAVE_SLOT_SIZE[0] + HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX)]) else { continue; };
    if !header_string.eq(HEADER_CAMELOT_ASCII_STRING) {
      continue;
    }

    let build_date_from_raw_save_as_tbs = u16::from_le_bytes([raw_save_file[i * SAVE_SLOT_SIZE[0] + BUILD_DATE_LOCATION_INDEX[0][0][0]], raw_save_file[i * SAVE_SLOT_SIZE[0] + BUILD_DATE_LOCATION_INDEX[0][0][1]]]);
    if GS_BUILD_DATE[0].contains(&build_date_from_raw_save_as_tbs) {
      return (Some(GameType::TheBrokenSeal), Some(i));
    }
    let build_date_from_raw_save_as_tla = u16::from_le_bytes([raw_save_file[i * SAVE_SLOT_SIZE[0] + BUILD_DATE_LOCATION_INDEX[0][0][0]], raw_save_file[i * SAVE_SLOT_SIZE[0] + BUILD_DATE_LOCATION_INDEX[0][0][1]]]);
    if GS_BUILD_DATE[1].contains(&build_date_from_raw_save_as_tla) {
      return (Some(GameType::TheLostAge), Some(i / 3));
    }
  }

  (None, None)
}

/* Links to other Golden Sun reference guide (save editing):
   https://gamefaqs.gamespot.com/gba/468548-golden-sun/faqs/43776
   https://gamefaqs.gamespot.com/gba/561356-golden-sun-the-lost-age/faqs/30811
   ----------------------------------------------------------------------------------
   More reference info/comment about GBA Golden Sun series save file from Dyrati (in "Obababot")

   https://github.com/Dyrati/obababot/blob/main/obababot/gsfuncs.py
   At line 579, the "get_save_data" function takes raw binary .sav data and returns individual save slots with all of the info from each valid save.
   The function checks the file at 0x1000 byte intervals.

   The first 16 bytes of each interval (the header) are organized as follows:
   - 7 bytes for the ASCII string "CAMELOT"
   - 1 byte for the slot number
   - 2 bytes for a checksum
   - 2 bytes for a priority number
   - 4 bytes of garbage data

   A header is valid if the first 7 bytes spell "CAMELOT", and the slot number is less than 16.
   In the case where multiple headers have the same slot number, use the header with the highest priority number.
   That should leave you with up to 3 valid headers.
   The next 0x2FF0 bytes after the header constitute the save data for that file. (Note: GS2 only)
   ----------------------------------------------------------------------------------
   Additional reference info/comment about the first Golden Sun save file from Dyrati

   For GS1, each save splits into two parts.
   In the .sav file, each section is 0x1000 bytes long.
   However two separate sections are joined together to create one save file.
   Some sections have slot numbers of 3, 4, or 5,
   those sections are the second half of slots 0, 1, and 2 respectively.
   But seems the second half of the save doesn't store the data for generating password. */
fn convert_save(mut raw_save_file: Vec<u8>, game_type_option: Option<GameType>, loop_start_index: usize, pc_name_type_option: Option<NameType>, build_date_type_option: Option<BuildDateType>) -> Vec<u8> {
  let game_type_index = match game_type_option.unwrap() {
    GameType::TheBrokenSeal => 0,
    GameType::TheLostAge => 1,
  };

  for i in loop_start_index..MAX_LOOP_COUNT[game_type_index] {
    // Skip "invalid" save data.
    if raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX] > MAX_VALID_SLOT_NUMBER {
      continue;
    }
    // Skip "invalid" save data.
    let build_date_from_raw_save = u16::from_le_bytes([raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + BUILD_DATE_LOCATION_INDEX[game_type_index][0][0]], raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + BUILD_DATE_LOCATION_INDEX[game_type_index][0][1]]]);
    if !GS_BUILD_DATE[game_type_index].contains(&build_date_from_raw_save) {
      continue;
    }

    if let Some(pc_name_type) = pc_name_type_option {
      let pc_name_type_index: usize = match pc_name_type {
        NameType::Japanese => 0,
        NameType::English => 1,
        NameType::German => 2,
        NameType::Spanish => 3,
        NameType::French => 4,
        NameType::Italian => 5,
        NameType::ChineseFanTranslationMobileTeam => 6,
        NameType::ChineseFanTranslation2023Team => 7,
      };

      let mut is_main_leader = true;
      /* The party leader name.
         This name only shows in save select menu, it's different from the actual names for party members.
         The max size for this name is 12 bytes, though normally the max size we can use for character name is 10 bytes (Like those Kana in Japanese version)

         Robin (Isaac) is the only possible party leader in Golden Sun.
         But in Golden Sun: The Lost Age, there are two possible party leaders: Garcia (Felix) and Jasmine (Jenna).
         Garcia is the main leader in TLA. */
      if game_type_index == 1 {
        for j in 0..12 {
          // Compare the name to Garcia's name, to see if this name is same as main leader Garcia's name.
          if raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j] != raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + PC_NAME_LOCATION_INDEX[game_type_index] + PARTY_MAIN_LEADER_INDEX[game_type_index] * 0x14C + j] {
            is_main_leader = false;
            break;
          }
        }
      }

      if is_main_leader {
        /* Golden Sun + is_main_leader(always true) -> Robin
           Golden Sun: The Lost Age + is_main_leader -> Garcia */
        for j in 0..12 {
          if j < 7 {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j] = PC_NAME[pc_name_type_index][PARTY_MAIN_LEADER_INDEX[game_type_index]][j];
          } else {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j] = 0x00;
          }
        }
      } else {
        // Golden Sun: The Lost Age + !is_main_leader -> Jasmine
        for j in 0..12 {
          if j < 7 {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j] = PC_NAME[pc_name_type_index][PARTY_MAIN_LEADER_INDEX[game_type_index] + 1][j];
          } else {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j] = 0x00;
          }
        }
      }

      // Change all party members' names.
      for j in 0..PARTY_MEMBERS_COUNT[game_type_index] {
        for k in 0..15 {
          if k < 7 {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + PC_NAME_LOCATION_INDEX[game_type_index] + j * 0x14C + k] = PC_NAME[pc_name_type_index][j][k];
          } else {
            raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + PC_NAME_LOCATION_INDEX[game_type_index] + j * 0x14C + k] = 0x00;
          }
        }
      }
    }

    /* Change build date (to "convert" save data "version")
       If the build date in save data doesn't match the one in game,
       the game will force player to start from sanctum after loading save.
       Every language version has a different build date. */
    if let Some(build_date_type) = build_date_type_option {
      let build_date_type_index = match build_date_type {
        BuildDateType::Japanese => 0,
        BuildDateType::English => 1,
        BuildDateType::German => 2,
        BuildDateType::Spanish => 3,
        BuildDateType::French => 4,
        BuildDateType::Italian => 5,
      };

      let build_date = GS_BUILD_DATE[game_type_index][build_date_type_index].to_le_bytes();
      for j in 0..2 {
        raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + BUILD_DATE_LOCATION_INDEX[game_type_index][0][j]] = build_date[j];
        raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + BUILD_DATE_LOCATION_INDEX[game_type_index][1][j]] = build_date[j];
        raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + BUILD_DATE_LOCATION_INDEX[game_type_index][2][j]] = build_date[j];
      }
    }

    /* Calculate save's checksum (16 bit, doesn't need to calculate each save's first 0x10 header),
       if the checksum of save data doesn't match the one in header,
       the game will not consider it as a valid save.
       If the checksum exceeds 4 digits(Hexadecimal, not decimal), just discard extra digits. */
    let mut checksum = 0;
    for j in 0..CHECKSUM_RANGE[game_type_index] {
      checksum += u32::from(raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + 0x10 + j]);
    }
    let checksum_bytes = checksum.to_le_bytes();
    for j in 0..2 {
      raw_save_file[i * SAVE_SLOT_SIZE[game_type_index] + HEADER_CHECKSUM_LOCATION_INDEX[j]] = checksum_bytes[j];
    }
  }

  raw_save_file
}
