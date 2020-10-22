# GraphQL笔记

GraphQL isn't tied(绑定) to any specific database or store engine

## Schema

想用graphql就必须要有一个全局变量/单例模式Schema(例如用OnceCell wrap Schema)

而Schema由3个部分组成，其中Query是required的，而Mutation和Subscription可以为空，例如async_graphql::EmptySubscription

Query部分可以理解为CRUD的R，Mutation则包括对数据的增删改，Subscription一般通过WebSocket实现数据发生变动后推送给订阅的客户端(LiveData)

use async_graphql::Object之后就会包含Object和field两个过程宏(filed过程宏在2.0版本已被删除?，改名为#[graphql])

---

# English words

- criminal and civil: 刑事和民事
- handicapped: 残疾地
- scams: 骗局
- worshippers: 信徒
- threat: 恐吓