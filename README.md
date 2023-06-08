# PNGme
A command line program that lets you encode/decode secret messages in PNG files.

## Characteristics
1. Encode a message with a key into a PNG file.
2. Decode a message with a key that is stored in a PNG file.
3. Remove the message from a PNG file.
4. Print a list of PNG chunks that can be searched for messages.

### Example
```shell
# To encoded
pngme encode image.png RusT "Secret message" "Secret key"

# To decoded
pngme decode image.png RusT "Secret key"

# To delete
pngme remove image.png RusT
```


## References
- [Rust introducction](https://doc.rust-lang.org/book/title-page.html)
- [PNGme rust challenge](https://picklenerd.github.io/pngme_book/introduction.html)
