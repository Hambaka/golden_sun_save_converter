# README (TODO)
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.en-US.md)
---
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

## Usage
```
Usage: golden_sun_save_converter.exe [OPTIONS] <--name <VALUE>|--date <VALUE>> <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Golden Sun/Golden Sun: The Lost Age save file

Options:
  -n, --name <VALUE>          The version of the names of playable characters
  -d, --date <VALUE>          Build date version
  -o, --output <OUTPUT_FILE>  Output save file location
```