# [Lab: Blind SQL injection with time delays and information retrieval](https://portswigger.net/web-security/sql-injection/blind/lab-time-delays-info-retrieval)

Finds the password for the user Administrator for the specific burp lab and then authenticates as admin, solving it.

Write a payload-password and payload-length to adjust your payload to find the length of the password and the actual password afterwards.

Use
```bash
cargo r -- -u https://<LAB-ID>.web-security-academy.net -e -e "/filter?category=" -p payload_password.txt -l payload_length.txt
```