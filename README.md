# mdBook

专注版[mdbook](https://github.com/rust-lang/mdBook)

专心写文档，忘掉summary。

## 功能

- 添加命令 `mdbook scan`，将扫描文档目录并生成 `SUMMARY.md`
- `build`,`watch`,`serve` 执行时，将前置执行 `scan`
- 自定义章节名称和排序

> 注：生成book时，每个 `.md` 文件作为一个章节出现在导航栏。

## 安装

`cargo install --git https://github.com/oulx/mdbook mdbook`

## 使用

指令一切照旧。

> `mdbook scan` 命令已前置在 `mdbook build`,`mdbook watch`,`mdbook serve` 中，不用特地执行。

> 注：`mdbook watch`, `mdbook serve` 只在命令前执行一次`mdbook scan`，命令执行中发现变动时只会重新加载文件，不会改变 `SUMMARY.md`

自定义章节名称和排序功能，也通过 `.md` 文档内容进行设置，不用关注其他部分。

两个知识点：
1. Markdown 一级标题 `# `，一般位于首行，使用后用于命名文章总标题。
2. Markdown(html) 注释行 `<!-- -->`，单独一行，内容不显示在最终文章中。

### 自定义名称

自定章节名称。

#### 一级标题

若 `.md` 文件存在首行一级标题 `# My Md Title`，则使用`My Md Title`作为文件名

#### 标题注释

若 `.md`文件**前10行**内，存在注释行 `<!-- title=My Comment Title -->`，则使用`My Comment Title`作为文件名。多次设置则以最后一次内容为准。

#### 优先级

**标题注释**为最高优先，若未设置或解析失败，则尝试获取**一级标题**，若无，将取**文件名**(非目录文件将去掉`.md`后缀)作为章节名称。

**优先级: 文档注释 >> 一级标题 >> 文件名**

### 自定义章节排序

若 `.md`文件**前10行**内，存在注释行 `<!-- order=1 -->`，则使用`1`作为文件排序值。多次设置则以最后一次内容为准。相同目录内的文件将按照此排序值进行**升序**排序。

**排序值范围: 0到65535**; 0最前，依次往后，相同排序值的文件将按照扫描顺序排序。

## 声明

仅供学习参考，协议沿用 ***Mozilla Public License v2.0***，详见 [LICENSE](LICENSE)。

↓↓↓↓↓原版README↓↓↓↓↓

---

[![CI Status](https://github.com/rust-lang/mdBook/actions/workflows/main.yml/badge.svg)](https://github.com/rust-lang/mdBook/actions/workflows/main.yml)
[![crates.io](https://img.shields.io/crates/v/mdbook.svg)](https://crates.io/crates/mdbook)
[![LICENSE](https://img.shields.io/github/license/rust-lang/mdBook.svg)](LICENSE)

mdBook is a utility to create modern online books from Markdown files.

Check out the **[User Guide]** for a list of features and installation and usage information.
The User Guide also serves as a demonstration to showcase what a book looks like.

If you are interested in contributing to the development of mdBook, check out the [Contribution Guide].

## License

All the code in this repository is released under the ***Mozilla Public License v2.0***, for more information take a look at the [LICENSE] file.

[User Guide]: https://rust-lang.github.io/mdBook/
[contribution guide]: https://github.com/rust-lang/mdBook/blob/master/CONTRIBUTING.md
[LICENSE]: https://github.com/rust-lang/mdBook/blob/master/LICENSE
