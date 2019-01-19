Might be a nice idea to support a flag for 'cycling' the first file, so you can keep repeating the pattern for as many lines that are in file2.

For example

```
$ cat file1.txt
name
type
gender
$ cat file2.txt
fido,dog,m
mittens,cat,f
foofoo,dog,f
$ zipit -d ': ' --cycle <(cat file1.txt) <(cat file2.txt | perl -pe 's/,/\n/g')
name: fido
type: dog
gender: m

name: mittens
type: cat
gender: f

name: foofoo
type: dog
gender: f
```
