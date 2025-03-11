# findDayTest

高可信软件技术课程作业.  这是一个用于验证 `findDay` 程序的项目。该项目由 Rust 编写，并包含生成自差分验证日志、差分验证日志和热力图的功能。

## 项目结构

- `src/`: 源代码目录
  - `main.rs`: 主程序文件
- `resources/`: 资源文件目录
  - `test.out`: 测试可执行文件
- `doc/`: 文档目录
  - `library.typ`: Typst 库文件
  - `report.typ`: Typst 报告文件
  - `report.pdf`: 生成的报告 PDF 文件
  - `heatmap.png`: 生成的热力图
- `output/`: 输出文件目录
  - `self_diff_log.txt`: 自差分验证日志
  - `diff_log.txt`: 差分验证日志
- `Cargo.toml`: Cargo 配置文件
- `Cargo.lock`: Cargo 锁文件
- `README.md`: 项目说明文件

## 使用说明

### 生成自差分验证日志

运行以下命令生成自差分验证日志：

```sh
cargo run -- s
```

生成的日志文件将保存在 `output/self_diff_log.txt` 中。

### 生成差分验证日志

运行以下命令生成差分验证日志：

```sh
cargo run -- e
```

生成的日志文件将保存在 `output/diff_log.txt` 中。

### 生成热力图

运行以下命令生成热力图：

```sh
cargo run -- h
```

生成的热力图文件将保存在 `doc/heatmap.png` 中。

### 打印帮助信息

如果没有提供参数或参数不满足条件，运行以下命令将打印帮助信息：

```sh
cargo run
```

或

```sh
cargo run -- help
```
