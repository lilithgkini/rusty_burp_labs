# [Lab: Infinite money logic flaw](https://portswigger.net/web-security/logic-flaws/examples/lab-logic-flaws-infinite-money)

Buys all the gift cards in the lab with the 30% off coupon and after enough iterations it purchases the l33t jacket.

Unfortunately we cant use concurrency cause this lab lists used coupons and we use a build-in logic to skip when it finds a used coupon. If it were to do this concurrently it could skip too soon and not redeem some legitimate coupons...

Use
```bash
cargo r -p gift_cards -- -u https://<LAB-ID>.web-security-academy.net
```