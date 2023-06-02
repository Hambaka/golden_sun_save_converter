# README (待完善)
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.en-US.md)
---
# golden_sun_save_converter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_save_converter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_save_converter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_save_converter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_save_converter/releases/latest)

为 GBA 上的 **《黄金太阳 开启的封印》** 和 **《黄金太阳 失落的时代》** 开发的一个很简单的小工具。  
本工具可以通过读取存档文件来实现以下两个功能：  
- 将主角团成员的名字更改为其他语言版本中的默认名。  
- 通过修改存档文件中的游戏构建日期来转换存档版本。  

注：  
- 本工具不仅支持官方版本，也支持一些民间翻译版。  
- 如果存档文件中的构建日期与游戏 ROM 中的构建日期不匹配，游戏将会强制玩家从神殿开始游戏。  

## 使用方法
```
使用方法：golden_sun_save_converter.exe [选项] <--name <VALUE>|--date <VALUE>> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》或《黄金太阳 失落的时代》的存档文件

选项：
  -n, --name <VALUE>          主角团姓名的语言版本
  -d, --date <VALUE>          构建日期的版本
  -o, --output <OUTPUT_FILE>  输出的存档文件保存位置
```