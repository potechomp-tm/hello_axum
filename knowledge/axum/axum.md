# Axum
はRustのwebフレームワーク
今回作るサービスではrustのwebapiを実装して
その中でjwt認証を組み込む＋アクセストークンがないと叩けないAPIみたいなものを作ってみる感じです。
[流れ]
1. 基本的なAPIを実装（router, handler作ってXXXポートでリスニングするだけ）
   1. helthcheck用endpoint
   2. 認証用endpoint
   3. アクセストークンもとにデータ取得するendpoint（後半のトークンベース認証を考えてからでも良い）
2. cors対策する（任意でよい。今時点では優先度低い。けどたぶんすぐやれる）
3. JWT認証を導入する

# API実装
axum


## router


```

```

# メモ
構文しかり注意点しかり
気になる部分は全部メモって後世に残していくスタイルで良く

## impl IntoResponse
エンドポイント叩いたときのレスポンス（axumのハンドラーの返り値）は、
IntoResponseトレイトを実装した型を返すような書き方をする
（値を返さないようなハンドラーなら別にいいが、普通なら完了メッセージとかを返すので）

通常はStatusCodeとレスポンスボディを内包したタプルを返すような書き方をするが、
struct MyResponse {}; 
impl IntoResponse for MyResponse;みたいな感じでトレイトを実装することで
独自のレスポンスを定義することもできる

```
// パターン①
// これでいいらしい
fn handler() -> impl IntoResponse{
   (StatusCode::OK, Json(json))
}

// パターン②
// こちらの方がそれっぽいけどどうなんだろ。
fn handler() -> Result<impl IntoResponse, StatusCode> {
   (〃)
}

//その他（独自のレスポンス定義して
struct MyResponse {
   status: StatusCode,
   body: Json,
}


impl IntoResponse for MyResponse {
   fn into_response(self) -> Response {
      // 書けなくて泣きそう。。；；（勉強しなさい○げ！）
   }   
}

```