## feature:

[x] 为 子命令 自动生成一些 example.
[x] Q&A style repl for rule 资料: https://yexiyue.github.io/dialogue-macro/guide/password.html
[ ] dialogue 交互式问答要求用户输入 path 的时候能提供 completion.
debug_检查子命令示example是否能正确的被解析
## know issue:

[x] 子命令的名称重复问题 [2025-01-30 17:09:54] App::debug_duplicate_names_check() 函数用于检查子命令名称重复的问题.


2025-02-12 

/Users/chenbao/Downloads/cmd_test/target/debug/cmd_test repl '["100","[\"2\",\"3\",\"4\"]","dsfadadsfsdsafasdf","[\"adsf\",\"sfad\",\"sadf\"]","true","sadf","[\"adf\"]","one","[]",""]'

/Users/chenbao/Downloads/cmd_test/target/debug/cmd_test repl $'["100","[\"2\",\"3\",\"4\"]","dsfadadsfsdsafasdf","[\"adsf\",\"sfad\",\"sadf\"]","true","sadf","[\"adf\"]","on'e","[]","' "]'

/Users/chenbao/Downloads/cmd_test/target/debug/cmd_test repl <<< '["100","[\"2\",\"3\",\"4\"]","dsfadadsfsdsafasdf","[\"adsf\",\"sfad\",\"sadf\"]","true","sadf","[\"adf\"]","one","[]",""]'

echo '-e' | /Users/chenbao/Downloads/cmd_test/target/debug/cmd_test repl

/Users/chenbao/Downloads/cmd*test/target/debug/cmd_test << '###mark###'
hello how are you?
" ' \' \
12234 ~!@#$%^&\*()*+{}[];:''"/>><<?
###mark###

/Users/chenbao/Downloads/cmd*test/target/debug/cmd_test aa --stdin << '###mark###'
hello how are you?
" ' \' \
12234 ~!@#$%^&\*()*+{}[];:''"/>><<?
###mark###

方案一:
是用 base64 编码
编码后的 base64 代码用户看不懂,

方案二:
是用 stdin
这个就需要在 repl 里加 flag 了,
app repl --stdin # 这个多出了一个 flag, 这个 flag 还是有参数的...
app repl--stdin # 这个跟 flag 长得太像了.
app repl-stdin # 这个跟 flag 长得也有点像.
app repl*stdin # 这个还行, 就是会给子命令名称添加一个小尾巴.
app repl@stdin # 这个还行, 就是会给子命令名称添加一个小尾巴.
app repl stdin << '###\_mark*###'
text here ###_mark_###
/Users/chenbao/Downloads/cmd*test/target/debug/cmd_test repl stdin << '###\_marker*###'
[
"1324",
"[\"1234\"]",
"1234",
"[\"2134\"]",
"true",
"3124",
"[\"321\"]",
"two",
"[\"one\",\"four\"]",
"vim "
] ###_marker_###


2025-02-13 00:28:25
Executed command 现在输出的是 toml 格式的了,
能从 toml 中解析出需要的 dialog 参数了 😄