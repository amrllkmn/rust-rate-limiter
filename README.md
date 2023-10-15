## API Rate Limiter

This is my attempt at building an API rate limiter. This is part of the [Coding Challenges](https://codingchallenges.fyi/) where I'm taking up the challenge to build a rate limiter as a way to learn a new language. For this, I've chosen Rust as I want to keep on building more experience in Rust.

## Stack

- [Axum](https://github.com/tokio-rs/axum)
- [Serde](https://serde.rs/)

## Features

- Two endpoints: `/limited` and `/unlimited`.

## To run

Assumption: You've installed Rust.

Run:

```sh
cargo run
```

## Challenge 1: Token Bucket

The algorithm for token bucket is (taken from the challenge website):

- There is a ‘bucket’ that has capacity for N tokens. Usually this is a bucket per user or IP address.
- Every time period a new token is added to the bucket, if the bucket is full the token is discarded.
- When a request arrives and the bucket contains tokens, the request is handled and a token is removed from the bucket.
- When a request arrives and the bucket is empty, the request is declined.

The idea is to create a bucket (with capacity of 10 tokens) per IP address and increment it at a rate of 1 token per second.

### How I implemented it?

First I created two endpoints: `/api/limited` and `/api/unlimited`.
Then, I created a struct to store the bucket and an id to identify each user per request.
To implement the algorithm, I created a middleware layer for the `/api/limited`. This allowed me to isolate the logic of identifying and checking the user's token bucket from the business logic.
