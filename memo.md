r2d2 で connection pooling を使用する時。
デフォルトの pool 数は10個らしい。要確認。
なので、actix-web の場合、起動時にアプリケーションが4スレッド立ち上がるので、4 * 10個のセッションがDBに接続される。

なので油断しているとDB側の max_connection 数がオーバーしてしまうなんてこともある。
https://www.letitride.jp/entry/2020/12/19/183524

---