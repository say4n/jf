# `$ jf`
flatten them json

## usage

grab the latest `$ jf` from the [releases tab](https://github.com/say4n/jf/releases/) for your platform

```
$ jf -f foo.json
{"foo.bar.0":"baz"}

$ jf --filename foo.json
{"foo.bar.0":"baz"}

$ echo '{"foo": {"bar": ["baz"]}}' | jf
{"foo.bar.0":"baz"}
```

## options

```
  -f, --filename <path_to_file.json>
  -s, --separator <SEPARATOR>         [default: .]
  -h, --help                          Print help
  -V, --version                       Print version
```

## intent

handling nested json data is messy, `$ jf` comes to the rescue.

it can turn highly nested json blobs like:

```json
{
    "this": {
        "is": {
            "nested": {
                "quite": {
                    "deep": ["but", "that", "is", "fine"]
                }
            }
        }
    },
    "some" : [
        {
            "other": ["stuff"]
        }
    ],
    "even": {
        "more": "data",
        "boolean": true,
        "number": 2
    }
}
```

into

```json
{
  "even.boolean": true,
  "even.more": "data",
  "even.number": 2,
  "some.0.other.0": "stuff",
  "this.is.nested.quite.deep.0": "but",
  "this.is.nested.quite.deep.1": "that",
  "this.is.nested.quite.deep.2": "is",
  "this.is.nested.quite.deep.3": "fine"
}
```
