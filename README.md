# ashares (开源 极简 A股实时行情数据API)


中国股市A股股票行情实时数据最简封装API接口,包含日线,分时分钟线,以及均线数据,可用来研究，量化分析，证券股票程序化自动化交易系统。目前提供新浪腾讯接口，为量化研究者在数据获取方面极大地减轻工作量，更加专注于策略和模型的研究与实现。


功能特点
---
* 采用rust语言编写，性能好，运行稳定。

* 异步同步接口自由选择。

* 核心库轻量化:项目库就一个文件lib.rs,不用安装设置，可自由裁剪，随用随走。

* 双内核封装，新浪财经，腾讯股票的实时行情数据，包括任意历史日线，周线，月线，分钟线，小时线以及均线等，已经稳定运行数年。

* 全部数据格式清理成标准的股票数据格式，方便使用。

* ashares 可以用在任何需要量化研究，量化分析的场合。

### 先看一个最简单的例子 [ashares_test](https://github.com/Xiaobaishushu25/ashares/blob/master/src/ashares_test.rs)

```rust
async fn test_get_price_sina() {
    init_ashares().await;
    let code = "sh000001";
    let count = 10;
    let frequency = "1M";//月线

    let stock_data = get_price_sina(code, count, frequency,&[5])
        .await
        .unwrap();
    for data in stock_data {
        println!("{:?}",data);
    }
}
```

运行结果：
```
StockData { time: Date(2024-04-30), open: 3049.021, close: 3104.825, high: 3123.288, low: 2995.542, volume: 764786214900.0, moving_averages: Some([MovingAverageData { name: 5, value: 2984.929, volume: 698298207440.0 }]) }
StockData { time: Date(2024-05-31), open: 3132.512, close: 3086.813, high: 3174.266, low: 3085.384, volume: 699695866900.0, moving_averages: Some([MovingAverageData { name: 5, value: 3007.305, volume: 717066091800.0 }]) }
StockData { time: Date(2024-06-28), open: 3085.981, close: 2967.403, high: 3097.204, low: 2933.33, volume: 570135119100.0, moving_averages: Some([MovingAverageData { name: 5, value: 3043.076, volume: 692588894440.0 }]) }
StockData { time: Date(2024-07-31), open: 2965.245, close: 2938.749, high: 3004.987, low: 2865.149, volume: 663558797300.0, moving_averages: Some([MovingAverageData { name: 5, value: 3027.791, volume: 694049259080.0 }]) }
StockData { time: Date(2024-08-30), open: 2940.12, close: 2842.214, high: 2947.877, low: 2815.383, volume: 596594223200.0, moving_averages: Some([MovingAverageData { name: 5, value: 2988.001, volume: 658954044280.0 }]) }
StockData { time: Date(2024-09-30), open: 2832.35, close: 3336.497, high: 3358.588, low: 2689.695, volume: 694997327400.0, moving_averages: Some([MovingAverageData { name: 5, value: 3034.335, volume: 644996266780.0 }]) }
StockData { time: Date(2024-10-31), open: 3674.405, close: 3279.824, high: 3674.405, low: 3152.821, volume: 1254206040800.0, moving_averages: Some([MovingAverageData { name: 5, value: 3072.937, volume: 755898301560.0 }]) }
StockData { time: Date(2024-11-29), open: 3275.577, close: 3326.456, high: 3509.818, low: 3227.355, volume: 1443980932700.0, moving_averages: Some([MovingAverageData { name: 5, value: 3144.748, volume: 930667464280.0 }]) }
StockData { time: Date(2024-12-31), open: 3328.482, close: 3351.763, high: 3494.867, low: 3323.009, volume: 1293592128600.0, moving_averages: Some([MovingAverageData { name: 5, value: 3227.351, volume: 1056674130540.0 }]) }
StockData { time: Date(2025-01-27), open: 3347.939, close: 3250.601, high: 3351.722, low: 3140.978, volume: 763711682700.0, moving_averages: Some([MovingAverageData { name: 5, value: 3309.028, volume: 1090097622440.0 }]) }
```
### 本人其他开源项目 - 如果有项目能帮助到您，请右上角帮我点亮 ★star 以示鼓励！
* [交易助手：一个金融助手客户端，帮助您分析您感兴趣的股票和基金。主要指标是箱体趋势和均线。](https://github.com/Xiaobaishushu25/trade_tip)

* [ashares最简股票行情数据接口API,A股行情完全开源免费 rust实现，支持异步和同步](https://github.com/Xiaobaishushu25/ashares)

### 致谢
本项目参考了[Ashare](https://github.com/mpquant/Ashare),主要是增加了异步支持、均线支持以及部分代码优化。