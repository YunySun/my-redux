/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-22 17:18:57
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-24 17:27:43
 * @Profile: 一个比较废柴的前端开发
 */
use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::get,
    AddExtensionLayer, Router,
};
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash, Hasher},
    num::NonZeroUsize,
    sync::Arc,
};

use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{info, instrument};

// 引入protobuf生成的代码
mod pb;

use pb::*;

// 参数使用serde做Deserialize，axum可以自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

// 解析出来的图片处理的参数
// struct ImageSpec {
//     specs: Vec<Spec>,
// }

// // 每个参数是支持的某种方式
// enum Spec {
//     Resize(Resize),
//     Crop(Crop),
// }

// // 处理图片的resize
// struct Resize {
//     width: u32,
//     height: u32,
// }

// message ImageSpec { repeated Spec specs = 1; }

// message Spec {
//     oneof data {
//         Resize resize = 1;
//         Crop crop = 2;
//     }
// }

// fn print_test_url(url: &str) {
//     use std::borrow::Borrow;
//     let spec1 = Spec::new_resize(600, 800, resize::SampleFilter::CatmullRom);
//     let spec2 = Spec::new_watermark(20, 20);
//     let spec3 = Spec::new_filter(filter::Filter::Marine);
//     let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
//     let s: String = image_spec.borrow().into();
//     let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
//     println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
// }

#[tokio::main]
async fn main() {
    // 初始化tracing
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    // 构建路由
    let app = Router::new()
        // Get /image 会执行generate函数，并把spec和url传递过去
        .route("/image/:spec/:url", get(generate));

    // 运行web服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 解析参数
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
