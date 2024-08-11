## send-pushover-notification-in-Rust
This Rust project demonstrates how to send notifications using the Pushover API. The example is built using reqwest for HTTP requests and tokio for asynchronous execution.

# Features

1-Async HTTP Requests: The project utilizes reqwest to send asynchronous HTTP POST requests to the Pushover API.
2-Dynamic Message Priority: Notifications can be sent with different priority levels (normal, high, and emergency). For emergency priority (priority = 2), additional parameters like expire and retry are included automatically.
3-Easy to Customize: The code is easily customizable to suit different Pushover tokens, user keys, and message contents.
