# ast

## 部件解析
position.rs
定义源码位置信息的数据结构，供 lexer / parser / 错误系统共享。

- `Position`
	使用 `(line, column)` 表示可读位置，字段类型为 `NonZeroUsize`，避免出现 0 行或 0 列。
- `LinearPosition`
	使用单一线性偏移表示位置，便于范围计算和快速比较。
- `Span`
	由起止 `Position` 组成，适用于错误信息展示。
- `LinearSpan`
	由起止 `LinearPosition` 组成，适用于词法/语法阶段的快速区间处理。

node.rs
用于定义 AST 节点（表达式、语句、声明等）的核心枚举与结构体。
当前文件已创建，后续将随 parser 开发逐步填充节点定义。

visitor.rs
用于定义遍历 AST 的 Visitor 接口，支持解释执行、代码生成和静态分析等后续阶段。
当前文件已创建，后续将在节点体系稳定后实现访问协议。


## 开发日志
- 3 月 23 日 
  - 创建 ast 模块目录与文件结构
  - 完成 `position.rs` 的位置与范围基础数据结构定义
  - 创建 `node.rs` / `visitor.rs` 文件骨架，待配合 parser 继续实现