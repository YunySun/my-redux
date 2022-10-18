use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

// 定义HTTPie的cli的主入口，包含若干子命令

// 下方 ///是注释文档，clap会将其作为cli的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的HTTP方法，目前只支持get/post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get子命令
#[derive(Parser, Debug)]
struct Get {
    /// Http请求的URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post子命令

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// Http请求的URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// Http请求的body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

// 定义一个检查URL是否合法的函数
fn parse_url(s: &str) -> Result<String> {
    // 检查url是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}

/// 命令行中的key=value可以通过parse_kv_pair解析成KvPair结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

// 当实现FromStr trait后，可以通过str.parse()方法将字符串解析成KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split，会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            // 从迭代器中取第一个结果作为key， 迭代器返回Some(T)/None
            // 将其转换成Ok(T)/Err(E)，然后用？处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取得第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 因为为KvPair实现了FromStr，可以直接通过s.parse()得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号和状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的HTTP headerr
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value)
    }
    println!("\n")
}

// 打印服务器返回的HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 application/json 使用pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        // 其他的直接输出
        _ => println!("{}", body),
    }
}

// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// 将服务器返回的content-type解析成mime类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// cargo run post http://m.mcdn.wuzhishuyuan.com/shudanlists.do tag_id=0 num=2 type=0
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    // 为Http客户端添加一些缺省的http头
    headers.insert("X-POWERED-BY", "Rust Httpie".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    // 生成一个Http客户端
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}
