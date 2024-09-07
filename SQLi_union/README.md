# [Lab: SQL injection attack, listing the database contents on Oracle](https://portswigger.net/web-security/sql-injection/examining-the-database/lab-listing-database-contents-oracle)

Finds the password for the user Administrator for the specific burp lab and then authenticates as admin, solving it.

Write a payload to adjust your payload to find the password.

Use
```bash
cargo r -- -u https://<LAB-ID>.web-security-academy.net -e "/filter?category=" -p payload
```