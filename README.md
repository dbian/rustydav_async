# rustydav

> Async  
> Add function: parse_xml to file list, see file.rs  
> Warning: This is deeply customized for my personal need of read Synology file

## Usage


```rust
use webdavc::client;
use webdavc::file::parse_xml;
use webdavc::prelude::*;

async fn scan() -> Result<(), String> {
    let webdav_client = client::Client::init("username", "pwd");
    let base_url = "https://yourwebdav.com";
    let resp = webdav_client
        .list(base_url, "1")
        .await
        .map_err(|e| e.to_string())?;
    let resp = resp
        .text()
        .await
        .map_err(|e| e.to_string())
        .and_then(|x| parse_xml(&x)).and_then(|x|
        // remove first element if any
        if x.len() > 0 {
            Ok(x[1..].to_vec())
        } else {
            Ok(x)
        }
        )?;
    println!("{:?}", resp);
    Ok(())
}

```



Implementation of webdav requests in rust

This is a small library written in rust and inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav) and uses [reqwest](https://github.com/seanmonstar/reqwest) library as the base.

This library can be used to make calls to webdav server.

Supported methods are:
- **get**
- **put**
- **delete**
- **unzip**
- **mkcol**
- **mv**
- **list**

[Changelog](CHANGELOG.md)

# Example
Small example on how to use this library

Include **rustydav** as a dependency
```rust
[dependencies]
rustydav = "0.1.3"
```
Then add this to your code
```rust
extern crate rustydav;

use rustydav::client;
use rustydav::prelude::*;
```
Short examples of call methods
```rust
// Every method will return a Result<Response, Error>

if (result.is_ok() {
    // the method completed with success
} else {
    // somenting when wrong
}

// Create the client
let webdav_client = client::Client::init(/*username*/, /*password*/);

// Get some file from server
// The result will contain the file data
let result = webdav_client.get(/*absolute url to the server file location*/);

// Upload a file to server. It can be any type of file as long as it is transformed to a vector of bytes (Vec<u8>).
// This can be achieved with std::fs::File or zip-rs for sending zip files.
let result = webdav_client.put(/*Vec<u8>*/, /*absolute path to the server file location*/);

// Delete a remote file from the server
let result = webdav_client.delete(/*absolute path to the file on the server*/);

// Unzip a zip archive on the server
let result = webdav_client.unzip(/*absolute path to the zip archive on the server*/);

// Create a new directory on server
let result = webdav_client.mkcol(/*absolute path to the server where to create the new folder*/);

// Rename or move a file / folder / zip on the server
// If the file location changes it will move the file, if only the file name changes it will rename it.
let result = webdav_client.mv(/*absolute path on the server for old file location/name*/, /*absolute on the server for new file location/name*/);

// List files and folders at the given path on the server
// Depth of "0" applies only to the resource, "1" to the resource and it's children, "infinity" to the resource and all it's children recursively
// The result will contain an xml list with the remote folder contents.
let result = webdav_client.list(/*absolute path on the server to list the files*/, /*depth being "0", "1" or "infinity"*/);
```
For some description about them please see the [**client.rs**](src/client.rs) file.
