use crate::{get_price_day_tx, get_price_min_tx, get_price_sina, init_ashares};

#[maybe_async::test(feature = "is_sync", async(all(not(feature = "is_sync")), tokio::test))]
async fn test_init_ashares() {
    init_ashares().await;
}
#[maybe_async::test(feature = "is_sync", async(all(not(feature = "is_sync")), tokio::test))]
async fn test_get_price_day_tx() {
    init_ashares().await;
    let code = "sh000001";
    let end_date = Some("2025-01-28");
    let count = 10;
    let frequency = "1d";
    let vec = get_price_day_tx(code, end_date, count, frequency)
        .await
        .expect("TODO: panic message");
    for i in vec {
        println!("{:?}", i);
    }
}
#[maybe_async::test(feature = "is_sync", async(all(not(feature = "is_sync")), tokio::test))]
async fn test_get_price_min_tx() {
    init_ashares().await;
    let code = "sh000001";
    let count = 10;
    let frequency = "1d";

    let stock_data = get_price_min_tx(code, count, frequency).await.unwrap();
    for data in stock_data {
        println!(
            "Time: {:?}, Open: {}, Close: {}, High: {}, Low: {}, Volume: {}",
            data.time, data.open, data.close, data.high, data.low, data.volume
        );
    }
}
#[maybe_async::test(feature = "is_sync", async(all(not(feature = "is_sync")), tokio::test))]
async fn test_get_price_sina() {
    init_ashares().await;
    let code = "sh000001";
    let count = 10;
    let frequency = "1M";

    let stock_data = get_price_sina(code, count, frequency,&[])
        .await
        .unwrap();
    for data in stock_data {
        println!("{:?}",data);
    }
}
