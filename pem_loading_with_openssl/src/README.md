# PEM loading with OpenSSL

Code for converting PEM-encoded, possibly encrypted, elliptic-curve based
private keys to hex-encoded integers.

## Docker

To build the docker image:

    $ docker build . -t coleslaw

To run the docker image with the repo mounted and drop into bash:

    $ docker run -v $(pwd):/project -it coleslaw bash

## OpenSSL Notes

### Key Generation

    $ openssl ecparam -genkey -name secp256k1 | openssl ec -out ec.key -aes128
              ^^^^^^^               ^^^^^^^^^                   ^^^^^^ ^^^^^^^
              1                     2                           3      4

1. Using EC
2. Name of the curve to use
3. Location and filename to write the key to
4. (optional) Cipher to use to encrypt the key (asks for password)

Sample output:

    -----BEGIN EC PRIVATE KEY-----
    Proc-Type: 4,ENCRYPTED
    DEK-Info: AES-128-CBC,3B7EE8945C8BDC31E75F7B1C3C1E87C7

    IqBMcOzVzSEbfnDKuuIc5khuJbf82tiF7n81dnOPvUl8LyO2hJYun7PtRh9II80K
    AECnnGWCZG3qezjgEdvThAC1xqHEr2Ycf/iEOp8jUlaNsGf1LfAtyuzHCFWyPt2u
    BakgAGzzNpUMcb37kJo5/i8Pru/FUresCZ9zjftHbuE=
    -----END EC PRIVATE KEY-----

### Key Display

    $ openssl ec -text -in ec.key
              ^^ ^^^^^
              1  2

1. Using EC
2. Show the info (if password encrypted, will ask)

Sample output:

    Private-Key: (256 bit)
    priv:
        00:b4:b6:9f:f4:4e:5c:cc:b6:5a:61:60:9e:fc:6a:
        99:c5:95:66:5f:dc:75:59:f3:66:51:e4:f8:27:09:
        93:7e:aa
    pub:
        04:60:75:cc:9c:85:c6:8f:ad:aa:10:79:d5:a2:79:
        80:06:39:de:66:6f:74:c7:a3:32:8a:da:e7:72:b3:
        c7:8a:a0:fe:41:d9:aa:eb:13:66:5e:9d:85:e5:3c:
        b5:66:57:04:d4:26:84:ba:8f:26:12:4c:c4:23:ab:
        f0:23:95:ee:25
    ASN1 OID: secp256k1

## C Program

Compile:

    $ cd src/c/ && make

Then:

    $ ./main ec.key [password]
        ^^^^ ^^^^^^ ^^^^^^^^^^
        1    2      3

1. Name of compiled binary
2. Name of key file output by keygen
3. (optional) Password used to encrypt key

##  Go Program

Compile:

    $ cd src/go/coleslaw/
    $ go generate
    $ go build

Then

    $ ./coleslaw ec.key [password]
        ^^^^ ^^^^^^ ^^^^^^^^^^
        1    2      3

1. Name of compiled binary
2. Name of key file output by keygen
3. (optional) Password used to encrypt key

##  Rust Program

Compile:

    $ cd src/rust/
    $ cargo build && mv ./target/debug/coleslaw ./

Then

    $ ./coleslaw ec.key [password]
        ^^^^ ^^^^^^ ^^^^^^^^^^
        1    2      3

1. Name of compiled binary
2. Name of key file output by keygen
3. (optional) Password used to encrypt key
