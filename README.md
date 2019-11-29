# date-shortener
A tool to shorten (encode) the date and expand (decode) shortened date back to original date

## Introduction
I have developed a new way to encode/decode dates within 3 characters by using base-36 format.

#### What is a base 36 format?

> Base 36 or hexatridecimal is a positional numeral system using 36 as the radix. The choice of 36 is convenient in that the digits can be represented using the Arabic numerals 0-9 and the Latin letters A-Z.

In simple language, base-36 format refers to series of 0-9 followed by A-Z characters, i.e., 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ; where each character refers to the position index in series.

e.g., `12 → C, 19 → J, 34 → Y`

Now here is a way to use the same logic for date, so to do that let us take a date and encode it as

`25-Oct-2019 → 25-10-2019 → 25.10.19 → P.A.J → paj`

Similarly, decode it back as

`paj → P.A.J → 25.10.19 → 2019-10-25 → 25-Oct-2019`

Note, here `P.A.J` is converted to lowercase as paj to make it more compact.

## Usage
Open the application in terminal & run the required commands as shown below

### Examples
Few sample usages are given below

```
$ ds -t
tbj
$ ds -t -e
29-11-2019 -> 29.11.19 -> T.B.J -> tbj
$ ds -c tbj -e
tbj -> T.B.J -> 29.11.19 -> 29-11-2019
$ ds -d 28/11/2019 -e
28-11-2019 -> 28.11.19 -> S.B.J -> sbj
```

### Help
Find out all the available command options & flags 

```
$ ds -h
Date Shortener is a tool to shorten (encode) the date and expand (decode) shortened date back to original date. 

Usage: ./target/release/ds [flag] [options]

Options:
    -d, --date DD-MM-YYYY
                        encode the provided date
    -c, --code DMY      decode the provided code
    -t, --today         encode today's date
    -e, --explain       explain with steps
    -h, --help          display the help menu
    -v, --version       display the application version

Examples: 
 $ ds -v 
 $ ds -t 
 $ ds -t -e 
 $ ds -d 15/08/2019 
 $ ds -d 15/08/2019 -e 
 $ ds -c f8j 
 $ ds -c f8j -e 

```

### Version
See the currently available version

```
$ ds -v
Date Shortener 
Version 1.0.0 
Licensed under MIT License
```

## Development
To generate the build and run these commands

### Debug build

```
$ cd date-shortener
$ cargo build
$ $ ./target/debug/ds -h
```

### Release build

```
$ cd date-shortener
$ cargo build --release
$ ./target/release/ds -h
```
## References
- [Encode or decode date in 3 characters](http://akzcool.blogspot.com/2019/10/encode-or-decode-in-3-characters.html)
