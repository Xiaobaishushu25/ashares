mod ashares_test;

use chrono::{NaiveDate, NaiveDateTime};
use log::debug;
use maybe_async::{async_impl, maybe_async, sync_impl};
use reqwest::Client;
use serde_json::Value;
use std::sync::OnceLock;

#[derive(Debug)]
pub enum StockTime {
    DateTime(NaiveDateTime),
    Date(NaiveDate),
}
/// 该结构体表示x日均线数据和对应的成交量
#[derive(Debug)]
pub struct MovingAverageData {
    pub name: i32,       //几日均线
    pub value: f64,       // 均线值
    pub volume: f64,      // 对应的成交量
}
/// 表示股票数据的结构体，包含股票在特定时间点的基本信息和可选的均线数据。
///
/// # 字段说明
/// - `time`: 表示股票数据的时间点，使用 [`crate::StockTime`] 类型。
/// - `open`: 表示股票在该时间点的开盘价。
/// - `close`: 表示股票在该时间点的收盘价。
/// - `high`: 表示股票在该时间点的最高价。
/// - `low`: 表示股票在该时间点的最低价。
/// - `volume`: 表示股票在该时间点的成交量。
/// - `moving_averages`: 可选字段，表示股票在该时间点的均线数据，存储为 Vec<[`crate::MovingAverageData`]> 类型。
///
/// # 注意事项
/// - `moving_averages` 字段是可选的，表示在某些情况下可能没有均线数据。
/// - 所有价格字段（`open`, `close`, `high`, `low`）都是 `f64` 类型，表示股票价格。
/// - `volume` 字段也是 `f64` 类型，表示成交量。
///
/// # 相关类型
/// - `StockTime`: 用于表示股票数据的时间点。
/// - `MovingAverageData`: 用于表示均线数据。
#[derive(Debug)]
pub struct StockData {
    pub time: StockTime,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub moving_averages: Option<Vec<MovingAverageData>>, // 可选的均线数据
}

