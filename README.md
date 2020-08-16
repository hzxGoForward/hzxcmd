# hzxcmd
a cmd tool implementation by Rust and clap

- 本程序使用clap实现了简易的一个命令行工具
- 主要子命令有3个：time、exists、cd
- time显示特定时区的时间；exists判定输入的一系列目录是否存在，目录之间用英文逗号隔开；cd用于切换目录
- 在debug文件下，输入 ./hzxcmd.exe 子命令 参数=参数列表形式，示例如下：
- “./hzxcmd.exe time --zone=utc” 显示当前utc时间
- “./hzxcmd.exe time --zone=local” 显示当地时间
- “./hzxcmd.exe time --zone=BJ” 显示北京时间
- “./hzxcmd.exe exists --list=D:\,C:\,E:\” 分别判断D:\ 、C:\、E:\ 是否存在
- “./hzxcmd.exe cd --dir=D:\” 用于切换到D盘，成功会显目录，失败则告知目录不存在 
- 为Time创建的结构体定义了fmt::Display函数
- 为exists子命令的参数使用了迭代器
- 整个程序中用到了所有权转移