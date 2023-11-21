# **实现功能**

在 ch4 基础上新增功能。

`TaskControlBlock::spwan` 对增父、子 `TaskControlBlock` 管理，添加任务并吐回 pid。

`TaskControlBlockInner` 新增 优先级和 stride 填充。

`TaskControlBlockInner::current_priority` 设置当前优先级和 stride。

`syscall::process` 完成 YOUR_JOB。

# **问答题**

# **荣誉准则**

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
   
   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
   
   > rcore作业讲解和答疑：https://sjodqtoogh.feishu.cn/docx/PQ4fd6LS9oTAeBxCnU8cQXisn7f

3. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
