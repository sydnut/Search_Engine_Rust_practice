# Search Engine in Rust

一个用 Rust 编写的轻量级全文搜索引擎练习项目。它能够递归索引 XML、XHTML 和 PDF 文档，并通过命令行、网页或 HTTP API 返回相关度最高的 10 个文件。

> 当前阶段的功能已完成，项目暂告一段落。项目现已使用属性宏自动注册 `FileHandler` 实现。

## 已实现功能

- 递归扫描目录并解析 `.xml`、`.xhtml` 和 `.pdf` 文档
- 对英文词元进行小写归一化，并使用 `snowstem` 的 English Snowball stemmer 提取词干
- 统计 TF 与 DF，将索引模型和源目录信息保存为 JSON
- 使用 TF-IDF 计算相关度，返回 Top 10 文件路径及分数
- 提供 CLI 搜索、网页搜索和 `POST /api/search` 接口
- 启动 HTTP 服务时异步检查新增、修改和删除的文件，并将索引更新写回磁盘
- 通过 `FileHandler` trait 解耦文件解析，并使用属性宏与 `inventory` 自动注册扩展名和处理器

## 快速开始

项目使用 Rust 2024 Edition。首先生成索引：

```bash
cargo run -p rs-search -- index <INPUT_DIR> [OUTPUT_FILE]
```

`OUTPUT_FILE` 默认为 `index.json`。生成索引后，可以在命令行中搜索：

```bash
cargo run -p rs-search -- search "OpenGL buffer" ./index.json
```

也可以启动 HTTP 服务：

```bash
cargo run -p rs-search -- serve ./index.json [ADDRESS]
```

`ADDRESS` 默认为 `127.0.0.1:8080`。启动后访问 <http://127.0.0.1:8080> 即可使用搜索页面。

搜索接口接收纯文本请求体，并返回 Top 10 文件路径组成的 JSON 数组：

```bash
curl -X POST --data "OpenGL buffer" http://127.0.0.1:8080/api/search
```

```json
["docs/example-1.xml", "docs/example-2.xml"]
```

## 扩展文件格式

新增文件处理器时，实现 `FileHandler`、提供 `new()` 构造函数，并用属性宏声明它支持的扩展名：

```rust
#[regiser_handler(exts = ["txt"])]
pub(crate) struct TxtFileHandler;
```

属性宏会生成处理器的注册信息；`init_handlers()` 通过 `inventory` 收集这些信息并建立扩展名到处理器的映射，无需再手工修改注册函数。

## 项目结构

```text
crates/
├── app/                         # CLI、TF-IDF 排序、HTTP 服务与搜索页面
└── core/
    ├── src/                     # 文件处理器、分词、索引构建与增量更新
    └── file-register-macro/     # 自动注册文件处理器的属性宏
```

## 学习来源

本项目跟随 [Tsoding 的 Search Engine in Rust 视频系列](https://www.youtube.com/watch?v=hm5xOJiVEeg&list=PLpM-Dvs8t0VZXC-91PpIp-eAt0WF5SKEv) 学习并编写，并在此基础上加入了 CLI 参数、递归目录索引、英文词干提取、Web 搜索、增量索引、支持 XML、XHTML、PDF 的文件处理接口，以及自动注册处理器的属性宏。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
