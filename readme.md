# JS 引擎

## 目前项目结构
```
js-engine/  
├── src/  
│   ├── lexer/            # 词法分析器：将源码转为 Tokens    
│   │   ├── mod.rs      # 入口  
│   │   ├── token.rs    # 定义 Token 结构和类型 (Kind)  
│   │   └── scanner.rs  # 具体的扫描逻辑  
│   ├── parser/         # 语法分析器：将 Tokens 转为 AST  
│   │   ├── mod.rs  
│   │   ├── expression.rs # 处理表达式优先级 (Pratt Parsing)  
│   │   └── statement.rs  # 处理 if, while, function 等语句  
│   ├── ast/            # 抽象语法树定义  
│   │   ├── node.rs     # 定义所有的 AST 节点类型  
│   │   └── visitor.rs  # 于遍历树的模式  
│   │   └── position.rs # 定义 Span/Position  
│   ├── shared/         # 通用工具  
│   │   └── error.rs    # 统一的报错机制  
│   │   └── source_text.rs    # 临时存放已经解析过的、规范化后的 UTF-16 文本
│   ├── source/         # 统一源码输入抽象（UTF-8 / UTF-16 / 文件）
│   │   ├── mod.rs      # Source / ReadChar 入口与构造器
│   │   ├── utf8.rs     # UTF-8 读取器实现
│   │   └── utf16.rs    # UTF-16 读取器实现
│   └── main.rs         # 引擎入口，协调各个阶段  
├── docs/               # 与 src 一级子目录同名文档存储开发日志
│   ├── ast.md
│   ├── lexer.md
│   ├── shared.md
│   └── source.md
├── tests/              # 集成测试（存放大量的 .js 测试文件）  
└── examples/           # 示例代码  
```

## 项目开发日志

- 3 月 23 日
  - chore: 初步设计项目结构
  - feat: 参考 boa 进行 SourceText 相关设计，具体变动参考 /doc/shared.md
  - feat: 参考 boa 进行 laxer 相关设计，具体变动参考 /doc/lexer.md
- 3 月 24 日
  - feat: 完成 `lexer/cursor.rs` 的基础能力（预读、消费、位置推进、CRLF 处理）
  - docs: 补充 `docs/lexer.md`，细化 cursor 设计与行为说明
  - docs: 新增 `docs/source.md`，梳理 source 模块抽象与 UTF-8/UTF-16 读取流程

## todo
- 定义 Span/Position + 统一 Error 报告
- 写 lexer：identifier/number/string/comment/operator + 单测
- 写 parser：表达式优先级 + block/if/while/for + ASI
- 定义 AST（尽量接近规范结构，后续好映射）
- 先实现执行的 Completion 机制（Return/Throw/Break/Continue）
- 实现 LexicalEnvironment + var/let/const（含 TDZ）
- 实现对象 + 属性描述符 + 原型链
- 实现函数对象 + this 绑定 + 调用/构造
- 实现 ToPrimitive/ToNumber/ToString/SameValue 等抽象操作
- 接入 test262 runner，用失败列表驱动开发