# Better RNG for Mystery Labs

The default implementation of the mystery labs portswigger has ends up producing way too many duplicates labs and reduces the chances to get some new lab or new category.

This program reads from a text file and makes sure to always give you a mystery lab that you haven't tried yet, from a new category. This ensures no duplicates.

use the "scrape" sup-command to create a list with all the categories and then keep reusing this file with the "mystery" flag to get a new lab based on the categories you haven't done so far in that file. 

Use

To generate a list
```bash
cargo r -- scrape -u https://portswigger.net -e '/web-security/mystery-lab-challenge' -f categories.txt
```

And then using the list, get a random mystery lab.
```bash
cargo r -- mystery -f categories.txt
```

Or if you dont want to see the link (no spoilers) and wish to place it straight to your clipboard pipe it to your clipboard manager, for example if you are using xclip:
```bash
cargo r -- mystery -f categories |xclip -i -select c
```