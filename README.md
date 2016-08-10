# Caesar Cipher

_Note: this is actually the Vigenère cipher_

This is a toy project to get familiar with rust, and rust tooling.

As such this will never be a "well designed" project, but more of
a "kitchen sink" approach.

## Caveat
Never ever use this for anything close to security purposes.

## Plan/Requirements
* Encrypt/Decrypt plaintext into/from ciphertext
  * Support for multiple encodings
* Unix-like CLI
* Logging
* Complete Documentation
* Test coverage where appropriate

## Current Status

* CLI working with encrypt/decrypt
* Multiple encoding support working, but not exposed to CLI
* Docs are not complete

## Future Goals
* Perform frequency analysis to break ciphertext
* Might make sense to break `shifty` out into a crate
* Process data in streaming I/O, rather than loading it all into memory.
