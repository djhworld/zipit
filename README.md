zipit
-------

Takes each line of two input files and prints them to stdout in either tabbed or JSON format.

Includes option to cycle the first file (`LEFT_FILE`) indefinitely.

Example:

```
$ cat file1.txt
foo
bar
baz
$ cat file2.txt
do
re
me
$ zipit file1.txt file2.txt
foo do
bar re
baz me
```

Example use cases
-------

You have a CSV and you want to convert it to a bunch of JSON-lines.

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
