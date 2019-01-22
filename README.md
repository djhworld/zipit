zipit
-------

Takes a line from two files and prints them to stdout, until either file reaches EOF.

Includes option to cycle the first file (`left_file`) indefinitely.

Usage:

```
zipit 0.1.0
djhworld <@djhworld>
Takes a line from each input file and prints them to stdout, until either file reaches EOF

USAGE:
    zipit [FLAGS] <LEFT_FILE> <RIGHT_FILE>

FLAGS:
    -c, --cycle      Cycle all lines from <LEFT_FILE> indefinitely
    -h, --help       Prints help information
    -j, --json       Output as JSON
    -V, --version    Prints version information

ARGS:
    <LEFT_FILE>     
    <RIGHT_FILE>    
```

Alternatively the second file can be provided via stdin by either providing a `-`


```
cat file2.txt | zipit --cycle file1.txt -
```

or omitting it all together

```
cat file2.txt | zipit --cycle file1.txt
```

Example use cases
-------

### CSV to JSON 

You have a CSV and you want to convert it to a bunch of JSON-lines.

```
$ cat animals.csv
name,type,gender
fido,dog,m
foofoo,dog,f
mittens,cat,f
casper,mouse,m
```

```
$ tail -n +2 animals.csv | transpose | zipit --json --cycle <(head -n 1 animals.csv | transpose)
{"name":"fido","type":"dog","gender":"m"}
{"name":"foofoo","type":"dog","gender":"f"}
{"name":"mittens","type":"cat","gender":"f"}
{"name":"casper","type":"mouse","gender":"m"}
```

### Parse log file into readable format

You have a log file that you know the structure of (e.g. web access logs) and want to quickly see the value of each field

```
$ cat fields.txt
date
time
sc-status
cs-uri-stem
sc-bytes
```

```
$ cat log.txt
2019-01-01	11:32:33	200	/	102
2019-01-01	11:32:34	200	/about.html	394
2019-01-01	11:32:35	200	/blog.html	9338
```

```
$ zipit --cycle fields.txt <(cat logs.txt | perl -pe 's/\t/\n/g')
date	2019-01-01
time	11:32:33
sc-status	200
cs-uri-stem	/
sc-bytes	102

date	2019-01-01
time	11:32:34
sc-status	200
cs-uri-stem	/about.html
sc-bytes	394

date	2019-01-01
time	11:32:35
sc-status	200
cs-uri-stem	/blog.html
sc-bytes	9338
```

### You have two files that you want to join together as key value pairs

```
$ cat keys.txt
a
b
c
```

```
$ cat values.txt
1
2
3
```

```
$ zipit --json keys.txt values.txt | jq .
{
  "1": "a"
  "2": "b",
  "3": "c",
}
```


Why?
-------

I was doing some work with a log file and wanted to print the headers + values for a row on the same line, like a set of key-value pairs. 


You're reinventing the wheel!
-------

This is doable in multiple ways using existing tools (e.g. `awk`) but I'm learning Rust so wanted to write a little tool that did this. 

Cheers. 
