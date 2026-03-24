# source

## 部件解析
mod.rs
定义统一源码输入抽象，把不同来源（字节切片、UTF-16 切片、文件）统一成可逐码点读取的接口。

- `ReadChar`
	- 统一读取协议：`next_char(&mut self) -> io::Result<Option<u32>>`
	- 约定返回 Unicode 码点（`u32`），`None` 表示输入结束。
- `Source<'path, R>`
	- 包装具体读取器 `reader: R`，并可选记录文件路径 `path`。
	- 提供多个构造入口：
	  - `from_bytes`：从 UTF-8 字节切片构建。
	  - `from_utf16`：从 UTF-16 码元切片构建。
	  - `from_file`：从文件构建（内部使用 `BufReader<File>` + `UTF8Input`）。

utf8.rs
实现 `UTF8Input<R>`，用于从 `Read` 流读取 UTF-8 并产出码点。

- ASCII 快路径：单字节 `< 0x80` 直接返回。
- 多字节慢路径：按首字节宽度计算并拼接后续 continuation byte。
- 对外仅暴露 `ReadChar` 语义，隐藏字节级读取细节。

utf16.rs
实现 `UTF16Source<'a>`，用于从 UTF-16 码元切片读取码点。

- 普通 BMP 码元：直接返回。
- 高代理项：读取并校验低代理项，组合成补充平面码点。
- 异常处理：
	- 缺失低代理项返回 `UnexpectedEof`
	- 非法代理项组合返回 `InvalidData`

## 当前状态与后续方向
- 已完成：统一输入协议 + UTF-8/UTF-16 两套读取器 + 多入口构造函数。
- 待完善：UTF-8 非法序列严格校验策略、错误信息精细化、与 lexer 错误恢复协同。

## 开发日志
- 3 月 24 日
  - 新增 `source` 模块文档，明确输入抽象边界与接口职责
  - 梳理 `Source` / `ReadChar` / `UTF8Input` / `UTF16Source` 的协作关系
  - 记录当前实现能力与后续完善方向
