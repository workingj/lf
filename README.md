# lf (list files/folders)

![MIT licensed][license-image]

[license-image]: https://img.shields.io/github/license/workingj/dh.svg

A simple tool like ls, to list files and/or folders.

```text
NAME:
    lf - List Files/Folders 1.0.0

DESCRIPTION:
    Lists all files and folders in the current directory

USAGE:
    lf [folder or path] [ -h | -v | -s | -n | -t ] [.file-extension]

OPTIONS:
    folder or path    Lists all entries in the given folder or path. Has to be a subfolder of the current path.
    -h,  --help        Print help information
    -v,  --version     Print version information
    -s,  --size-asc    Sort entries size ascending
    -sd, --size-desc   Sort entries size descending
    -n,  --name-asc    Sort entries name ascending
    -nd, --name-desc   Sort entries name descending
    -t,  --time-asc    Sort entries time asending
    -td, --time-desc   Sort entries time desending
    .file-extension    List only files with given file-extension.
```
