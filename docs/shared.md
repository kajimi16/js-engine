# shaerd


## 部件解析
source_text.rs 
使用 Vec<u16> 储存 source_text。因为 ECMAScript 规范明确规定，JS 字符串在内部是以 UTF-16 编码的序列。
使用 u16（即 Code Unit，码元），以实现 $O(1)$ 的随机访问。

## 开发日志
- 3 月 23 日 实现数据结构和读取码点的方法