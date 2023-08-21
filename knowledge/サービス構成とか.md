# これは何か
rustでバックエンド開発する際の構成とかを調べていく中で分かったことをまとめていくナレッジ的な何か
あとこれと関係ないけどrustのハンズオン的なものをたくさん作るのいいかもしれない。（ここに書くことではないが）

## axum webサーバーのメインエントリ部分
[必要なもの]
- Router
APIエンドポイントへのアクセスの振り分けをする
ユースケース単位とかで作成し、その中でhttpメソッドの違いも踏まえた方がいいかも
```

let helth_router = Router::new()
    .route("/", get(hc))        // サービスのヘルスチェック
    .route("/db", get(hc_db));  // DBのヘルスチェック

let app = Router::new()
    .nest("/hc", health_router)
    .nest(...)
    .layer(AddExtentionLayer::new(module)); //moduleの状態管理ができるらしい 
```


- handler 


リクエストをうけてそれを裁くロジック部分。<br>
裁く×<br>
捌く〇<br>

```
pub async fn hc() -> impl IntoResponse {
    StatusCode::NO_CONTEXT
}
pub async fn hc_db() -> Result<impl IntoResponse, StatusCode>
```
やはり私にはrustは早すぎるのかもしれない。

## エントリポイント
rustの