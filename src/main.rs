use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::string::String;
use clap::{Command, arg, ArgGroup, value_parser};

/* Golden Sun/Golden Sun: The Lost Age build date
   Source: Golden Sun Hacking Community Discord Server
   GS1 (J) = 0x159C
   GS1 (U) = 0x1652
   GS1 (G) = 0x1849
   GS1 (S) = 0x1885
   GS1 (F) = 0x1713
   GS1 (I) = 0x1886

   GS2 (J) = 0x198A
   GS2 (U) = 0x1C85
   GS2 (G) = 0x1D97
   GS2 (S) = 0x1DC7
   GS2 (F) = 0x1D98
   GS2 (I) = 0x1DC8 */
const GS_BUILD_DATE: [[[u8; 2]; 6]; 2] = [
  [[0x9C, 0x15], [0x52, 0x16], [0x49, 0x18], [0x85, 0x18], [0x13, 0x17], [0x86, 0x18]],
  [[0x8A, 0x19], [0x85, 0x1C], [0x97, 0x1D], [0xC7, 0x1D], [0x98, 0x1D], [0xC8, 0x1D]]
];

/* Main characters' default names in different languages.

   Japanese:                                   "ロビン", "ジェラルド", "イワン", "メアリィ", "ガルシア", "ジャスミン", "シバ",  "ピカード"
   English:                                    "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Sheba", "Piers"
   German:                                     "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Cosma", "Aaron"
   Spanish:                                    "Hans",   "Garet",      "Iván",   "Mia",      "Félix",    "Nadia",      "Sole",  "Piers"
   French:                                     "Vlad",   "Garet",      "Ivan",   "Sofia",    "Pavel",    "Lina",       "Cylia", "Piers"
   Italian:                                    "Isaac",  "Garet",      "Ivan",   "Mia",      "Felix",    "Jenna",      "Sara",  "Piers"
   Chinese fan translation for GS2 (Share same encoding)
   ├── Simplified Chinese:                     "罗宾",   "杰拉德",     "伊万",   "米雅莉",   "加西亚",   "加斯敏",     "西芭",  "皮卡德"
   └── Traditional Chinese:                    "羅賓",   "傑拉德",     "伊萬",   "米雅莉",   "加西亞",   "加斯敏",     "西芭",  "皮卡德"
   Simplified Chinese fan translation for GS1: "罗宾",   "杰拉德",     "伊万",   "梅雅莉",   "加西亚",   "加斯敏",     "西芭",  "皮卡德" */
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

fn main() {
  let matches = Command::new("Golden Sun Save Converter")
    .version("0.1.2")
    .author("Hambaka")
    .about("Read save data to change all party members' names to the default name in other language versions, \nand save data can be converted to aother language version by editing build date.")
    .allow_negative_numbers(true)
    .arg(
      arg!(
        <INPUT_FILE> "GS1/GS2 save file"
        )
        .value_parser(value_parser!(PathBuf))
        .required(true)
    )
    .arg(
      arg!(
        -g --game <VALUE> "input 1 for GS1 save file, 2 for GS2"
      )
        .required(true)
        .requires("content")
    )
    .arg(
      arg!(
        -n --name <VALUE> "Change party members' names"
      ).required(false)
    )
    .arg(
      arg!(
        -b --build <VALUE> "Change save version (language)"
      ).required(false)
    )
    .group(
      ArgGroup::new("content")
        .required(true)
        .args(&["name", "build"])
        .multiple(true)
    )
    .arg(
      arg!(
        -o --output <OUTPUT_FILE> "Output save file location"
      )
        .value_parser(value_parser!(PathBuf))
        .required(false)
    )
    .get_matches();

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

  let game = matches.get_one::<String>("game").unwrap();
  if !game.eq("1") && !game.eq("2") {
    eprintln!("Please input a valid game type value!");
    return;
  }

  let name_type;
  if let Some(name_option) = matches.get_one::<String>("name") {
    name_type = match name_option.as_str() {
      // j -> Japanese version
      "j" => 0,
      // e -> English version
      "e" => 1,
      // g -> German version
      "g" => 2,
      // s -> Spanish version
      "s" => 3,
      // f -> French version
      "f" => 4,
      // i -> Italian version
      "i" => 5,
      // oc -> Chinese fan translation (old, GS2 only)
      "oc" => 6,
      // nc -> Chinese fan translation (new, GS1 only)
      "nc" => 7,
      // 100 -> Invalid value
      _ => 100
    };
  } else {
    // 255 -> Don't change party members' names.
    name_type = 255;
  }

  let build_date_type;
  if let Some(build_date_option) = matches.get_one::<String>("build") {
    build_date_type = match build_date_option.as_str() {
      // j -> Japanese version
      "j" => 0,
      // e -> English version
      "e" => 1,
      // g -> German version
      "g" => 2,
      // s -> Spanish version
      "s" => 3,
      // f -> French version
      "f" => 4,
      // i -> Italian version
      "i" => 5,
      // oc -> Chinese fan translation (old, GS2 only, based on English version)
      "oc" => 1,
      // nc -> Chinese fan translation (new, GS1 only, based on Japanese version)
      "nc" => 0,
      // 100 -> Invalid value
      _ => 100
    };
  } else {
    // 255 -> Don't change build date value.
    build_date_type = 255;
  }

  if name_type == 100 {
    eprintln!("Please input a valid name type value!");
    return;
  }

  if build_date_type == 100 {
    eprintln!("Please input a valid build date type value!");
    return;
  }

  if (game.eq("1") && name_type == 6) || (game.eq("2") && name_type == 7) {
    eprintln!("This combination is not supported!");
    return;
  }

  // Get output save data file.
  let mut raw_save_file = Vec::new();
  input_file.read_to_end(&mut raw_save_file).unwrap();

  let output_save = convert_save(raw_save_file, game, name_type, build_date_type);

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

    let file_extension;
    if path.extension().is_none() {
      file_extension = "";
    } else {
      file_extension = path.extension().unwrap().to_str().unwrap();
    }

    let mut file_name_str = String::new().to_owned();
    file_name_str.push_str(file_stem);
    file_name_str.push_str("_output");

    if file_extension.len() != 0 {
      file_name_str.push_str(".");
      file_name_str.push_str(file_extension);
    }

    output_path = PathBuf::from(parent).join(file_name_str);
  }
  output_file = File::create(output_path.clone()).expect(&*format!("Failed to create \"{}\"!", output_path.to_str().unwrap()));
  output_file.write_all(&*output_save).expect(&*format!("Failed to create \"{}\"!", output_path.to_str().unwrap()));
}

