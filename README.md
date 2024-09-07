# Overview
This is an attempt to write rust programs to solve various portswigger labs that are very tedious to solve manually.

It serves both as an exercise on my coding and practice for the portswigger academy labs.

# Contents
**Labs**
- [Infinite money logic flaw](gift_cards/README)
- [Low-level logic flaw](integer_overflow/README)
- [Blind SQL injection with tie delays and information retrieval](SQLi_time/README)
- [SQL injection attack, listing the database contents on Oracle](SQLi_union/README)
- [Blind SQL injection with conditional errors](SQLi_conditonal/README)

**Mystery Labs**
- [Better mystery labs](get_labs/README)

**My personal libraries**
- [Libraries](Libraries/README)

# Workspace
Rust compiles all the library dependencies in the binary which results in really large files. 
The solution to this is "Workspaces". 
This ensures that all these binaries that have the same dependencies can share one source.

# Libraries
The "Libraries/my_request" serves as a library that has the class "MyClient" which is a wrapper for the Reqwest library, alongside others.

The "Libraries/sqli" is a library with functions needed for the sql injection labs.

# Notes
Don't forget to use the "proxy" flag at the end of any command if you want to proxy your request through your burp, or any proxy you might be using.

You can also adjust how many requests you want to do concurrently in some of the labs by editing the "concurrency" value in the source code.

# Use
To build run the following command in the root directory.
```bash
cargo build
```

You can either use the binaries in the "target" directory after you build them or use "cargo run" with the p flag and the name of the desired binary to execute.
```bash
cargo r -p get_labs 
```
Or you can "cd" in each directory and use just the typical "cargo run"
```
cd get_labs
cargo r 
```