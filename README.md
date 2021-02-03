# RedisLTree

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
~~2021.02.11 23:59:59 之前用RUST实现Postgres Ltree到Redis的复刻~~

> **太难了，可能需要半年的时间，**


### 资料
- nom 做lquery语法分析