/* Links to other Golden Sun reference guide (save editing):
   https://gamefaqs.gamespot.com/gba/468548-golden-sun/faqs/43776
   https://gamefaqs.gamespot.com/gba/561356-golden-sun-the-lost-age/faqs/30811 */
fn convert_save(mut raw_save_file: Vec<u8>, game: &str, name_type: usize, build_date_type: usize) -> Vec<u8> {
  let mut blank_save_slot_count = 0;
  let camelot_header = [0x43u8, 0x41u8, 0x4Du8, 0x45u8, 0x4Cu8, 0x4Fu8, 0x54u8];

  let game_type;
  let max_loop;
  let save_slot_size;
  let party_leader;
  let party_members_count;
  let name_location;
  let mut build_date_location = [0x36usize, 0x250usize, 0x00usize];
  let checksum_range;

  if game.eq("1") {
    // 0 -> Golden Sun
    game_type = 0;
    // The size of each save slot is 4KB.
    save_slot_size = 0x1000;
    // 64KB / 4KB = 16
    max_loop = 16;
    // 0 -> Robin (Isaac)
    party_leader = 0;
    // Include Garcia (Felix), Jasmine (Jenna) and Shiba. (Sheba)
    party_members_count = 7;
    name_location = 0x510;
    build_date_location[2] = 0x508;
    // 0x1000 - 0x10 (header size)
    checksum_range = 0xFF0;
  } else {
    // 1 -> Golden Sun: The Lost Age
    game_type = 1;
    // The size of each save slot is 12KB.
    save_slot_size = 0x3000;
    // 64KB / 12KB = 5
    max_loop = 5;
    // 4 -> Garcia (Felix)
    party_leader = 4;
    // We have Picard (Piers) in party now.
    party_members_count = 8;
    name_location = 0x530;
    build_date_location[2] = 0x528;
    // 0x3000 - 0x10 (header size)
    checksum_range = 0x2FF0;
  }

  for i in 0..max_loop {
    /* A lazy way to check if save slot has no save data.
       If the first byte is "FF", that means this slot does not contain any save data,
       then skip current iteration. */
    if raw_save_file[i * save_slot_size] == 0xFF {
      blank_save_slot_count += 1;
      continue;
    }

    /* A lazy and inaccurate way to detect if save file is Golden Sun/Golden Sun: The Lost Age save file.
       In Golden Sun/Golden Sun: The Lost Age, each save data(slot) take 4KB (0x1000)/ 12KB (0x3000) space.
       The first 7 bytes of each slot containing save data are "CAMELOT". */
    for j in 0..7 {
      if raw_save_file[i * save_slot_size + j] != camelot_header[j] {
        match game_type {
          0 => eprintln!("The input save file is not a Golden Sun save file!"),
          _ => eprintln!("The input save file is not a Golden Sun: The Lost Age save file!")
        }
        process::exit(1);
      }
    }

    // A lazy way to check if you are using a GS2 save file as GS1 save file.
    if game_type == 1 && ((raw_save_file[i * save_slot_size + 0x1000] == camelot_header[0] || raw_save_file[i * save_slot_size + 0x1000] == 0xFF) || (raw_save_file[i * save_slot_size + 0x2000] == camelot_header[0] || raw_save_file[i * save_slot_size + 0x2000] == 0xFF)) {
      eprintln!("The input save file is not a Golden Sun: The Lost Age save file!");
      process::exit(1);
    }

    /* Some backup save data does not store names and build date, so I think maybe I should skip this kind of save data...
       Another lazy way to detect it, if there is a valid name, the name should end with 0x00. */
    if raw_save_file[i * save_slot_size + 0x10] != 0x00 && raw_save_file[i * save_slot_size + 0x11] == 0x00 && raw_save_file[i * save_slot_size + 0x12] != 0x00 {
      continue;
    }

    if name_type != 255 {
      let mut is_main_leader = true;
      /* The party leader name.
         This name only shows in save select menu, it's different from the actual names for party members.
         The max size for this name is 12 bytes, though normally the max size we can use for character name is 10 bytes (Like those Kana in Japanese version)

         Robin (Isaac) is the only possible party leader in Golden Sun.
         But in Golden Sun: The Lost Age, there are two possible party leaders: Garcia (Felix) and Jasmine (Jenna).
         Garcia is the main leader in TLA. */
      if game_type == 1 {
        for j in 0..12 {
          // Compare the name to Garcia's name, to see if this name is same as main leader Garcia's name.
          if raw_save_file[i * save_slot_size + 0x10 + j] != raw_save_file[i * save_slot_size + name_location + party_leader * 0x14C + j] {
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
            raw_save_file[i * save_slot_size + 0x10 + j] = PC_NAME[name_type][party_leader][j];
          } else {
            raw_save_file[i * save_slot_size + 0x10 + j] = 0x00;
          }
        }
      } else {
        // Golden Sun: The Lost Age + !is_main_leader -> Jasmine
        for j in 0..12 {
          if j < 7 {
            raw_save_file[i * save_slot_size + 0x10 + j] = PC_NAME[name_type][party_leader + 1][j];
          } else {
            raw_save_file[i * save_slot_size + 0x10 + j] = 0x00;
          }
        }
      }

      // Change all party members' names.
      for j in 0..party_members_count {
        for k in 0..15 {
          if k < 7 {
            raw_save_file[i * save_slot_size + name_location + j * 0x14C + k] = PC_NAME[name_type][j][k];
          } else {
            raw_save_file[i * save_slot_size + name_location + j * 0x14C + k] = 0x00;
          }
        }
      }
    }

    /* Change build date (to "convert" save data "version")
       If the build date in save data doesn't match the one in game,
       the game will force player to start from sanctum after loading save.
       Every language version has a different build date. */
    if build_date_type != 255 {
      for j in 0..2 {
        raw_save_file[i * save_slot_size + build_date_location[0] + j] = GS_BUILD_DATE[game_type][build_date_type][j];
        raw_save_file[i * save_slot_size + build_date_location[1] + j] = GS_BUILD_DATE[game_type][build_date_type][j];
        raw_save_file[i * save_slot_size + build_date_location[2] + j] = GS_BUILD_DATE[game_type][build_date_type][j];
      }
    }

    /* Calculate save's checksum (16 bit, doesn't need to calculate each save's first 0x10 header),
       if the checksum of save data doesn't match the one in header,
       the game will not consider it as a valid save.
       If the checksum exceeds 4 digits(Hexadecimal, not decimal), just discard extra digits. */
    let mut checksum = 0;
    for j in 0..checksum_range {
      checksum += raw_save_file[i * save_slot_size + 0x10 + j] as u32;
    }
    let checksum_bytes = checksum.to_le_bytes();
    raw_save_file[i * save_slot_size + 0x08] = checksum_bytes[0];
    raw_save_file[i * save_slot_size + 0x09] = checksum_bytes[1];
  }

  if blank_save_slot_count == max_loop {
    eprintln!("The save file has no save data!");
    process::exit(1);
  }

  return raw_save_file;
}
