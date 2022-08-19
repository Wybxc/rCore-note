# rCore 学习笔记

此项目是按照 rCore 编写的学习用操作系统，添加了注释作学习笔记之用。

## 运行

需要将 `qemu-system-riscv64` 加入 PATH 中。

```
cd os
make run
```

## 调试

需要将 `riscv64-unknown-elf-gdb` 加入 PATH 中。

```
cd os
make debug
```

### gdb 调试方法

运行 `make debug` 之后，会打开 tmux 终端复用，左侧为 `qemu`，右侧为 `gdb`。

调试开始时 qemu 会在执行第一条指令（0x1000地址）之前暂停。需要首先打一个断点，然后继续执行，以进入操作系统的代码。

例如：

```
b rust_main
c
```

GDB 常用的命令有：

- `c`：继续执行
- `n`：单步跳过
- `s`：单步进入
- `u <line>`：执行到指定行
- `ni`：汇编单步跳过
- `si`：汇编单步进入
- `b <name>`：在指定函数名的地方添加断点
- `b *<address>`：在指定地址的地方添加断点
- `b <line>`：在当前文件指定行的地方添加断点
- `i r`：查看寄存器
- `i b`：查看断点
- `i line`：查看当前执行位置
- `p <expr>`：查看表达式的值
- `p /x <expr>`：查看表达式的值，以十六进制显示
- `d <id>`：删除指定断点

### VSCode GDB 调试

借助 `Native Debug` 插件，实现 VSCode 与 GDB 集成。

运行 `make gdbserver` 打开 gdb 服务端，然后在 VSCode 中启动调试任务即可。

### 自行编译支持 risc-v 的 GDB

编译方法参考 rCore 的旧版文档：

> 1. 安装依赖（针对 Linux，macOS 可以遇到错误再去搜索）
>    * python 并非必须
>    * 在 `Ubuntu 20.04` 等系统中，`python` 和 `python-dev` 需要替换成 `python2` 和 `python2-dev`
>      ```
>      sudo apt-get install libncurses5-dev python python-dev texinfo libreadline-dev
>      ```
> 2. 前往[清华镜像](https://mirrors.tuna.tsinghua.edu.cn/gnu/gdb/?C=M&O=D)下载最新的 GDB 源代码
> 3. 解压源代码，并定位到目录
> 4. 执行以下命令
>    * `--prefix` 是安装的路径，按照以上指令会安装到 `/usr/local/bin/` 下
>    * `--with-python` 是 `python2` 的地址，它和 `--enable-tui` 都是为了支持后续安装一个可视化插件，并非必须
>      ```
>      mkdir build
>      cd build
>      ../configure --prefix=/usr/local --with-python=/usr/bin/python --target=riscv64-unknown-elf --enable-tui=yes
>      ```
> 5. 编译安装
>    ```
>    # Linux
>    make -j$(nproc)
>    # macOS
>    make -j$(sysctl -n hw.ncpu)
>    
>    sudo make install
>    ```
> 6. （可选）安装 [`gdb-dashboard`](https://github.com/cyrus-and/gdb-dashboard/) 插件，优化 debug 体验
>    ```
>    wget -P ~ https://git.io/.gdbinit
>    ```

编译安装完成后，`riscv64-unknown-elf-gdb` 将会被安装到 `/usr/local/bin/` 下。
