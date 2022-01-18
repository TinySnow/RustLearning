# RustLearning

## Introduction/介绍

#### English: 

Here is some examples copied from &lt;Rust By Example> to learn RUST and all the 'Activites' at each page is implemented, which can be found in source code.

#### Chinese：

该仓库的源代码是从《通过例子学 Rust 中文版》复制而来而学习 RUST，并且每个页面下的”动手试一试“均已被实现，可从源代码中找到。

## Examples/凡例

#### English:

All the 'Activities' in the src code will begin with a comment written in <code>/\*\n实现开始\n\*/</code>(,which 
means the implements will start here.) and it also will end with a comment written in <code>/\*\n实现结束\n\*/</code>
(,which is the same meaning above but end here.)

Sometimes, there will be some code like this:

```rust
println!("============ 实现开始 =============");
/* some implement code here. */
println!("============ 实现结束 =============");
```

It means the same as first paragraph and it just purposes to let you know where the implement code works in the console.

#### Chinese：

所有“动手试一试”的实现代码在源码中均会以注释 <code>/\*\n实现开始\n\*/</code> 开始，以注释 <code>/\*\n实现结束\n\*/</code> 结束。

有时，源码中会有如下代码：

```rust
println!("============ 实现开始 =============");
/* 一些实现代码。 */
println!("============ 实现结束 =============");
```

它的意义与第一段所述相同。它旨在于让学习者知道实现代码在哪里开始起作用，并可以在控制台中看到。

## Attention/注意

#### English:

- All the comment is copied from &lt;Rust By Example>(Chinese Simplified version). 

- Package modules -> File file_hierarchy.rs:

    - This project is in file-hierarchical, so we pass this section.

- Package use_crate:

    - File name do NOT follow the official docs' content name, cause the compile command won't work if we do so.
    
    - The compile command is given in each file's comment.(The comment uses Chinese and does NOT provide English version.)
    
    - The compile command is nothing changed but the path, comparing the official doc's command.
    
- Package use_cargo:
    
    - Cause using cargo, this section is not about the code, so we pass.

#### Chinese：

- 所有注释是从《通过例子学 Rust 中文版》中拷贝而来的。

- modules 包 -> file_hierarchy.rs 文件：

    - 该工程已经文件分层，所以跳过此节。

- use_crate 包：

    - 文件命名不遵守官方文档的目录命名，因为按此做法编译命令不会工作。
    
    - 编译命令已在每个文件的注释中给出。（注释使用中文，并且不提供英文版本。）
    
    - 编译命令与官方文档上的相比，除了路径不同，其他没有区别。
    
- use_cargo 包：

    - 因为此节使用 cargo，无关于代码，所以跳过。