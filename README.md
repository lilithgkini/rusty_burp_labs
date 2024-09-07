This is an attempt to write rust programs to solve various portswigger labs that are very tedious to solve manually.

It serves both as an exercise on my coding and practice for the portswigger academy labs.

# Contents
<u>Labs</u>
[[gift_cards/README|Infinite money logic flaw]]
[[integer_overflow/README|Low-level logic flaw]]
[[SQLi_time/README|Blind SQL injection with time delays and information retrieval]]
[[SQLi_union/README|SQL injection attack, listing the database contents on Oracle]]
[[SQLi_conditonal/README|Blind SQL injection with conditional errors]]

<u>Mystery Lab</u>
[[get_labs/README|Better mystery labs]]

<u>My personal libraries</u>
[[Libraries/README|Libraries]]

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
You can either use the binaries in the "target" directory after you build them or use "cargo run" with the p flag and the name of the desired binary to execute.
```bash
cargo r -p get_labs 
```

or you can "cd" in each directory and use just the typical "cargo run"
```
cd get_labs
cargo r 
```