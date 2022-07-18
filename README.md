# Demo Risingcloud webtask written in Rust

## Goal

The goal of this demo is to create a risingcloud worker that you can send a request of the format `name: <Your name>`, and get a response
of the format `greeting: "Hello, it's nice to meet you, <Your name>"`

## Setup

To initialize this repo, we just ran `cargo new risingcloud-demo-rs`

Then we edited `Cargo.toml` to use the libraries we need

- `serde`: responsible for  serializing & deserializing (fancy way of saying take our rust types and turn them into json and vice versa)

## Accepting Input for cloud worker

Risingcloud webtasks get their input by reading a file written in their working directory called `request.json`.
So we'll create a type in Rust to model the data we'll get from this file.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IncomingRequest {
    name: String
}
```

Now all we need to do is load this file.

```rust
impl IncomingRequest {
    fn load() -> Self {
        
    }
    pub fn load() -> Result<Self> {
        let mut fp = std::fs::File::options()
            .read(true)
            .open("request.json")?;

        // file_contents is the plain-text file contents of `request.json`
        let mut file_contents = String::new();
        fp.read_to_string(&mut file_contents)?;

        Ok(serde_json::from_str::<Self>(&file_contents)?)
    }
}
```

## Doing some work with the input data

For this project we're just going format a standard greeting with a unique name. See `src/scary_logic.rs` for the implementation of this.

## Communicating the result back

All the worker has to do to communicate the result back to the caller is to save a file called `response.json`. This is largely similar to getting the request, but in reverse!

```rust
#[derive(Serialize, Deserialize)]
pub struct Response {
    greeting: String,
}
```

```rust
impl Response {
    pub fn save(&self) -> Result<()> {
        let mut fp = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("response.json")?;
        let out_file_contents = serde_json::to_string(&self)?;
        write!(fp, "{}", out_file_contents)?;
        Ok(())
    }
}
```

## Putting it all together

All we have to do now is put together our `main.rs` so we do all 3 steps

```rust
fn main() {
    let caller = req::IncomingRequest::load().unwrap();
    let response = scary_logic::Response::from(caller);
    response.save().unwrap();
}
```

## Deploying to risingcloud

see [Official Documentation](https://risingcloud.com/docs/hello-world) for more information
Risingcloud deployment is also very simple
- `risingcloud login`
`- risingcloud init -s risingcloud-demo-rs`
- Edit the file `risingcloud.yaml`
- `risingcloud push`
- `risingcloud build`
- `risingcloud deploy`

The changes to `risingcloud.yaml` are:
from:
	`from: rust:alpine`
deps:
	`deps: cargo install --path .`
run:
	`run: risingcloud-demo-rs`
