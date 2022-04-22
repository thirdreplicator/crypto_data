# Redis Schema for Candle Data

We will be storing the candlestick data from the [Binance API](https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-data) in a local Redis database. The most common query to be performed will be loading up candle data within a date range, so we'll need the Redis sorted set (ZADD, ZRANGEBYSCORE) to store the candle timestamps of the start time. The actual price data for the candle will be stored as a Redis hash map (HMSET, HMGET) with a key that incorporates the timestamp. Inserting and retrieving data will be a two step process. 

I don't know if this is the best way to store the data in Redis, but it seems like a natural first attempt. Let me know if you have a better schema that would load faster. Since there are 525,000 minutes per year, it would take about 500,000 queries to load up a year's worth of data. I guess we could use Redis pipelining though. Anyway, I'm open to hearing your ideas. Maybe you can open an issue in the project if you would like to discuss.

## Insert Candle
1. ZADD the start time in milliseconds
2. HSET the key-value pairs.

## Read/Load Candles
1. ZRANGEBYSCORE <start_time_ms> <stop_time_ms>
2. For each timestamp, HMGET <key_prefix>:<timestamp> <field_0> [field_1 [field_2] ... ]

<key_prefix> looks like candles:<exchange>:<pair>:<period>. See the example below.

---
## Fields

The Redis hash map fields roughly correspond to the Binance API:
| field | data type |
|-------|-----------|
| open_time | Unixtime (ms) |
| open | float |
| high | float |
| low | float |
| close | float| 
| volume | float|
| quote_volume | float|
| num_trades | int|
| base_taker_buy_volume | float|
| quote_taker_buy_volume | float|


---

## Insert Candle
### Schema
```redis
HSET candles:<exchange>:<pair>:<period>:<unix_time_ms> time <unixtime_ms> open <float> high <float> low <float> close <float> volume <float> quote_volume <float> num_trades <int> base_taker_buy_volume <float> quote_taker_buy_volume <float>
```

### Example:
```redis
HSET candles:binance:btc/usdt:1m:1650499200000 time 1650499200000 open 41273.42 high 41299.88 low 41101.5 close 41.200.01 volume 23814.2 quote_volume 10013456.342 num_trades 232  base_taker_buy_volume 12584.3 quote_taker_buy_volume 513487239.43
```

---
## Insert Timestamp
To a range of dates on available unixtime_ms, insert unix time of candle as the score. When exchanges go down for maintenace there will be gaps more than the length of the period in ms.
### Schema
```
ZADD candles:<exchange>:<pair>:<period> <unix_timestamp_ms>
```
### Example
```
ZADD candles:binance:btc/usdt:1m 1650499200000 1
```
## Get Timestamps in Range

### Schema
```
ZRANGEBYSCORE candles:<exchange>:<pair>:<period> <unixtime_ms_start> <unixtime_ms_stop> WITHSCORES
```
### Example
```
ZRANGEBYSCORE candles:binance:btc/usdt:1m 1650499200000 1660499200000 WITHSCORES
```

## Get candle data
### Schema
```
HMGET candle:binance:btc/usdt:1m open high low close volume
```
### Example
```
HMGET candle:binance:btc/usdt:1m open high low close volume
```

