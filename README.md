# WebRusher
The program fetches HTML content from a specified URL, parses it, and extracts data based on a CSS selector. The example targets the Rust by Example documentation to showcase how to retrieve menu titles from the provided webpage. The code is asynchronous, utilizing the tokio runtime for handling asynchronous tasks.

### Dependencies 
- reqwest = "0.11.23"
- scraper = "0.18.1"
- tokio = { version = "1", features = ["full"] }
