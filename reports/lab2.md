# **实现功能**

在 ch3 基础上新增功能。

`mm::memory_set` 新增清空虚拟页段、检查虚拟页段已被 map 方法。

`task::mod::TaskManager` 新增 task 虚拟页段映射、task 虚拟页段映射清空、检测 task 已被 map、获取 task 页表 token 方法。

`syscall::process` 完成 YOUR_JOB。

# **问答题**

# **荣誉准则**

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 同期生 Luv_Ray 同学。提供 xv6 相关资料。
   >
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > sys_get_time 部分参考 rCore作业: https://sjodqtoogh.feishu.cn/docx/PQ4fd6LS9oTAeBxCnU8cQXisn7f
   >

   > Permission 设置 riscv.h: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/riscv.h
   >
3. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
