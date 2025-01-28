# 命令行程序命令设计规范:

现在的命令更多的是人类手动在 Terminal 中输入, 此规范主要是为了让人类在手动输入命令行指令时能 更轻松 更愉快 更容易输入正确的指令.

> 如今，尽管许多 CLI 程序主要(甚至专门)是被人类使用，而不是被程序调用，但它们的许多交互设计仍然承载着过去的包袱。 现在是时候摆脱这些历史包袱了: 如果一个命令主要是被人类所使用的，而不是程序，那么它就应该首先以人为本设计。
> from: https://sunbk201.github.io/cli-guidelines-zh/

程序名 命令 参数
```sh
cargo new "./folder/projectName"
```

参数可以省略:
```sh
cargo init
```

只有程序名的情况:

1. 则可以打印帮助信息, 例如:
```sh
cargo
```

也可以直接执行, 例如: 
```sh
ls
```


如果是多个 “参数” 并且每个参数的类型相同:
备注: 很多时候 “参数” 默认被当作 字符串, 还是建议使用 半角双引号 包裹起来.
```sh
creator new "a.txt" "b.txt" "c/" "d.txt" "e/in_e.txt"

```


如果 “参数” 数量非常多, 类型各有不同, 建议使用 “交互式问答” 的形式来让用户能够 愉快的 轻松的 正确的 完成参数的填写, Don't let your user Panic.
方案一:
```sh
git commit --repl
> did you want commit all changed files(y/n)?
y
> input commit message:
新增了某某功能.
> runing command: git api "'command': 'commit', 'flags': [-a, -m='新增了某某功能.']" # for API.

> runing command: git.commit().all().message("新增了某某功能.")

```

 
