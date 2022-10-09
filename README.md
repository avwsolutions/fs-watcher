# fs-watcher Tutorial
Today I finally made some time to play around with Rust. Lately you may have heared a lot of good news around this programming language, such as `Rust` has been accepted to come and being used for Linux kernel development. This means you can expect more Linux kernel features being written in Rust over C, which is the traditional language of choice. This triggered me to dive further into `Rust`.

To learn more I started my journey with visiting the [Rust Programming Language | Getting Started ](https://www.rust-lang.org/learn/get-started) page. For a first glance you may want to visit the [Rust Playground](https://play.rust-lang.org/), but I learn you to setup a Rust development environment locally. Installing Rust is very easy with a tool called `Rustup`, which is used for installation and version management. I also added a prereqs section below, since this is a common issue when `builds fail`. 

Let's get started!

## Setup your Development Environment

### Prereqs for Ubuntu
Below the prereqs for running your build on Ubuntu. Be aware for CentOS you need to install `gcc`.
```
sudo apt-get update
sudo apt-get install build-essential
```
Below the error that you can expect when this prereq is missing.

```
Updating crates.io index
Installing bandwhich v0.6.0
Compiling libc v0.2.66
error: linker `cc` not found
|
= note: No such file or directory (os error 2)

error: aborting due to previous error

error: failed to compile `bandwhich v0.6.0`, intermediate artifacts can be found at `/tmp/cargo-installrqSeTB`

Caused by:
could not compile `libc`.
```

### Installing Rust via Rustup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Update Rust via Rustup

```
rustup update
```

Example output:

```
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: checking for self-updates

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.64.0 (a55dd71d5 2022-09-19)

info: cleaning up downloads & tmp directories
```

### Meet the Build and Package manager called Cargo

When you succesfully installed Rust through Rustup you also have an additional tool available, called `Cargo`. Cargo is a CLI tool that you will use the most during development activities.
For example during build, testing or running your project. Additionally it will help with publishing documentation and [crates.io](https://crates.io) hosted libraries. Awesome isn't it?

Just try it out yourself by running ```cargo --version```. Later you will look furter into building (```cargo build```) and running (```cargo run```) your project.

Can´t wait to learn more about Cargo?, just dive into the [Cargo Book](https://doc.rust-lang.org/cargo/index.html).

### IDE of choice

Nobody is surprised that Rust is already supported in various editors, just pick your favorite choice. During this tutorial I try to stay unopinionated  ;) by using `vim`.

### Initial .gitignore file for Git

During development I found out that the generated `.gitignore` was rather small. Below is a full version, which helps both Linux and Windows users, so it will exclue the `build target`, `debug`stuff, `lock`file and other temporary files.

```
# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb
```

## Start with the Hello World Example

### Creating your first project

You already met Cargo, the tool you can use to initiate our package project space. Just run the following line.

```
cargo new hello-world
```

After running this command you'll notice a new directory called `hello-world` with the following contents. You may also have a skeleton `.gitignore`.
```
.
├── Cargo.toml
└── src
    └── main.rs
```

Take a look into both files. `Cargo.toml` is the manifest file for Rust, which contains a metadata like the name, version and later dependencies. `main.rs` is the place where you can shine and put your awesome code. For now, just take a look and imagine what the output you can expect.

### Try-out our first line

You are now ready to execute your first Rust package. Run the following line to make the magic happen!

```
cargo run
```
You may have expected the output: `Hello, world!`.

Notice that during the `run` your package get`s compiled/build for you!

Just start playing around with the code and try to build ```cargo build``` and run ```cargo run`` the package again.

### Adding some dependencies

Now that you already played around with Rust, you are ready to add a dependency. The dependency which you are going to add is called `ferris-says`, look at the [library page](https://crates.io/crates/ferris-says) to learn more about the features and usage.Do you remember the manifest file called `Cargo.toml`?  Open this file in your favorite editor.

```
vi Cargo.toml
```

and add the following line below [dependencies] and save the file. For thrillseekers (dev only!!!) you may want to use an asterix as version.

```
[dependencies]
ferris-says = "0.2.1"
```

You are now at the moment that Cargo knows about our required dependencies. But how do you define this within our code `main.rs`? Similiar to some other languages you can set `use <library>`.
Just add the following to the `main.rs` and save the file.

```
use ferris_says::say;
```

Take a notice that you only export the `say` function here. As you may know, Rust takes the Lean route by only importing your needs and encourage you to removing unnessary / unused functions.

Now you are ready to download the library and build the package first. Letś try this out and notice for any `warning` messages.

```
cargo build
```

### Complete our small Application with a story

Now that you have imported the function successfully, you still have to integrate and use `Ferris` inside your code, but first start with the `fun facts` about Ferris. Did you know that Ferris is the `unofficial` mascot of the Rust Community. You may learn another term how many Rust programmers call themselves.

You will now start with adding our final code. First of all you will add a standard library for I/O functionality called `std::io`. Here you only use `{stdout, BufWriter}`, which you can lookup at the [module page](https://doc.rust-lang.org/std/io/index.html). Also take notice that standard libraries are not added to the dependencies list as they come by default.

Now replace the `main` function code with the following.

```
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

Take a moment to look into the actual code block and spot places where the imported functions `say`, `stdout` and `BufWriter` are used. Commonly seen keyword is `let`, which helps you to define variables, see the [documentation](https://doc.rust-lang.org/std/keyword.let.html).

Now run your application by doing.

```
cargo run
```

Yes, you may see the following output. This means you succesfully completed these lines of code. You also revealed another term called `Rustaceans`. This is how Rust programmers call themselves, which is a play on the word `crustacean`, which refers to the mascot crab `Ferris`. 

You are now a `Rustacean`, welcome!

```
__________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

## Building our watcher called fs-watcher

### Introduction

Now that you have survived the `get-started` you can jump into your first Rust project. People around me know that I love `observability`and `monitoring`, so my simple goal is to `observe file and folder changes` with a collector layer, generate an `audit event` and make it available in a data source like a `SIEM` such as `Elastic`. For simplicity I chose a local syslog.

Resume everything for this mini project,you want to achieve the following:
- Monitor files or folders changes on both creation, updates and deletes on actual content and/or metadata.
- Write the actual monitor events to a local (or later remote) syslog facility.

To achieve this I have looked over many libraries that are available on [library page](https://crates.io/). In my case I found two interested libraries to use, namely `notify` and `syslog`.

### Developing the collector part for Observing file and folder changes

As said `notify` here really is the first part of your journey. Reading through the [library documentation](https://crates.io/crates/notify) You can find some good practice examples how to tackle this challenge.

As summary
- You need to setup two additional libraries for this, `notify` and `std::path`.
- Setup the logic to accept an input parameter.
- Start the actual `watcher`process.
- Generate an `event` when a change has happened.

Some example code snippet from examples.

```
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
```

After running this piece of code it seems to solve the first part.

```
cargo new fs-watcher
vi Cargo.toml # add the dependency
vi src/main.rs # add the code
cargo build
echo ruby >> /tmp/secret
cargo run /tmp/secret &
touch /tmp/secret
```

After running this piece of code you can conclude this `watcher` triggers on changes and generates an event that can be printed on the `standard output`. This is exactly what you need!

### Forward the Event towards Syslog

Again here you start with looking through the available libraries on [Crates](https://crates.io/). Here you find a library called `syslog`. When you look at the [documentation](https://crates.io/crates/syslog), you also can find some practical examples here. Important to note that you can combine it with a `log` [library](https://crates.io/crates/log).

As summary
- You need to add two additional libraries called `syslog`and `log`.
- Define an object for the syslog message pattern.
- Initiate the connection towards the (local) syslog daemon.
- Have a log method to send the audit warning towards the connected syslog.

Also here you have some interesting code snippet from examples.

```
extern crate syslog;
#[macro_use]
extern crate log;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{SetLoggerError, LevelFilter};

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "myprogram".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info));

    info!("hello world");
}
```

Also here you can give it a try, but notice the three lines at the top mentioning keyword `exern crates`. Let's look if you really need this in your code. First look at the [documentation](https://doc.rust-lang.org/reference/items/extern-crates.html). here you learn that it helps to refer to `external crates`. After reading some [references](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#an-exception) it seems that in most cases it isn't needed anymore, only in in some exceptional cases. In our case `macro_use` you can just mention them in the actual import. 


### Final application code for fs-watcher

After analysing the code, doing some tests and optimising parts. I finally combined everything together.

```
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter, warn};


fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "fs-watcher".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Warn))
        .expect("could not register logger");

    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            // If there is a match execute the logevent function with the event::notify::Event as
            // input 
            Ok(event) => logevent(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn logevent(event:notify::Event) { 
    warn!("change occured: {:?}", event);
}
```

The following things are adjusted.

- You can get rid of the `external crate` lines. You ensured that the actual macro `warn` is added to the `use statement`.
- You can change the severity level from `info` to `warn` level.
- You can ensure that every foundation things are created once in the `main` function, like path argument, syslog params, logger init and starting the watcher.
- You can reuse the `watch`function, but you have to ensure that the `Ok(event)` triggers an audit event.
- Last step you can create an additional `logevent`function that requires a parameter `event:notify::Event`and logs the actual `change occured` at `warn`level.

Just copy over the code and tryout yourself!

```
cargo new fs-watcher
vi Cargo.toml # add the dependency
vi src/main.rs # add the code
cargo build
echo ruby >> /tmp/secret
cargo run /tmp/secret &
touch /tmp/secret
```

If you are interested in the binary, this can be found at the `./target/debug`folder. Just copy over `fs-watcher` and execute it directly from the CLI like `/tmp/fs-watcher /tmp/secret`.

## Conclusions

First of all, this is an execellent exercise, but isn't near the level of a `Pro Rust Master`. Things that I learned here is that Rust is really well-thought language, works well when you like Object Oriented Programming and first lines of code are there within a hour. I really love the community attention and I'm confident that Rust is the way to modernise the Linux Kernel future.

Hopefully you have enjoyed this tutorial and Goodbye Rustacean!
