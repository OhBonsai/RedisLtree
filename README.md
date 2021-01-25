# RedisTree

Reimplement [Postgres Ltree](https://www.postgresql.org/docs/12/ltree.html) in Redis

## Quick View
```redis
tree.set tree top field1 "value1" filed2 2
tree.set tree top.x field1 "value1 filed2 3
tree.query tree "~  *.Astropy"
tree.query tree "<@ top"
```

## Problems
- lquery的实现
- gist的实现




## 就算是插满起了旗帜，我也不介意再来一个😄
- milestone1: 2020.12.30 23:59:59 之前用RUST实现Postgres Ltree到Redis的复刻

### 严肃学习前的强烈冲动
- Redis缺乏一个树数据结构基础设施，有用Lua脚本实现的，我觉得太LOW!
- Rust语言**不是**一门类C语言，可以拓宽自己对于语言的理解
- 相对于用redisModule实现一个应用，这个项目更加容易上RedisModule列表，同时可以获得多个Star
- 挺Cool的

### 期待的目标
- 采用先github开发流程，一个人也要像一个团队
  - CircleCI自动化测试/部署 
  - Issue Robot进行代码管理 
  - Release包管理
  - 交叉编译
- 努力运营该项目
  - Star数据量能够在redis module排前五
- 为Redis module开发提供完整测试最佳实践
  - 交叉编译打包 
  - 测试数据与测试环境构建
  - 如何更方便的黑盒测试(docker/python)
  
  
### 资料
- nom 做lquery语法分析