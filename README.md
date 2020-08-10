# huobi_future_async

Rust Async Library for the [Huobi Future API(restful+websocket)](https://huobiapi.github.io/docs/dm/v1/cn/)

[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
huobi_future_async = { git = "https://github.com/hbdmapi/huobi_future_async.git" }
```

## Risk Warning

Use at your own risk. We will not be responsible for your investment losses.

## Quick Start

```shell
git clone https://github.com/hbdmapi/huobi_future_async.git
cd huobi_future_async
cd examples
// add your api key and secret key in endpoints.rs.
vim endpoints.rs
cargo run --example endpoints
// add you api key and secret key in websocket.rs
vim websocket.rs
cargo run --example websocket
```

Happy Coding!

## Implemented Restful APIs

| Interface | Method Name | API Doc |
| --- | --- | --- |
| Get Contract Info(/api/v1/contract_contract_info) | get_contract_info | https://huobiapi.github.io/docs/dm/v1/cn/#6b15dcb6a3 |
| Get Market Depth(/market/depth) |  get_all_book_tickers | https://huobiapi.github.io/docs/dm/v1/cn/#1716b8327e |
| Get Klines(/market/history/kline) | get_klines | https://huobiapi.github.io/docs/dm/v1/cn/#k |
| Get Index Klines(/index/market/history/index) | get_index_klines | https://huobiapi.github.io/docs/dm/v1/cn/#k-2 |
| Get Basis Data(/index/market/history/basis) | get_basis | https://huobiapi.github.io/docs/dm/v1/cn/#8cfe8b0489 |
| Get Merged Data(/market/detail/merged) | get_merged_data | https://huobiapi.github.io/docs/dm/v1/cn/#k |
| Get Price Limit(/api/v1/contract_price_limit) | get_price_limit | https://huobiapi.github.io/docs/dm/v1/cn/#025c787500|
| Get Account Info(/api/v1/contract_account_info) | get_account_info | https://huobiapi.github.io/docs/dm/v1/cn/#e807c44c06 |
| Get Account and Position Info(/api/v1/contract_account_position_info) | get_account_position_info| https://huobiapi.github.io/docs/dm/v1/cn/#2aa4c454c6 |
| Place Order(/api/v1/contract_order) | place_order | https://huobiapi.github.io/docs/dm/v1/cn/#9dc85ffb46|
| Place Orders(/api/v1/contract_batchorder) | place_orders | https://huobiapi.github.io/docs/dm/v1/cn/#33123f0c09|
| Lightning close(/api/v1/lightning_close_position) | lightning_close | https://huobiapi.github.io/docs/dm/v1/cn/#k |
| Cancel order(/api/v1/contract_cancel) | cancel_orders | https://huobiapi.github.io/docs/dm/v1/cn/#4e53c0fccd |
| Cancel all orders(/api/v1/contract_cancelall) | cancel_allorders | https://huobiapi.github.io/docs/dm/v1/cn/#f175a031e7 |
| Get order info(/api/v1/contract_order_info) | get_order_info | https://huobiapi.github.io/docs/dm/v1/cn/#f36cfcbaef |
| Get order detail(/api/v1/contract_order_detail) | cancel_orders | https://huobiapi.github.io/docs/dm/v1/cn/#81b4bd7186 |
| Get open orders(/api/v1/contract_openorders) | get_open_orders | https://huobiapi.github.io/docs/dm/v1/cn/#dd332a7a9c |
| Place trigger orders(/api/v1/contract_trigger_order) | place_trigger_order | https://huobiapi.github.io/docs/dm/v1/cn/#97a9bd626d | 
| Cancel trigger orders(/api/v1/contract_trigger_cancel) | cancel_trigger_orders | https://huobiapi.github.io/docs/dm/v1/cn/#0d42beab34 | 
| Cancel all trigger orders(/api/v1/contract_trigger_cancelall) | cancel_all_trigger_orders | https://huobiapi.github.io/docs/dm/v1/cn/#3d2471d520 | 
| Get open trigger orders(/api/v1/contract_trigger_openorders) | get_trigger_open_orders | https://huobiapi.github.io/docs/dm/v1/cn/#b5280a27b3 | 
| Get history trigger orders(/api/v1/contract_trigger_hisorders) | get_trigger_his_orders | https://huobiapi.github.io/docs/dm/v1/cn/#37aeb9f3bd | 
| Get transfer limit(/api/v1/contract_transfer_limit) | get_transfer_limit | https://huobiapi.github.io/docs/dm/v1/cn/#36078ddf99 | 
| Transfer between spot and future(https://api.huobi.pro/v1/futures/transfer) | transfer | https://huobiapi.github.io/docs/dm/v1/cn/#88a4ecc2bc| 

## Implemented Websocket Interfaces

Support All the websocket subscriptions of API docs:

- [x] Market Websocket Subscription: https://huobiapi.github.io/docs/dm/v1/cn/#websocket-3

- [x] Index and Basis Websocket Subscription: https://huobiapi.github.io/docs/dm/v1/cn/#websocket-4

- [x] Account and Order Websocket Subscription: https://huobiapi.github.io/docs/dm/v1/cn/#websocket-5

