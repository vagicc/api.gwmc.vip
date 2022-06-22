跟我买车 - API

请先把根目录下的“env”文件复制为“.env”文件，再运行

主要基于rust的warp框架+ORM（rust下唯一的好像）diesel
开发时增删改查数据表时，请看DIESEL.md文件！！！

Git分支说明：
product分支为生产服务器对应;
main分支对应预上线测试; 
test对测试服分支（代码合并到此分支后自自动布署到测试服务器）;

开发流程：
先从“test”测试分支拉代码，并创建个人开发分支
下面创建我个人开发分支“luck”:
# ==============
```bash
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git branch
  main
  product
* test
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git switch -c luck
切换到一个新分支 'luck'
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git push --set-upstream origin luck
总共 0（差异 0），复用 0（差异 0），包复用 0
remote: 
remote: Create a pull request for 'luck' on GitHub by visiting:
remote:      https://github.com/vagicc/api.gwmc.vip/pull/new/luck
remote: 
To github.com:vagicc/api.gwmc.vip.git
 * [new branch]      luck -> luck
分支 'luck' 设置为跟踪来自 'origin' 的远程分支 'luck'。
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git branch
* luck
  main
  product
  test
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git add -A -- /home/luck/Code/Rust/api_gwmc_vip/README.md
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git commit -a -m "开发说明"
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git push
枚举对象: 5, 完成.
对象计数中: 100% (5/5), 完成.
使用 8 个线程进行压缩
压缩对象中: 100% (3/3), 完成.
写入对象中: 100% (3/3), 1.04 KiB | 1.04 MiB/s, 完成.
总共 3（差异 1），复用 0（差异 0），包复用 0
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
To github.com:vagicc/api.gwmc.vip.git
   2620a42..9bd1897  luck -> luck
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git branch 
* luck
  main
  product
  test
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git switch test
切换到分支 'test'
您的分支与上游分支 'origin/test' 一致。
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git pull
已经是最新的。
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git merge luck --squash 
更新 2620a42..52d0b04
Fast-forward
压缩提交 -- 未更新 HEAD
 README.md | 50 ++++++++++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 50 insertions(+)
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git push
Everything up-to-date
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git commit 
[test c006bfa] Squashed commit of the following:
 1 file changed, 50 insertions(+)
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$ git push
枚举对象: 5, 完成.
对象计数中: 100% (5/5), 完成.
使用 8 个线程进行压缩
压缩对象中: 100% (3/3), 完成.
写入对象中: 100% (3/3), 1.38 KiB | 1.38 MiB/s, 完成.
总共 3（差异 1），复用 0（差异 0），包复用 0
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
To github.com:vagicc/api.gwmc.vip.git
   2620a42..c006bfa  test -> test
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
```
# ==========
原则上一个开发人员只能提交一个个人开发分支到线上

---------------------
生产环境运行时重定向日志：
cargo run >> log.txt 2>>error.txt

