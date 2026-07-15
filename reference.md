# Reference
## Api Solana Dex
<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_wallet_profile</a>(request: GetWalletProfileDexRequest) -> Result&lt;SolanaDexWalletProfile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a wallet profile for a specific wallet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_wallet_profile(
            &GetWalletProfileDexRequest {
                wallet: "suqh5sHtr8HyJ7q8scBimULPkPpA557prMG47xCHQfK".to_string(),
                options: Some(SolanaDexWalletProfilePayloadOptions {
                    include_metadata: Some(true),
                    include_labels: Some(true),
                    include_metrics: Some(vec![SolanaDexWalletProfileTimeWindowEnum::Window7D]),
                    ..Default::default()
                }),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**wallet:** `String` — Wallet address to retrieve the profile for.
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<SolanaDexWalletProfilePayloadOptions>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">search_wallet_profiles</a>(request: SearchWalletProfilesDexRequest) -> Result&lt;SearchWalletProfilesDexResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Filter, query, and sort wallet profiles based on specified metrics and conditions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .search_wallet_profiles(
            &SearchWalletProfilesDexRequest {
                limit: Some(10),
                query: Some(SolanaDexWalletProfileSearchPayloadQuery {
                    fields: Some(vec![
                        SolanaDexWalletProfileSearchPayloadQueryTargetsEnum::IdentityName,
                    ]),
                    text: "cupsey".to_string(),
                    ..Default::default()
                }),
                sort: Some(SolanaDexProfileSearchPayloadSort {
                    field: "metrics.7d.cumulative_pnl".to_string(),
                    direction: SolanaDexProfileSearchPayloadSortDirectionEnum::Desc,
                }),
                dynamic_labels: Some(SolanaDexProfileSearchPayloadDynamicLabels(HashMap::from([
                    (
                        "smart".to_string(),
                        SolanaDexProfileSearchPayloadFilter {
                            ..Default::default()
                        },
                    ),
                ]))),
                options: Some(SolanaDexWalletProfilePayloadOptions {
                    include_metadata: Some(true),
                    include_labels: Some(true),
                    include_metrics: Some(vec![SolanaDexWalletProfileTimeWindowEnum::Window7D]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `Option<SolanaDexWalletProfileSearchPayloadQuery>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<SolanaDexProfileSearchPayloadFilter>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<SolanaDexProfileSearchPayloadSort>` 
    
</dd>
</dl>

<dl>
<dd>

**dynamic_labels:** `Option<SolanaDexProfileSearchPayloadDynamicLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<SolanaDexWalletProfilePayloadOptions>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_token_profile</a>(request: GetTokenProfileDexRequest) -> Result&lt;SolanaDexTokenProfile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the profile for a specific token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_token_profile(
            &GetTokenProfileDexRequest {
                token: "Z4d9YXR4pSkdKcu9UBcwxHp7i32buzdDtAR1b1Gbonk".to_string(),
                options: Some(SolanaDexTokenProfilePayloadOptions {
                    include_metadata: Some(true),
                    include_market: Some(true),
                    include_labels: Some(true),
                    include_metrics: Some(vec![SolanaDexTokenProfileTimeWindowEnum::Window7D]),
                    ..Default::default()
                }),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` — Token address to retrieve the profile for.
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<SolanaDexTokenProfilePayloadOptions>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">search_token_profiles</a>(request: SearchTokenProfilesDexRequest) -> Result&lt;SearchTokenProfilesDexResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Filter, query, and sort token profiles based on specified metrics and conditions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .search_token_profiles(
            &SearchTokenProfilesDexRequest {
                limit: Some(10),
                query: Some(SolanaDexTokenProfileSearchPayloadQueryField {
                    fields: Some(vec![
                        SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum::MetadataName,
                    ]),
                    text: "bonk".to_string(),
                    ..Default::default()
                }),
                sort: Some(SolanaDexProfileSearchPayloadSort {
                    field: "market.liquidity".to_string(),
                    direction: SolanaDexProfileSearchPayloadSortDirectionEnum::Desc,
                }),
                dynamic_labels: Some(SolanaDexProfileSearchPayloadDynamicLabels(HashMap::from([
                    (
                        "trending".to_string(),
                        SolanaDexProfileSearchPayloadFilter {
                            ..Default::default()
                        },
                    ),
                ]))),
                options: Some(SolanaDexTokenProfilePayloadOptions {
                    include_metadata: Some(true),
                    include_market: Some(true),
                    include_labels: Some(true),
                    include_metrics: Some(vec![SolanaDexTokenProfileTimeWindowEnum::Window7D]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `Option<SolanaDexTokenProfileSearchPayloadQueryField>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<SolanaDexProfileSearchPayloadFilter>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<SolanaDexProfileSearchPayloadSort>` 
    
</dd>
</dl>

<dl>
<dd>

**dynamic_labels:** `Option<SolanaDexProfileSearchPayloadDynamicLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<SolanaDexTokenProfilePayloadOptions>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_trades</a>(request: GetTradesDexRequest) -> Result&lt;GetTradesDexResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns trades for a combination of wallet, token and/or pool.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_trades(
            &GetTradesDexRequest {
                limit: Some(20),
                wallet: Some("suqh5sHtr8HyJ7q8scBimULPkPpA557prMG47xCHQfK".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**wallet:** `Option<String>` — Wallet address to filter trades by.
    
</dd>
</dl>

<dl>
<dd>

**token:** `Option<String>` — Token address to filter trades by.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_swaps</a>(request: GetSwapsDexRequest) -> Result&lt;GetSwapsDexResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns swaps for a combination of wallet, token and/or pool.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_swaps(
            &GetSwapsDexRequest {
                limit: Some(20),
                wallet: Some("suqh5sHtr8HyJ7q8scBimULPkPpA557prMG47xCHQfK".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**wallet:** `Option<String>` — Wallet address to filter swaps by.
    
</dd>
</dl>

<dl>
<dd>

**token:** `Option<String>` — Token address to filter swaps by.
    
</dd>
</dl>

<dl>
<dd>

**pool:** `Option<String>` — Pool address to filter swaps by.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_price</a>(request: GetPriceDexRequest) -> Result&lt;Vec&lt;SolanaDexPrice&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns prices for one or more tokens or pools.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_price(
            &GetPriceDexRequest {
                tokens: Some(vec![
                    "Z4d9YXR4pSkdKcu9UBcwxHp7i32buzdDtAR1b1Gbonk".to_string()
                ]),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**tokens:** `Option<Vec<String>>` — Token addresses to retrieve the latest prices for.
    
</dd>
</dl>

<dl>
<dd>

**pools:** `Option<Vec<String>>` — Pool addresses to retrieve the latest prices for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_price_stats</a>(request: GetPriceStatsDexRequest) -> Result&lt;Vec&lt;SolanaDexPriceStats&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns price stats for one or more tokens or pools.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_price_stats(
            &GetPriceStatsDexRequest {
                tokens: Some(vec![
                    "Z4d9YXR4pSkdKcu9UBcwxHp7i32buzdDtAR1b1Gbonk".to_string()
                ]),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**tokens:** `Option<Vec<String>>` — Token addresses to retrieve price statistics for.
    
</dd>
</dl>

<dl>
<dd>

**pools:** `Option<Vec<String>>` — Pool addresses to retrieve price statistics for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_price_candles</a>(request: GetPriceCandlesDexRequest) -> Result&lt;Vec&lt;SolanaDexPriceCandle&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns price candles for a specific token and/or pool.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_price_candles(
            &GetPriceCandlesDexRequest {
                token: Some("Z4d9YXR4pSkdKcu9UBcwxHp7i32buzdDtAR1b1Gbonk".to_string()),
                from: Some(DateTime::parse_from_rfc3339("2026-04-27T00:00:00Z").unwrap()),
                to: Some(DateTime::parse_from_rfc3339("2026-04-27T01:00:00Z").unwrap()),
                interval: 60,
                pool: None,
                count: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `Option<String>` — Token address to filter by.
    
</dd>
</dl>

<dl>
<dd>

**pool:** `Option<String>` — Pool address to filter by.
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` 

Start of the candle range, as a date-time RFC3339 string.
Can be combined with `to` to define a bounded range.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` 

End of the candle range, as a date-time RFC3339 string. 
Defaults to the current time.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` 

Number of candles to return.
Must be combined with `from` or `to`.
    
</dd>
</dl>

<dl>
<dd>

**interval:** `i64` — Sampling interval between data points, in seconds.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api().solana().dex.<a href="/src/api/resources/api/solana/dex/client.rs">get_price_history</a>(request: GetPriceHistoryDexRequest) -> Result&lt;Vec&lt;SolanaDexPriceHistory&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns price history for one or more tokens or pools.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use prism_rs_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = Client::new(config).expect("Failed to build client");
    client
        .api
        .solana
        .dex
        .get_price_history(
            &GetPriceHistoryDexRequest {
                tokens: Some(vec![
                    "Z4d9YXR4pSkdKcu9UBcwxHp7i32buzdDtAR1b1Gbonk".to_string()
                ]),
                from: DateTime::parse_from_rfc3339("2026-04-27T00:00:00Z").unwrap(),
                to: Some(DateTime::parse_from_rfc3339("2026-04-27T01:00:00Z").unwrap()),
                interval: 3600,
                pools: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**tokens:** `Option<Vec<String>>` — Token addresses to retrieve price history for.
    
</dd>
</dl>

<dl>
<dd>

**pools:** `Option<Vec<String>>` — Pool addresses to retrieve price history for.
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — Start of the history range, as a date-time RFC3339 string.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` 

End of the history range, as a date-time RFC3339 string. 
Defaults to the current time.
    
</dd>
</dl>

<dl>
<dd>

**interval:** `i64` — Sampling interval between data points, in seconds.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

