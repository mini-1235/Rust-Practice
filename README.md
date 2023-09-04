# Hw1 myfind

This is a simple implementation of the find command. It supports the following options:

- –v/--verbose 参数输出所有遍历到的⽂件
- 同时⽀持匹配多个正则
- 给输出结果去重排序
- tracing 库打⽇志
- ⽀持命令⾏彩⾊输出

and some more common features of find

### Build

```bash
cd myfind
Cargo build
./target/debug/myfind [-v|--verbose] <target directory> [-p<pattern_count>] <pattern1> <pattern2> ...

```

For example

```
./target/debug/myfind    ~/Desktop/quick-demo  -p3 main 1 doc
```

without verbose, three regex expressions
![without verbose](https://i.imgur.com/ch4r9MR.png)

with verbose, three regex expressions

![pGVJG4A](https://i.imgur.com/pGVJG4A.png)

### Warning

This tool is a practice of Rust, and it is not intended to be used in production.
