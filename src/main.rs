use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::{read,read_to_string};
use rand::Rng;
use std::env;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {

    let args: Vec<String> = env::args().collect();
    let address;
    if args.len() < 2 {
        println!("WARN:未指定监听地址\n");
        address  = "0.0.0.0:12520";
    } else {
        address  = &args[1];
    }
    //读取配置文件
    let config_path = "config.txt";
    let mut config: Vec<String> = Vec::new();
    read_config(config_path, &mut config);

    // 监听本地端口，等待 TCP 连接的建立
    let listener = match TcpListener::bind(address) {
        Ok(ok) => ok,
        Err(e) =>{
        println!("ERROR:监听地址失败:{}",e);
        panic!()
        },
    };
    
    println!("INFO:监听地址:http://{}\n",address);

    // listener.incoming()送代器将阻塞for循环，直到传入新请求
    for stream in listener.incoming() {
        if let Ok(request) = stream {
            
            let config_clone = config.clone();
            //处理请求
            tokio::spawn(async move {
                handle_request(request,config_clone).await;
            });
        } else if let Err(e) = stream {
            println!("WARN:无法处理请求:{}\n", e);
        };
    }
}

async fn handle_request(mut request: TcpStream,config: Vec<String>) {
    let mut request_data = [0; 1024];
    //读取请求的数据
    if let Err(e) = request.read(&mut request_data) {
        println!("WARN:无效请求:{}\n", e);
        //直接结束这个异步函数的运行
        return;
    };

    let get_path = b"GET / HTTP/1.1\r\n";   //http路径

    let mut response: Vec<u8>;  //这个是存储http响应体的变量，要求Vec<u8>类型
    //判断http路径，若不符合则返回404
    if request_data.starts_with(get_path) {
        //生成随机数
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..config.len() as usize);
        //根据生成的随机数随机选择图片文件路径
        let file_path = &config[random_number];
        //获取到文件后缀名
        let file_suffix: Vec<&str> = file_path.split('.').collect();
        let file_suffix = file_suffix[file_suffix.len() - 1];
        //读取图片文件内容并构造http响应体
        match read(file_path){
            Ok(content) =>{
                response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: image/{}\r\n\
                    Content-Length: {}\r\n\
                    Cache-Control: max-age=3600\r\n\
                    Connection: keep-alive\r\n\r\n",
                    file_suffix,
                    content.len()
                ).into_bytes();
                //最后写入图片文件内容
                response.extend_from_slice(&content);
                // 获取请求的 IP 地址并打印日志
                let peer_addr = match request.peer_addr() {
                    Ok(addr) => addr.ip().to_string(),
                    Err(e) => {
                        println!("WARN:无法获取请求的IP地址:{}\n", e);
                        return;
                    }
                };
                println!("INFO: request-IP:{} response:{}",peer_addr,file_path)
            },
            Err(e) => {
                response = "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string().into_bytes();
                println!("ERROR:读取文件失败:{}",e);
            },
        };
    } else {
        response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string().into_bytes();
    }

    //将回复内容写入连接缓存中
    if let Err(e) = request.write_all(&response) {
        println!("ERROR:写入连接缓存错误:{}\n", e);
        //直接结束这个异步函数的运行
        return;
    }

    //使用 flush 将缓存中的内容发送到客户端
    if let Err(e) = request.flush() {
        println!("ERROR:响应请求错误:{}\n", e);
        return;
    }
}

fn read_config(config_path: &str,config: &mut Vec<String>) {
    //保存路径配置项到数组
    match read_to_string(config_path){
        Ok(content) =>{
            let lines: Vec<&str> = content.split_terminator(if env::consts::OS == "windows" {"\r\n"} else {"\n"}).collect();
            for line in lines {
                config.push(line.to_string());
            }
        },
        Err(e) => {
            println!("ERROR:读取config.txt配置文件失败:{}",e);
            panic!()
        },
    }
}