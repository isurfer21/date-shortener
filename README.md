# date-shortener
A tool to shorten (encode) the date and expand (decode) shortened date back to original date

## Introduction
I have developed a new way to encode/decode dates within 3 or 4 characters by using base-99 format.

#### What is a base 99 format?

In simple language, base-99 format refers to series of 0-9 followed by small & capital A-Z characters and variations of vowel characters, i.e., 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZàèìòùÀÈÌÒÙáéíóúÁÉÍÓÚâêîôûÂÊÔÛÎäëïöüÄËÏ; where each character refers to the position index in series.

e.g., `12 → C, 19 → J, 34 → Y`

Now here is a way to use the same logic for date, so to do that let us take a date and encode it as

`15-8-2019 -> 15.8.2019 -> f.8.kj -> f8kj`

Similarly, decode it back as

`f8kj -> f.8.kj -> 15.8.2019 -> 15-8-2019`

## Usage
Open the application in terminal & run the required commands as shown below

### Examples
Few sample usages are given below

```
$ ds -t
2ckj
$ ds -t -s
2-12-2019 -> 2.12.2019 -> 2.c.kj -> 2ckj
$ ds -d 2ckj -s
2ckj -> 2.c.kj -> 2.12.2019 -> 2-12-2019
$ ds -e 2/12/2019 -s
2-12-2019 -> 2.12.2019 -> 2.c.kj -> 2ckj
```

### Help
Find out all the available command options & flags 

```
$ ds -h
Date Shortener is a tool to shorten (encode) the date and expand (decode) shortened date back to original date. 

Usage: ds [flag] [options]

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
Version 0.2.0 
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
