/*
 * @Description:
 * @Author: 李昶
 * @Date: 2022-10-25 23:03:36
 * @LastEditors: 李昶
 * @LastEditTime: 2022-10-25 23:28:29
 * @Profile: 一个比较废柴的前端开发
 */
use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT a a1, b, 123, myfunc(b), * FROM data_source WHERE a > b AND c BETWEEN 10 AND 20 ORDER BY a DESC, b LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast)
}
