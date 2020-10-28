# GraphQL笔记

GraphQL isn't tied(绑定) to any specific database or store engine

## schema简介

想用graphql就必须要有一个全局变量/单例模式Schema(例如用OnceCell wrap Schema)

而Schema由3个部分组成，其中Query是required的，而Mutation和Subscription可以为空，例如async_graphql::EmptySubscription

Query部分可以理解为CRUD的R，Mutation则包括对数据的增删改，Subscription一般通过WebSocket实现数据发生变动后推送给订阅的客户端(LiveData)

use async_graphql::Object之后就会包含Object和field两个过程宏(filed过程宏在2.0版本已被删除?，改名为#[graphql])

## types

grapqhl basic scalar types: String, Int(i32), Float(f32), Boolean, and ID

类型后接个叹号表示NonNull，所以如果下面例子中的value是Optional，则类型Float后面不会接上叹号

```
type ValueAndChangeRate {
  value: Float!
  changeRate: Float!
}
```

## Graphql通信

Request和Response的消息格式都是: Content-Type: application/json

Request会用POST请求将json数据携带在payload中: `{query: "{hello}", variables: null}`

### warp graphql路由

一定要先写playground再or post，`graphql_post.or(graphql_playground)`这种写法连playground页面都进不了

## Graphql携带变量

> curl -X POST -H "Content-Type: application/json" -d '{"query": "query($a: Int!){add(a:$a,b:1)}", "variables": {"a": 1}}' http://localhost:8003

如果没有变量，可以将`query{add(a:1,b:1)}`省去外层的query(因为默认就当做是query?)

如果携带了变量，一定要在query内定义好变量名和类型，例如`query{add()}``

前端payload如果定义了变量但是没有使用就会报错variables is not used，如果没有定义变量但是payload的variables字段多传了变量参数则没有问题

---

## English words

### English(graphql)

- exclamation point: 感叹号

### English(other)

- criminal and civil: 刑事和民事
- handicapped: 残疾地
- scams: 骗局
- worshippers: 信徒
- threat: 恐吓