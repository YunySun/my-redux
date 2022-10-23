/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-23 10:29:04
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-23 10:39:16
 * @Profile: 一个比较废柴的前端开发
 */
fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("src/pb");
    prost_build.compile_protos(&["abi.proto"], &["."]).unwrap();
}

// fn main() -> Result<()> {
//     let mut prost_build = prost_build::Config::new();
//     prost_build.btree_map(&["."]);
//     prost_build.compile_protos(&["src/frontend.proto", "src/backend.proto"], &["src"])?;
//     Ok(())
// }
