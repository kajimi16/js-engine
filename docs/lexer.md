# lexer 


## 部件解析
cursor.rs
词法游标，负责把底层字符读取器包装成“可预读、可消费、可追踪位置、可回收源码片段”的统一接口。

- `Cursor<R>`
	- `iter: R`：底层字符流，要求实现 `ReadChar`。
	- `pos: Position`：当前行列位置，初始为 `(1, 1)`。
	- `peeked: [Option<u32>; 4]`：固定长度预读缓冲，当前支持最多 4 个码点窗口。
	- `source_collector: SourceText`：记录已消费码点（以 UTF-16 形式存储），为后续 token 切片与错误上下文做准备。
- `new(iter)`
	初始化游标状态和缓存。
- `peek_char()`
	查看下一个码点但不消费；如果缓冲为空则从 `iter` 拉取并写入 `peeked[0]`。
- `peek_n(n)`
	将预读窗口补齐到 `n`（上限由缓冲长度决定），返回整个预读数组引用。
- `next_char()`
	优先消费预读缓冲，否则直接读取 `iter`；消费后同步写入 `source_collector`，并更新位置信息：
	- `\r\n` 视为单次换行（读取到 `\r` 后会吞掉紧随的 `\n`）。
	- `\n`、`U+2028`、`U+2029` 触发行号 +1、列号重置到 1。
	- 其他码点执行列号 +1。
- `next_if(c)`
	若下一个码点匹配 `c` 则消费并返回 `true`，否则返回 `false`。

token.rs
定义词法阶段的核心数据结构：`Token`、`TokenKind`、`Numeric`。

- `Token`
	保存 `kind`、`span`、`linear_span` 三类信息，用于同时支持人类可读位置（行列）和线性偏移。
- `TokenKind`
	覆盖当前引擎需要识别的主要词法类别，并提供配套关联构造函数，便于扫描器统一创建 Token。
- `Numeric`
	以枚举区分 `Rational`、`Integer`、`BigInt`，并通过 `From` 实现支持数值到词法数值类型的转换。

scanner.rs
扫描器主逻辑文件，负责从源码流中逐字符识别并产出 `Token` 序列。
当前文件已创建，后续将在此实现跳过空白、识别标识符/关键字、数字与字符串字面量等扫描流程。

## 开发日志
- 3 月 23 日 
  - 创建 lexer 模块目录与文件结构
  - 完成 `token.rs` 的 Token 数据结构、数值字面量表示与 `TokenKind` 关联函数
  - 创建 `scanner.rs` 文件骨架，待补充扫描流程实现
- 3 月 24 日
	- 新增 `cursor.rs`，实现词法游标基础能力：`peek_char` / `peek_n` / `next_char` / `next_if`
	- 完成游标位置信息推进逻辑，覆盖普通字符、`CRLF`、`LF`、`LS`、`PS`
	- 接入 `SourceText` 收集已消费码点，为后续 token 文本提取与报错上下文做准备