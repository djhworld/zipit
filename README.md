zipit
-------

Takes each line of two input files and prints them to stdout, separated by a delimiter.

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


Why?
-------

I was doing some work with a CSV and wanted to print the headers + values for a row on the same line, like a set of key-value pairs. 


You're reinventing the wheel!
-------

This is doable in multiple ways using existing tools (e.g. `awk`) but I'm learning Rust so wanted to write a little tool that did this
