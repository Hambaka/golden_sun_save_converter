# golden_sun_save_converter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_save_converter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_save_converter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_save_converter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_save_converter/releases/latest)

A simple tool for two GBA games, Golden Sun and Golden Sun: The Lost Age.  
This tool can do two things by reading a save file:  

- Change the names of all playable characters to their default names in other languages.  
- Convert the save version by modifying the build date in the save file.  

Note:  

- This tool also supports some other languages' fan translation version.  
- If the build date in the save file does not match the build date the game ROM, the game will force the player to start the game from the sanctum.  

## README  

[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English%20(TODO)-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.en-US.md)

## Usage

```text
Usage: golden_sun_save_converter [OPTIONS] <--name <VALUE>|--date <VALUE>> <INPUT_FILE>

Arguments:
  <INPUT_FILE>
          Golden Sun/Golden Sun: The Lost Age save file

Options:
  -n, --name <VALUE>
          The version of the names of playable characters

          Possible values:
          - j:  Japanese
          - e:  English
          - g:  German
          - s:  Spanish
          - f:  French
          - i:  Italian
          - oc: Chinese fan translation by Mobile Team, TLA only
          - nc: Chinese fan translation by 2023 Team, TBS only
          - p:  Polish fan translation, TBS only, same as "English"
          - k:  Korean fan translation, same as "Japanese"

  -d, --date <VALUE>
          Build date version

          Possible values:
          - j:  Japan
          - u:  USA, Europe
          - e:  USA, Europe
          - g:  Germany
          - s:  Spain
          - f:  France
          - i:  Italy
          - oc: Chinese fan translation by Mobile Team, TLA only, same as "USA, Europe"
          - nc: Chinese fan translation by 2023 Team, TBS only, same as "Japan"
          - p:  Polish fan translation, TBS only, same as "USA, Europe"
          - k:  Korean fan translation, same as "Japan"

  -o, --output <OUTPUT_FILE>
          Output save file location
```
