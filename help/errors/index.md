# Errors using thiserror

For errors we use the crate `thiserror` because it is well-maintained, pragmatic, and provides detailed errors suitable for libraries. 

We removed the crate `error-chain` because it wasn't well-maintained.


## enum Error

Our convention for error definition looks like this:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    …
}
```
The enum is named `Error`, not anything else. A side effect of this convention is that in each file, the file's enum `Error` convention supersedes any other `Error` such as `std::error::Error`, i.e. in each file, any other `Error` must be fully qualified.


### Error variant for a trivial error

Example of a trivial error with only one field that doesn't benefit from a field name:

```rust
#[error("AlfaBravo ➡ {0:?}")]
AlfaBravo(String),
```

Our convention: the error message starts with the enum variant name, space, Unicode right arrow, space, then any data such a debug string.


### Error variant for a typical error

Example of a more-complex typical error that uses multiple fields and that benefits from field names:

```rust
#[error("CharlieDelta ➡ echo: {echo:?}, foxtrot: {foxtrot:?}")]
CharlieDelta {
    echo: String,
    foxtrot: String,
}
```

Our convention: the error message starts as usual, then each field prints it field name and debug data message.


### Error variant for a wrapper error with #[from] and #[source]

Example of a wrapper error that uses multiple fields and that benefits from field names:

```rust
Io(#[from] std::io::Error),
```

The #[from] attribute implements a From trait for the field.

* The variant using #[from] must not contain any other fields beyond the source error (and possibly a backtrace — see below). 
 
* Usually #[from] fields are unnamed, but #[from] is allowed on a named field too.

The #[from] attribute implies the #[source] attribute, which implements the source() method:

* The source() method identifies the underlying lower level error that caused your error.

* The source() method returns whichever field has a #[from] attribute or #[source] attribute or is named "source".


### Our convention

Our convention for `pub enum Error`:


* The error message for a struct includes each field and its debug representation, so long as it's practical, i.e. we want to behave akin to a library error, and we want print as much information as possible for developers who are debugging.


## Catch errors

Our preferred pattern for handling a `Result` uses `map_or_else` to return the error `err` or unwrap the value `val` like this:

```rust
foo().map_or_else(
    |err| MyError::StdIoError(err),
    |val| val
)?;
```

### Catch errors at end of function

Catch errors at the tail of a function:

```rust
pub fn foo() -> Result<T, Error> {
    whatever().map_or_else(
    |err| Err(Error::Foo(err)),
    |x| Ok(x)
    )
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Foo ➡ {0:?}")]
    Foo(err),
}
```

### Catch errors with #[from] and #[source]

The #[from] attribute always implies that the same field is #[source], so you don't ever need to specify both attributes.

The Error trait's source() method is implemented to return whichever field has a #[source] attribute or is named source, if any. This is for identifying the underlying lower level error that caused your error.


Example:

```rust
fn init() -> Result<(), Error> {
    …
    let config: Config = crate::app::confy::config()?;
    …
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Confy ➡ {0:?}")]
    Confy(#[from] ::confy::ConfyError)

}
```

### Catch errors in main

Our convention is a file `main.rs` where the setup happens:

```rust
fn main() {
    env_logger::init();
    match crate::app::run::run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
```

Our convention is a file `run.rs` where the work happens:

```rust
pub(crate) fn run() -> Result<()> {
    …
    Ok(())
}
```


## Wrap errors

Say you have a file `alfa.rs` with any typical function that can return an error such as:

```rust
pub fn positive(x: i8) -> Result<(), Error> {
    if x > 0 {
        return x
    } else {
        Error::Parma(x)
    }
}
    
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parma ➡ {0:?}")]
    Param(i8),
}
```

Say you also have a file `bravo.rs` that calls `alfa.rs` such as:

```rust
pub fn f(x: i8) -> Result<(), Error> {
    crate::alfa::positive(3)
    .map_or_else(
        |err| Error::Wrap(err),
        |val| val.ok()
    )?
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Wrap ➡ {0:?}")]
    Wrap(crate::alfa::Error)
}
```
