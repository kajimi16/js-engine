# lexer 


## 部件解析
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