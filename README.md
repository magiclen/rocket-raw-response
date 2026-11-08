Raw Response for Rocket Framework
====================

[![CI](https://github.com/magiclen/rocket-raw-response/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/rocket-raw-response/actions/workflows/ci.yml)

This crate provides a response struct used for responding raw data.

```rust
use rocket::get;
use rocket_raw_response::RawResponse;

#[get("/")]
fn index() -> RawResponse {
    RawResponse::from_vec(b"Hello world!".to_vec()).file_name("hello.txt")
}
```

The media type is guessed from the extension of the file name, but it can also be set explicitly.

```rust
use rocket::get;
use rocket_raw_response::{RawResponse, mime};

#[get("/")]
fn index() -> RawResponse {
    RawResponse::from_vec(b"a,b\n1,2\n".to_vec())
        .file_name("report.csv")
        .content_type(mime::TEXT_CSV)
}
```

A file on disk is opened asynchronously, and both the file name and the media type default to what the path says.

```rust
use std::io;

use rocket::get;
use rocket_raw_response::RawResponse;

#[get("/")]
async fn index() -> io::Result<RawResponse> {
    RawResponse::from_file("images/image.jpg").await
}
```

A reader is sent as it is.
Its length can be given so that a `Content-Length` header is sent along with it.

```rust
use std::io::Cursor;

use rocket::get;
use rocket_raw_response::RawResponse;

#[get("/")]
fn index() -> RawResponse {
    RawResponse::from_reader_sized(Cursor::new(b"Hello world!"), 12)
        .file_name("hello.txt")
}
```

An uploaded file can be sent straight back.
Rocket drops the extension from `TempFile::name`, so this crate uses the original file name to recover it and falls back to the upload's media type.

```rust
use std::io;

use rocket::{form::Form, fs::TempFile, post};
use rocket_raw_response::RawResponsePro;

#[post("/", data = "<file>")]
async fn echo(file: Form<TempFile<'_>>) -> io::Result<RawResponsePro<'_>> {
    RawResponsePro::from_temp_file(file.into_inner()).await
}
```

`attachment` asks the client to save the body instead of displaying it.

```rust
use rocket::get;
use rocket_raw_response::RawResponse;

#[get("/")]
fn index() -> RawResponse {
    RawResponse::from_vec(b"Hello world!".to_vec())
        .file_name("hello.txt")
        .attachment()
}
```

## Crates.io

https://crates.io/crates/rocket-raw-response

## Documentation

https://docs.rs/rocket-raw-response

## License

[MIT](LICENSE)
