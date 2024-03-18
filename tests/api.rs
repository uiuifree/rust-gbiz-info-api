use std::env;
use dotenv::dotenv;
use gbiz_info_api::{GBizInfoApiV1, GBizInfoApiV1HojinQuery, GBizInfoApiV1UpdateInfoQuery};

fn client() -> GBizInfoApiV1 {
    dotenv().ok();

    GBizInfoApiV1::new(env::var("API_TOKEN").unwrap_or_default())
}


#[tokio::test]
async fn hoge() {
    println!("test");
    let client = client();


    let mut query = GBizInfoApiV1UpdateInfoQuery::default();
    query.set_from("20230401")
        .set_to("20230501");

    println!("{:?}", query);
    let res = match client.update_info(&query).await {
        Ok(v) => { v }
        Err(e) => { return assert!(false, "{:?}", e); }
    };
    let infos = match res.hojin_infos {
        None => { return assert!(false); }
        Some(v) => { v }
    };
    assert!(!infos.is_empty());
    for info in infos.into_iter().take(3) {
        println!("{} {}", info.corporate_number(), info.name());

        let mut query = GBizInfoApiV1HojinQuery::default();
        query.name = Some(info.name.unwrap());
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

}