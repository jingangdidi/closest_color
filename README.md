# closest_color

**find closest color from 657 R colors or specified colors**

**从657个R颜色（默认）或指定的待选颜色（-d）中寻找与指定颜色（-c）最接近的颜色**

## Arguments
```
Usage: closest_color.exe -c <color> [-d <candidate>] [-n <num>] [-a <algorithm>]

find closest color

Options:
  -c, --color       colors, can be a file(one color per line) or colon separated multiple colors, support rgb(0-255 or 0~1, e.g. 171,193,35:0.3,0.8,0.1) and hex(with or without the "#", shorthand of three letters format, e.g. #f034e6:f034e6:f3e)
  -d, --candidate   candidate colors, can be a file or colon separated colors, default 657 R colors
  -n, --num         number of closest colors, default: 1
  -a, --algorithm   distance/difference algorithm, 1(Ciede2000), 2(DeltaE), 3(EuclideanDistance), default: 1
  --help, help      display usage information
```

## download pre-built binary

[latest release](https://github.com/jingangdidi/closest_color/releases)

## Usage
**1. Find the closest one to the specified color '#DEB887' among the 657 colors in R**
```
./closest_color.exe -c #DEB887

# +-------------------------------------------------+
# | #DEB887 1 closest colors: #DEB887(burlywood, 0) |
# +-------------------------------------------------+
```
**2. Among the 657 colors in R, search for the 5 colors that are closest to the specified 2 colors '#DEB887' and '000', and use the DeltaE algorithm**
```
./closest_color.exe -c #DEB887:000 -n 5 -a 2

# +-------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
# | #DEB887 5 closest colors: #DEB887(burlywood, 0), #EEC591(burlywood2, 5.171824), #CDAA7D(burlywood3, 5.7135954), #D2B48C(tan, 6.28841), #CDB38B(navajowhite3, 7.6204195) |
# | 000 5 closest colors: #000000(black, 0), #000000(gray0, 0), #000000(grey0, 0), #030303(gray1, 0.822525), #030303(grey1, 0.822525)                                       |
# +-------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
```
**3. Search for the closest one among the three specified colors, '#DEB887' and '#000', respectively**
```
./closest_color.exe -c #DEB887:000 -d #FAEBD7:#FFEFDB:#EEDFCC

# +----------------------------------------------+
# | #DEB887 1 closest colors: #EEDFCC(13.177636) |
# | 000 1 closest colors: #EEDFCC(84.99748)      |
# +----------------------------------------------+
```

## Building from source
```
git clone https://github.com/jingangdidi/closest_color.git
cd chatsong
cargo build --release
```

