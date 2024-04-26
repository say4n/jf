# jf
flatten them json

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
    ]
}
```

into

```json

{
    "this.is.nested.quite.deep.0" : "but",
    "this.is.nested.quite.deep.1" : "that",
    "this.is.nested.quite.deep.2" : "is",
    "this.is.nested.quite.deep.3" : "fine",
    "some.0.other.0": "stuff"
}
```
