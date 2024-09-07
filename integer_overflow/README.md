# [Lab: Low-level logic flaw](https://portswigger.net/web-security/logic-flaws/examples/lab-logic-flaws-low-level)

Adds to the card the l33t jacket enough times to cause an integer overflow and then adds enough products to get back to a positive number and purchases everything.

Unfortunately we cant use concurrency, thats why it uses the function "repeater_serial" and not the concurrent "repeater"...

Use
```bash
cargo r -p integer_overflow -- -u https://<LAB-ID>.web-security-academy.net
```