#[async_impl]
static CLIENT: OnceLock<Client> = OnceLock::new();
#[sync_impl]
static CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();
/// 初始化同步的股票数据客户端。
///
/// 此函数创建一个同步的 HTTP 客户端，并将其存储在全局的 `OnceLock` 中。
/// 该客户端用于后续的股票数据请求。
#[sync_impl]
pub fn init_ashares() {
    let client = reqwest::blocking::Client::new();
    CLIENT
        .set(client)
        .expect("Failed to initialize sync client");
}
/// 初始化异步的股票数据客户端。
///
/// 此函数创建一个异步的 HTTP 客户端，并将其存储在全局的 `OnceLock` 中。
/// 该客户端用于后续的股票数据请求。
#[async_impl]
pub async fn init_ashares() {
    let client = reqwest::Client::new();
    CLIENT
        .set(client)
        .expect("Failed to initialize async client");
}
/// 获取指定股票代码在指定日期范围内的日线、周线或月线数据。
/// 特点：可以指定截止日期，往前获取指定条数的历史数据。
///
/// # 参数
/// - `code`: 股票代码，例如 "sh600000"。
/// - `end_date`: 结束日期，格式为 "YYYY-MM-DD"。如果为 `None` 则与当前日期相同。
/// - `count`: 请求的数据条数。
/// - `frequency`: 数据频率，支持 "1d"（日线）、"1w"（周线）、"1M"（月线，这里的M大写是为了和分钟m区分）。
///
/// # 返回值
/// - `Result<Vec<StockData>, Box<dyn std::error::Error>>`: 返回一个包含 `StockData` 结构体的向量，或一个错误。
///
/// # 示例
/// ```rust
/// let stock_data = get_price_day_tx("sh600000", Some("2023-10-01"), 100, "1d").await?;
/// ```
#[maybe_async]
pub async fn get_price_day_tx(
    code: &str,
    end_date: Option<&str>,
    count: usize,
    frequency: &str,
) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    let unit = match frequency {
        "1w" => "week",
        "1M" => "month",
        _ => "day",
    };
    //如果 end_date 为 None 或与当前日期相同，则设置为空字符串。
    let end_date = match end_date {
        Some(date) => {
            if date == chrono::Local::now().format("%Y-%m-%d").to_string() {
                ""
            } else {
                date
            }
        }
        None => "",
    };

    let url = format!(
        "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={},{},,{},{},qfq",
        code, unit, end_date, count
    );
    // println!("{}", url);
    let response = CLIENT
        .get()
        .expect("client is not be initialized")
        .get(&url)
        .send()
        .await?
        .text()
        .await
        .unwrap();
    let json: Value = serde_json::from_str(&response)?;

    let data = &json["data"][code];
    let buf = if data.get(format!("qfq{}", unit)).is_some() {
        data[format!("qfq{}", unit)].as_array().unwrap()
    } else {
        data[unit].as_array().unwrap()
    };
    let mut result = Vec::new();
    for item in buf {
        let time = NaiveDate::parse_from_str(item[0].as_str().unwrap(), "%Y-%m-%d")?;
        let open = item[1].as_str().unwrap().parse::<f64>()?;
        let close = item[2].as_str().unwrap().parse::<f64>()?;
        let high = item[3].as_str().unwrap().parse::<f64>()?;
        let low = item[4].as_str().unwrap().parse::<f64>()?;
        let volume = item[5].as_str().unwrap().parse::<f64>()?;
        result.push(StockData {
            time: StockTime::Date(time),
            open,
            close,
            high,
            low,
            volume,
            moving_averages: None,
        });
    }
    Ok(result)
}
/// 获取指定股票代码在指定日期范围内的分钟线数据。
///
/// # 参数
/// - `code`: 股票代码，例如 "sh600000"。
/// - `count`: 请求的数据条数。
/// - `frequency`: 数据频率，可用'1m','5m','15m','30m','60m'。
///
/// # 返回值
/// - `Result<Vec<StockData>, Box<dyn std::error::Error>>`: 返回一个包含 `StockData` 结构体的向量，或一个错误。
///
/// # 示例
/// ```
/// let stock_data = get_price_min_tx("sh600000", Some("2023-10-01"), 100, "5m").await?;
/// ```
#[maybe_async]
pub async fn get_price_min_tx(
    code: &str,
    count: usize,
    frequency: &str,
) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    // 解析K线周期数
    let ts = if frequency.chars().last().unwrap().is_ascii_digit() {
        frequency[..frequency.len() - 1].parse::<usize>()?
    } else {
        1
    };
    let url = format!(
        "https://ifzq.gtimg.cn/appstock/app/kline/mkline?param={},m{},,{}",
        code, ts, count
    );
    debug!("{}", url);
    let response = CLIENT
        .get()
        .expect("client is not initialized")
        .get(&url)
        .send()
        .await?
        .text()
        .await?;
    let json: Value = serde_json::from_str(&response)?;
    let data = json["data"][code][format!("m{}", ts)].as_array().unwrap();

    //将数据转换为 Vec<StockData>
    let mut stock_data = Vec::new();
    for item in data {
        let time = NaiveDateTime::parse_from_str(item[0].as_str().unwrap(), "%Y%m%d%H%M")?;
        let open = item[1].as_str().unwrap().parse::<f64>()?;
        let close = item[2].as_str().unwrap().parse::<f64>()?;
        let high = item[3].as_str().unwrap().parse::<f64>()?;
        let low = item[4].as_str().unwrap().parse::<f64>()?;
        let volume = item[5].as_str().unwrap().parse::<f64>()?;

        stock_data.push(StockData {
            time: StockTime::DateTime(time),
            open,
            close,
            high,
            low,
            volume,
            moving_averages: None,
        });
    }

    // 处理最新基金数据
    if let Some(latest_close) = json["data"][code]["qt"][code][3].as_f64() {
        if let Some(last_entry) = stock_data.last_mut() {
            last_entry.close = latest_close;
        }
    }
    Ok(stock_data)
}
/// 从新浪财经 API 获取股票的历史价格数据，并计算指定的均线数据。
/// 特点：可以获取均线数据（该接口目前支持5、10、15、20、30日均线数据）。
///
/// 该函数通过新浪财经的 API 获取指定股票代码的历史 K 线数据，并根据请求的频率（如日线、周线等）
/// 和均线参数（如 MA5、MA15 等）返回包含股票数据和均线信息的 `StockData` 列表。
///
/// # 参数
/// - `code`: 股票代码，例如 `"sh600000"` 表示上证指数的浦发银行。
/// - `count`: 请求的数据条数，例如 `10` 表示获取最近 10 条数据。
/// - `frequency`: K 线数据的频率，支持以下格式：
///   - `"1d"`: 日线（默认）
///   - `"1w"`: 周线
///   - `"1M"`: 月线
///   - 其他自定义频率，例如 `"30m"` 表示 30 分钟线。
/// - `mas`: 需要计算的均线天数列表，例如 `&[5, 15, 20]` 表示计算 MA5、MA15 和 MA20。该接口目前支持5、10、15、20、30日均线数据。
///
/// # 返回值
/// - 成功时返回 `Result<Vec<StockData>, Box<dyn std::error::Error>>`，其中 `Vec<StockData>` 包含解析后的股票数据。
/// - 失败时返回一个动态错误类型 `Box<dyn std::error::Error>`，可能包含网络请求失败、数据解析失败等错误。
///
/// # 示例
/// ```
/// use your_crate::{get_price_sina, StockData};
///
/// #[tokio::main]
/// async fn main() {
///     let code = "sh600000";
///     let count = 10;
///     let frequency = "1d";
///     let mas = &[5, 15, 20];
///
///     match get_price_sina(code, count, frequency, mas).await {
///         Ok(data) => {
///             for stock in data {
///                 println!("{:?}", stock);
///             }
///         }
///         Err(e) => eprintln!("Failed to fetch stock data: {}", e),
///     }
/// }
/// ```
///
/// # 错误
/// - 如果网络请求失败（例如无法连接到新浪财经服务器），会返回网络错误。
/// - 如果 API 返回的数据格式不符合预期，会返回数据解析错误。
/// - 如果均线参数 `mas` 中的天数在 API 返回的数据中不存在，则对应的均线数据会被忽略。
///
/// # 注意
/// - 该函数依赖于新浪财经的 API，API 的稳定性和数据格式可能会发生变化。
/// - 默认情况下，日线数据的时间类型为 `StockTime::Date`，其他频率的时间类型为 `StockTime::DateTime
#[maybe_async]
pub async fn get_price_sina(
    code: &str,
    count: usize,
    frequency: &str,
    mas: &[i32],
) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    let frequency = frequency
        .replace("1d", "240m")
        .replace("1w", "1200m")
        .replace("1M", "7200m");
    //如果解析失败，默认为240，即日线
    let ts = frequency[..frequency.len() - 1]
        .parse::<i64>()
        .unwrap_or(240);
    let ma_str = mas.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let url = format!(
        "http://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={}&scale={}&ma={}&datalen={}",
        code, ts, ma_str, count
    );
    // println!("{}",url);
    let response = CLIENT
        .get()
        .expect("client is not initialized")
        .get(&url)
        .send()
        .await?
        .text()
        .await?;
    let data: Vec<Value> = serde_json::from_str(&response).unwrap();

    let mut stock_data = Vec::new();
    for item in data {
        //因为从240开始就是日线级别了
        let time = if ts > 239 {
            StockTime::Date(NaiveDate::parse_from_str(item["day"].as_str().unwrap(), "%Y-%m-%d").unwrap())
        } else {
            StockTime::DateTime(NaiveDateTime::parse_from_str(item["day"].as_str().unwrap(), "%Y-%m-%d %H:%M:%S").unwrap())
        };
        let open = item["open"].as_str().unwrap().parse::<f64>().unwrap();
        let close = item["close"].as_str().unwrap().parse::<f64>().unwrap();
        let high = item["high"].as_str().unwrap().parse::<f64>().unwrap();
        let low = item["low"].as_str().unwrap().parse::<f64>().unwrap();
        let volume = item["volume"].as_str().unwrap().parse::<f64>().unwrap();
        let mut moving_averages = Vec::with_capacity(mas.len());
        for ma in mas {
            let ma_price = item[&format!("ma_price{}", ma)].as_f64();
            let ma_volume = item[&format!("ma_volume{}", ma)].as_f64();
            if let (Some(price),Some(volume)) = (ma_price, ma_volume) {
                moving_averages.push(MovingAverageData{name:*ma,value: price, volume });
            }
        }
        stock_data.push(StockData {
            time,
            open,
            close,
            high,
            low,
            volume,
            moving_averages: if moving_averages.is_empty() { None } else { Some(moving_averages) },
        });
    }
    Ok(stock_data)
}