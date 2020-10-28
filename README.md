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

### warp路由

一定要先写playground再or post，`graphql_post.or(graphql_playground)`这种写法连playground页面都进不了

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