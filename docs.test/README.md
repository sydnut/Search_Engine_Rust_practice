# 测试文档说明

这里有 10 个用于测试索引和搜索的 XML 文件。`science/` 和 `computing/` 是子目录，可用于检查递归遍历。本轮增量索引验证以修改前生成的 `test.json`（含 11 篇文档）作为旧索引。

## 特征关键词

| 搜索词 | 预期匹配的文件 |
| --- | --- |
| `nebula` | `science/astronomy.xml` |
| `climate` | `science/climate.xml` |
| `ocean` | `science/ocean.xml` |
| `orchid` | `science/botany.xml` |
| `rust` | `computing/rust.xml` |
| `database` | `computing/database.xml` |
| `transaction` | `computing/database.xml` |
| `network` | `computing/network.xml` |
| `storage` | `computing/storage.xml` |
| `expedition` | `history.xml` |
| `librarian` | `story.xml` |

## 检查词频和文档频率

- `shared` 出现在 astronomy、climate、ocean、Rust、database、history 和 story 文档中，可检查跨文档的 DF 统计；本轮从 botany 中移除了它。
- `index` 出现在多个文档中。`computing/storage.xml` 的标题、正文和关键词里都有它，可检查 TF 是否正确累计；本轮从 database 中移除了它。
- `orchid` 在 `science/botany.xml` 中多次出现，包括关键词部分，可用于检查单篇文档内的 TF；修改前的 `garden` 已移除。
- 本轮还从 database 中移除了 `queries`，加入多次出现的 `transaction`，并删除了 `manual.xml`，用于验证文档删除和 DF 更新。
- `search`、`document`、`service` 等词也出现在多个文档中，可检查较宽泛的搜索结果。

## XML 解析检查

- XML 元素有多层嵌套，可检查解析器是否提取不同层级的文本。
- 属性中含有正文里没有的干扰词，例如 `decoy`。如果索引器只读取 XML 文本节点，这些属性值不应被索引。
- 文件都比较小，索引可能很快完成。若要观察索引期间的并发搜索，可以复制文件或添加更大的 XML 文档。

这些测试文档均为生成内容，没有从外部下载。
