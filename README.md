# Discard

Remove from `start` to `end` from a file without causing memory issues.


## Download

Go to [releases](https://github.com/invm/discard/releases), download the latest version for your arch and os and extract it.

## Usage

Assuming you have a file with the following content:

```txt
aaa,bbb,ccc,ddd
```

If you want to remove the content between the `aaa,` and `ccc,` strings, you can use the following command:

```bash
discard ./input -s 'aaa,' -e 'ccc,'
```

Which will result in the following content:

```txt
aaa,ccc,ddd
```

This can be useful to remove unwanted parts of a file, like JSON output from a command:

```json
{
  "a": {
    "b": {
      "c": "long string - like a 1gb long"
    }
  }
}
```

Loading such a file with `jq`, even with a stream parser will cause a huge memory usage. You can discard the `c` field with the following command:

```bash
discard ./input -o ./output -s '\"c\": \"' -e '\"\n'
```

## Limitations

Reading a file by chunks can cause issues with the start and end strings in cases where the chunk ends in the middle of the start or end string.

This can be solved by modifying the chunk size with the `--step` parameter.

## API

Run `discard --help` for more information.

```bash
Read a file and discard everything between the first occurrence of START and the first occurrence of END

Usage: discard [OPTIONS] -s <START> -e <END> <INPUT>

Arguments:
  <INPUT>  Input file

Options:
  -o <OUTPUT>        Output file. If not provided, it will write to stdout
  -s <START>         Start pattern to search for
  -e <END>           End pattern to search for
      --step <STEP>  Step size for reading the file [default: 100000]
  -h, --help         Print help
  -V, --version      Print version```
```
