# Search Engine in Rust

一个用 Rust 编写的轻量级全文搜索引擎练习项目。它可以递归读取 XML/XHTML 文档、生成本地索引，并通过网页或 HTTP API 返回相关度最高的 10 个文件路径。

> 项目仍在开发中，接口和索引格式可能继续调整。

## 已实现功能

- 递归扫描目录中的 `.xml` 与 `.xhtml` 文件
- 提取文档文本并进行简单分词
- 将词频索引序列化为 JSON 文件
- 使用 TF-IDF 对搜索结果排序
- 提供轻量级 HTTP 服务和搜索页面
- `POST /api/search` 返回 Top 10 文件路径

## 快速开始

需要安装支持 Rust 2024 Edition 的 Rust 工具链。

首先为文档目录生成索引：

```bash
cargo run -p app -- index <文档目录> [索引文件]
```

例如：

```bash
cargo run -p app -- index ./docs ./index.json
```

然后启动搜索服务：

```bash
cargo run -p app -- serve ./index.json [监听地址]
```

监听地址默认为 `127.0.0.1:8080`。启动后访问 <http://127.0.0.1:8080> 即可使用搜索页面。

也可以直接调用 API：

```bash
curl -X POST --data "OpenGL buffer" http://127.0.0.1:8080/api/search
```

响应是一个 JSON 路径数组：

```json
["docs/example-1.xml", "docs/example-2.xhtml"]
```

## 项目结构

```text
crates/
├── app/       # CLI、搜索排序、HTTP 服务与网页
├── core/      # XML 解析、分词与索引构建
└── common/    # 共享代码
```

## 后续计划

- 完成命令行 `search` 子命令
- 改进分词、相关度计算和错误处理
- 增加测试与更完整的 Web 界面
