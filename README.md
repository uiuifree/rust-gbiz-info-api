
## gBizInfoについて
https://info.gbiz.go.jp/about/index.html から抜粋

### 提供している情報
法人として登記されている約400万社を対象とし、法人番号、法人名、本社所在地に加えて、府省との契約情報、表彰情報等の政府が保有し公開している法人活動情報を本サイトで一括検索、閲覧できます。
法人の中には、行政機関や管理組合等、法人番号が付与されている組織すべてが含まれています。





## WEB APIのトークン取得
gBizのAPIを使用するためには[Web API利用申請](https://info.gbiz.go.jp/hojin/api_registration/form)からWEBトークンを行ってください。


## インストール
```toml
gbiz-info-api="0.1"
```


## 使い方
```rust
async fn main() {
    let client = BizInfoApiV1::new(env::var("API_TOKEN").unwrap_or_default());
    let mut query = GBizInfoApiV1HojinQuery::default();
    query.name = Some("株式会社");
    query.set_limit(10);
    let  res =match  client.hojin(&query).await{
        Ok(v) => {v}
        Err(_) => {return assert!(false)}
    };
    let infos = res.hojin_infos.unwrap_or_default();
    for info in infos{
        println!("└ {}",info.name.unwrap());
    }
}
```