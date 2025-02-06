# HTTP Random Pictures API

这是一个使用 Rust 编写的简单随机图片 HTTP api 接口，使用tokio库实现了高性能，它会随机返回配置文件中指定的图片。

## 依赖库

 - std::io::prelude::*
 - std::net::TcpListener
 - std::net::TcpStream
 - std::fs::{read,read_to_string}
 - rand::Rng
 - std::env
 - tokio

## 配置

启动参数1为http监听地址：

示例：`./rust_http_random_pictures_api 0.0.0.0:12520`

`config.txt` 包含图片路径项：

配置文件的每一行是图片文件的路径。

[查看配置文件示例](https://github.com/WHQ12520/rust_http_random_pictures_api/blob/master/config.txt)

## 从源代码运行项目

1. 克隆项目到本地：

```sh
git clone https://github.com/WHQ12520/rust_http_random_pictures_api.git
cd rust_http_random_pictures_api
cargo build
cargo run
## 编译源代码
cargo build --release
