[![License](https://img.shields.io/badge/license-MIT-green)](https://choosealicense.com/licenses/mit/)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()

# This project for docker registry automatization
## Installation

Install Rust by [rustup](https://rustup.rs/)

## Usage 

```cargo run -- --config <path_to_your_config>``` 

or 

```<path_to_compiled_bin_file> --config <path_to_your_config>```

### config example

```
{
    "dreg_proto_address":"<http(s)://ip_or_name>",
    "dreg_user":"<user>",
    "dreg_pass":"<password\token>",
    "dreg_api_type":"portus"
}



```


## License

[MIT](https://choosealicense.com/licenses/mit/)
