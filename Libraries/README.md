"my_request" is a library that serves as a wrapper for the "reqwest" alongside other crates that are necessary for many of these labs.

Instead of reusing code for each binary I wanted to write a library that would take care of all this and in my main i would just call the object and its methods.

The "sqli" library contains useful functions for the SQL labs and is dependent on the my_request library.