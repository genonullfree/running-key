# Running Key Cipher

This cipher encrypts and decrypts plain text (ignoring special characters) using running key encryption.

## Usage
```
    running-key [FLAGS] [OPTIONS]

FLAGS:
    -d, --decrypt    Decrypt with running key encryption
    -e, --encrypt    Encrypt with running key encryption
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --key <key>      Key to use for encrypt/decrypt
    -t, --text <text>    Text to encrypt/decrypt
```

## Example

```
$ cargo run -- -e -k "This cipher encrypts and decrypts plain text (ignoring special characters) using running key encryption." -t "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dol"
> Encrypted: EVZWOQEZYDHBNFPHBLAZHWGQEQTVLTEUZNWMMBAIVBXMYOLHIFLOPKBSDOFMIDHIJQAIZXVQCAZEXJEOQICTMLCY

$ cargo run -- -d -k "This cipher encrypts and decrypts plain text (ignoring special characters) using running key encryption." -t "EVZWOQEZYDHBNFPHBLAZHWGQEQTVLTEUZNWMMBAIVBXMYOLHIFLOPKBSDOFMIDHIJQAIZXVQCAZEXJEOQICTMLCY"
> Decrypted: LOREMIPSUMDOLORSITAMETCONSECTETURADIPISCINGELITSEDDOEIUSMODTEMPORINCIDIDUNTUTLABOREETDOL
