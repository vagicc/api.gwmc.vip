跟我买车 - API

请先把根目录下的“env”文件复制为“.env”文件，再运行

主要基于rust的warp框架+ORM（rust下唯一的好像）diesel

Git分支说明：
product分支为生产服务器对应;
main分支对应预上线测试; 
test对测试服分支（代码合并到此分支后自自动布署到测试服务器）;

开发流程：
先从“test”测试分支拉代码，并创建个人开发分支
下面创建我个人开发分支“luck”:
# =====================================================================================
`
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
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
luck@HP-ENVY-x360:~/Code/Rust/api_gwmc_vip$
`
# ========================================================================
原则上一个开发人员只能提交一个个人开发分支到线上

