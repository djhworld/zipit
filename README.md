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
$ zipit --json --cycle <(xsv headers animals.csv | awk '{print $2}') <(cat animals.csv| tail -n +2 | xsv flatten --no-headers -s '' | awk '{print $2}')
{"name":"fido","type":"dog","gender":"m"}
{"name":"foofoo","type":"dog","gender":"f"}
{"name":"mittens","type":"cat","gender":"f"}
{"name":"casper","type":"mouse","gender":"m"}
```

### Log File to record

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

Why?
-------

I was doing some work with a CSV and wanted to print the headers + values for a row on the same line, like a set of key-value pairs. 


You're reinventing the wheel!
-------

This is doable in multiple ways using existing tools (e.g. `awk`) but I'm learning Rust so wanted to write a little tool that did this
