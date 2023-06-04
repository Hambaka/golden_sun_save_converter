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

## README  

[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English%20(TODO)-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_save_converter/blob/main/README.en-US.md)

## 使用方法

```text
使用方法：golden_sun_save_converter [选项] <--name <VALUE>|--date <VALUE>> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》或《黄金太阳 失落的时代》的存档文件

选项：
  -n, --name <VALUE>          主角团姓名的语言版本
  -d, --date <VALUE>          构建日期的版本
  -o, --output <OUTPUT_FILE>  输出的存档文件保存位置
```

### 说明  

- 各参数和选项的输入位置随意，没有先后顺序的限制。  
- `<INPUT_FILE>` 为《黄金太阳 开启的封印》或《黄金太阳 失落的时代》的存档文件，**必要参数**。  
- `name` 为**可选选项**，有效的值为：
  - `j, e, g, s, f, i, oc, nc, p, k`  
  - `j：日文版，e：英文版，g：德文版，s：西班牙文版，f：法文版，i：意大利文版，oc：2代民间老汉化版，nc：1代民间新汉化版，p：民间波兰文版（同英文版），k：民间韩文版（同日文版）`  
- `date` 为**可选选项**，有效的值为：
  - `j, u, e, g, s, f, i, oc, nc, p, k`  
  - `j：日版，u：欧/美版，e：欧/美版，g：德版，s：西班牙版，f：法版，i：意大利版，oc：2代民间老汉化版（同欧/美版），nc：1代民间新汉化版（同日版），p：民间波兰版（同欧/美版），k：民间韩版（同日版）`。  
- `name` 和 `date` 虽皆为可选选项，但是**必须要有其中一个**。  
- `output` 是**可选选项**，若不使用会默认将转换后的存档文件保存到输入文件的同目录下。  

### 示例  

完整命令：

```bash
golden_sun_save_converter 输入存档.sav --name e --date u --output 输出存档.sav
```

完整命令简易版：

```bash
golden_sun_save_converter 输入存档.sav -n e -d u -o 输出存档.sav
```

只修改主角团名字，修改为日文默认名（不指定输出存档的保存位置位置）：  

```bash
golden_sun_save_converter 输入存档.sav -n j
```

只修改游戏构建日期，即仅转换存档版本为日版（不指定输出存档的保存位置位置）：  

```bash
golden_sun_save_converter 输入存档.sav -d j
```
