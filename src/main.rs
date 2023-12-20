use std::fs;
use std::path::Path;

struct Module {
  name: String,
  content: String,
}

fn main() {
    println!("Hello, raiopack!");
    
    // 1. 读取入口文件内容
    let entry_file_content = read_file("src/main.js");

    // 2. 解析模块
    let modules = parse_modules(&entry_file_content);

    // 3. 打印模块内容
    for module in modules {
        println!("Module: {}\n{}", module.name, module.content);
    }
}


// 辅助函数：读取文件内容
fn read_file(file_path: &str) -> String {
  let path = Path::new(file_path);
  match fs::read_to_string(path) {
      Ok(content) => content,
      Err(_) => panic!("Failed to read file: {}", file_path),
  }
}

// 辅助函数：解析模块
fn parse_modules(content: &str) -> Vec<Module> {
  // 在这里实现模块解析逻辑
  // 可以使用正则表达式或其他方式查找导入语句并提取模块内容
  // 这里为了简化，我们直接假设一个模块就是一个 JavaScript 文件
  vec![Module {
      name: "main".to_string(),
      content: content.to_string(),
  }]
}
