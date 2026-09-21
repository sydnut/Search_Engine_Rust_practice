# Search Engine in Rust

一个用 Rust 编写的轻量级全文搜索引擎练习项目。它可以递归读取 XML/XHTML 文档、生成本地索引，并通过 CLI、网页或 HTTP API 返回相关度最高的 10 个文件路径。

> 项目仍在开发中，接口和索引格式可能继续调整。

## 已实现功能

- 递归扫描目录中的 `.xml` 与 `.xhtml` 文件
- 提取文档文本，并对英文词元进行小写归一化
- 使用 `snowstem` 的 English Snowball stemmer 提取词干，使不同词形能够匹配
- 预先统计 TF 与 DF，并将索引模型保存为 JSON 文件
- 使用 TF-IDF 对结果评分和排序
- 通过 CLI 搜索并显示 Top 10 文件路径及分数
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

可以直接在命令行中搜索：

```bash
cargo run -p app -- search "OpenGL buffer" ./index.json
```

或者启动搜索服务：

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
└── core/      # XML 解析、分词、英文词干提取与索引构建
```

## 后续计划

- 增加停用词过滤，并继续改进相关度计算与错误处理
- 探索更多语言的分词与词形归一化
- 在 Web 搜索结果中展示相关度分数
- 增加测试与更完整的 Web 界面

## 学习来源

本项目跟随 [Tsoding 的 Search Engine in Rust 视频系列](https://www.youtube.com/watch?v=hm5xOJiVEeg&list=PLpM-Dvs8t0VZXC-91PpIp-eAt0WF5SKEv) 学习并编写，同时加入了 CLI 参数、递归目录索引、英文词干提取和 Web 搜索页面等扩展。
