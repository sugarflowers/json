# json crate
## about
項目不定のjsonを扱う場合、読み込む場合は簡単だけど値を追加したりするのは簡単ではないので、構造体にマップして利用するのではなく項目不明のまま利用できるようなjson crateにしようと考えている。

## todo:
- とりあえずjsonデータの読み書きや値の出し入れが出来るという機能は実装できたが階層構造のデータに値を入れる場合少し無理がある。構造体の関数の見直しが必要。
  - メソッドチェーンにする。
  - データ取得の戻り値について ```Option<&Value>``` にして関数を共通化する。
