zipit
-------

Takes each line of two input files and prints them to stdout in either tabbed or JSON format.

Includes option to cycle the first file (`LEFT_FILE`) indefinitely.

Example:

```
$ cat file1.txt
name
type
```

```
$ cat file2.txt
mittens
cat
fido
dog
casper
mouse
```
```
$ zipit --cycle file1.txt file2.txt
name mittens
type cat

name fido
type dog

name casper
type mouse
```

Example use cases
-------

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
$ zipit --json --cycle <(xsv headers animals.csv | awk '{print $2}') <(cat animals.csv| tail -n +2 | xsv flatten --no-headers -s '' | awk '{print $2}')
{"name":"fido","type":"dog","gender":"m"}
{"name":"foofoo","type":"dog","gender":"f"}
{"name":"mittens","type":"cat","gender":"f"}
{"name":"casper","type":"mouse","gender":"m"}
```


Why?
-------

I was doing some work with a CSV and wanted to print the headers + values for a row on the same line, like a set of key-value pairs. 


You're reinventing the wheel!
-------

This is doable in multiple ways using existing tools (e.g. `awk`) but I'm learning Rust so wanted to write a little tool that did this
