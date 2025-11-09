Market data
/public/get_apr_history
Retrieves historical APR data for specified currency. Only applicable to yield-generating tokens (USDE, STETH, USDC, BUILD).

📖 Related Support Article: Yield reward-bearing coins

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	usde
steth
usdc
build	Currency for which to retrieve APR history
limit	false	integer		Number of days to retrieve (default 365, maximum 365)
before	false	integer		Used to receive APR history before given epoch day
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  continuation	string	Continuation token for pagination.
  ›  data	array of object	
  ›    ›  apr	number	The APR of the day
  ›    ›  day	integer	The full epoch day
/public/get_book_summary_by_currency
Retrieves the summary information such as open interest, 24h volume, etc. for all instruments for the currency (optionally filtered by kind). Note - For real-time updates, we recommend using the WebSocket subscription to ticker.{instrument_name}.{interval} instead of polling this endpoint.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
kind	false	string	future
option
spot
future_combo
option_combo	Instrument kind, if not provided instruments of all kinds are considered
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  ask_price	number	The current best ask price, null if there aren't any asks
  ›  base_currency	string	Base currency
  ›  bid_price	number	The current best bid price, null if there aren't any bids
  ›  creation_timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  current_funding	number	Current funding (perpetual only)
  ›  estimated_delivery_price	number	Optional (only for derivatives). Estimated delivery price for the market. For more details, see Contract Specification > General Documentation > Expiration Price.
  ›  funding_8h	number	Funding 8h (perpetual only)
  ›  high	number	Price of the 24h highest trade
  ›  instrument_name	string	Unique instrument identifier
  ›  interest_rate	number	Interest rate used in implied volatility calculations (options only)
  ›  last	number	The price of the latest trade, null if there weren't any trades
  ›  low	number	Price of the 24h lowest trade, null if there weren't any trades
  ›  mark_iv	number	(Only for option) implied volatility for mark price
  ›  mark_price	number	The current instrument market price
  ›  mid_price	number	The average of the best bid and ask, null if there aren't any asks or bids
  ›  open_interest	number	Optional (only for derivatives). The total amount of outstanding contracts in the corresponding amount units. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  price_change	number	24-hour price change expressed as a percentage, null if there weren't any trades
  ›  quote_currency	string	Quote currency
  ›  underlying_index	string	Name of the underlying future, or 'index_price' (options only)
  ›  underlying_price	number	underlying price for implied volatility calculations (options only)
  ›  volume	number	The total 24h traded volume (in base currency)
  ›  volume_notional	number	Volume in quote currency (futures and spots only)
  ›  volume_usd	number	Volume in USD
/public/get_book_summary_by_instrument
Retrieves the summary information such as open interest, 24h volume, etc. for a specific instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  ask_price	number	The current best ask price, null if there aren't any asks
  ›  base_currency	string	Base currency
  ›  bid_price	number	The current best bid price, null if there aren't any bids
  ›  creation_timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  current_funding	number	Current funding (perpetual only)
  ›  estimated_delivery_price	number	Optional (only for derivatives). Estimated delivery price for the market. For more details, see Contract Specification > General Documentation > Expiration Price.
  ›  funding_8h	number	Funding 8h (perpetual only)
  ›  high	number	Price of the 24h highest trade
  ›  instrument_name	string	Unique instrument identifier
  ›  interest_rate	number	Interest rate used in implied volatility calculations (options only)
  ›  last	number	The price of the latest trade, null if there weren't any trades
  ›  low	number	Price of the 24h lowest trade, null if there weren't any trades
  ›  mark_iv	number	(Only for option) implied volatility for mark price
  ›  mark_price	number	The current instrument market price
  ›  mid_price	number	The average of the best bid and ask, null if there aren't any asks or bids
  ›  open_interest	number	Optional (only for derivatives). The total amount of outstanding contracts in the corresponding amount units. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  price_change	number	24-hour price change expressed as a percentage, null if there weren't any trades
  ›  quote_currency	string	Quote currency
  ›  underlying_index	string	Name of the underlying future, or 'index_price' (options only)
  ›  underlying_price	number	underlying price for implied volatility calculations (options only)
  ›  volume	number	The total 24h traded volume (in base currency)
  ›  volume_notional	number	Volume in quote currency (futures and spots only)
  ›  volume_usd	number	Volume in USD
/public/get_contract_size
Retrieves contract size of provided instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  contract_size	integer	Contract size, for futures in USD, for options in base currency of the instrument (BTC, ETH, ...)
/public/get_currencies
Retrieves all cryptocurrencies supported by the API.

Try in API console

Parameters
This method takes no parameters

Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  apr	number	Simple Moving Average (SMA) of the last 7 days of rewards. If fewer than 7 days of reward data are available, the APR is calculated as the average of the available rewards. Only applicable to yield-generating tokens (USDE, STETH, USDC, BUILD).
  ›  coin_type	string	The type of the currency.
  ›  currency	string	The abbreviation of the currency. This abbreviation is used elsewhere in the API to identify the currency.
  ›  currency_long	string	The full name for the currency.
  ›  decimals	integer	The number of decimal places for the currency
  ›  in_cross_collateral_pool	boolean	true if the currency is part of the cross collateral pool
  ›  min_confirmations	integer	Minimum number of block chain confirmations before deposit is accepted.
  ›  min_withdrawal_fee	number	The minimum transaction fee paid for withdrawals
  ›  network_currency	string	The currency of the network
  ›  network_fee	number	The network fee
  ›  withdrawal_fee	number	The total transaction fee paid for withdrawals
  ›  withdrawal_priorities	array of object	
  ›    ›  name	string	
  ›    ›  value	number	
/public/get_delivery_prices
Retrieves delivery prices for then given index

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
index_name	true	string	btc_usd
eth_usd
ada_usdc
algo_usdc
avax_usdc
bch_usdc
bnb_usdc
btc_usdc
btcdvol_usdc
buidl_usdc
doge_usdc
dot_usdc
eurr_usdc
eth_usdc
ethdvol_usdc
link_usdc
ltc_usdc
near_usdc
paxg_usdc
shib_usdc
sol_usdc
steth_usdc
ton_usdc
trump_usdc
trx_usdc
uni_usdc
usde_usdc
usyc_usdc
xrp_usdc
btc_usdt
eth_usdt
eurr_usdt
sol_usdt
steth_usdt
usdc_usdt
usde_usdt
btc_eurr
btc_usde
btc_usyc
eth_btc
eth_eurr
eth_usde
eth_usyc
steth_eth
paxg_btc
drbfix-btc_usdc
drbfix-eth_usdc	Index identifier, matches (base) cryptocurrency with quote currency
offset	false	integer		The offset for pagination, default - 0
count	false	integer		Number of requested items, default - 10, maximum - 1000
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  data	array of object	
  ›    ›  date	string	The event date with year, month and day
  ›    ›  delivery_price	number	The settlement price for the instrument. Only when state = closed
  ›  records_total	number	Available delivery prices
/public/get_expirations
Retrieves expirations for instruments. This method can be used to see instruments's expirations.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
any
grouped	The currency symbol or "any" for all or '"grouped"' for all grouped by currency
kind	true	string	future
option
any	Instrument kind, "future" or "option" or "any"
currency_pair	false	string	btc_usd
eth_usd
ada_usdc
algo_usdc
avax_usdc
bch_usdc
bnb_usdc
btc_usdc
btcdvol_usdc
buidl_usdc
doge_usdc
dot_usdc
eurr_usdc
eth_usdc
ethdvol_usdc
link_usdc
ltc_usdc
near_usdc
paxg_usdc
shib_usdc
sol_usdc
steth_usdc
ton_usdc
trump_usdc
trx_usdc
uni_usdc
usde_usdc
usyc_usdc
xrp_usdc
btc_usdt
eth_usdt
eurr_usdt
sol_usdt
steth_usdt
usdc_usdt
usde_usdt
btc_eurr
btc_usde
btc_usyc
eth_btc
eth_eurr
eth_usde
eth_usyc
steth_eth
paxg_btc
drbfix-btc_usdc
drbfix-eth_usdc	The currency pair symbol
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	A map where each key is valid currency (e.g. btc, eth, usdc), and the value is a list of expirations or a map where each key is a valid kind (future or options) and value is a list of expirations from every instrument
  ›  currency	string	Currency name or "any" if don't care or "grouped" if grouped by currencies
  ›  kind	string	Instrument kind: "future", "option" or "any" for all
/public/get_funding_chart_data
Retrieve the list of the latest PERPETUAL funding chart points within a given time period.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
length	true	string	8h
24h
1m	Specifies time period. 8h - 8 hours, 24h - 24 hours, 1m - 1 month
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  current_interest	number	Current interest
  ›  data	array of object	
  ›    ›  index_price	number	Current index price
  ›    ›  interest_8h	number	Historical interest 8h value
  ›    ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  interest_8h	number	Current interest 8h
/public/get_funding_rate_history
Retrieves hourly historical interest rate for requested PERPETUAL instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch)
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  index_price	number	Price in base currency
  ›  interest_1h	float	1hour interest rate
  ›  interest_8h	float	8hour interest rate
  ›  prev_index_price	number	Price in base currency
  ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
/public/get_funding_rate_value
Retrieves interest rate value for requested period. Applicable only for PERPETUAL instruments.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch)
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	float	
/public/get_historical_volatility
Provides information about historical volatility for given cryptocurrency.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of [timestamp, value]	
/public/get_index
Retrieves the current index price for the instruments, for the selected currency.

Try in API console

This method is deprecated and will be removed in the future.
Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  BTC	number	The current index price for BTC-USD (only for selected currency == BTC)
  ›  ETH	number	The current index price for ETH-USD (only for selected currency == ETH)
  ›  edp	number	Estimated delivery price for the currency. For more details, see Documentation > General > Expiration Price
/public/get_index_price
Retrieves the current index price value for given index name.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
index_name	true	string	btc_usd
eth_usd
ada_usdc
algo_usdc
avax_usdc
bch_usdc
bnb_usdc
btc_usdc
btcdvol_usdc
buidl_usdc
doge_usdc
dot_usdc
eurr_usdc
eth_usdc
ethdvol_usdc
link_usdc
ltc_usdc
near_usdc
paxg_usdc
shib_usdc
sol_usdc
steth_usdc
ton_usdc
trump_usdc
trx_usdc
uni_usdc
usde_usdc
usyc_usdc
xrp_usdc
btc_usdt
eth_usdt
eurr_usdt
sol_usdt
steth_usdt
usdc_usdt
usde_usdt
btc_eurr
btc_usde
btc_usyc
eth_btc
eth_eurr
eth_usde
eth_usyc
steth_eth
paxg_btc
drbfix-btc_usdc
drbfix-eth_usdc	Index identifier, matches (base) cryptocurrency with quote currency
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  estimated_delivery_price	number	Estimated delivery price for the market. For more details, see Documentation > General > Expiration Price
  ›  index_price	number	Value of requested index
/public/get_index_price_names
Retrieves the identifiers of all supported Price Indexes

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
extended	false	boolean		When set to true, returns additional information including future_combo_creation_enabled and option_combo_creation_enabled for each index
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  future_combo_creation_enabled	boolean	Whether future combo creation is enabled for this index (only present when extended=true)
  ›  name	string	Index name
  ›  option_combo_creation_enabled	boolean	Whether option combo creation is enabled for this index (only present when extended=true)
/public/get_instrument
Retrieves information about instrument

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  base_currency	string	The underlying currency being traded.
  ›  block_trade_commission	number	Block Trade commission for instrument.
  ›  block_trade_min_trade_amount	number	Minimum amount for block trading.
  ›  block_trade_tick_size	number	Specifies minimal price change for block trading.
  ›  contract_size	integer	Contract size for instrument.
  ›  counter_currency	string	Counter currency for the instrument.
  ›  creation_timestamp	integer	The time when the instrument was first created (milliseconds since the UNIX epoch).
  ›  expiration_timestamp	integer	The time when the instrument will expire (milliseconds since the UNIX epoch).
  ›  future_type	string	Future type (only for futures)(field is deprecated and will be removed in the future, instrument_type should be used instead).
  ›  instrument_id	integer	Instrument ID
  ›  instrument_name	string	Unique instrument identifier
  ›  instrument_type	string	Type of the instrument. linear or reversed
  ›  is_active	boolean	Indicates if the instrument can currently be traded.
  ›  kind	string	Instrument kind: "future", "option", "spot", "future_combo", "option_combo"
  ›  maker_commission	number	Maker commission for instrument.
  ›  max_leverage	integer	Maximal leverage for instrument (only for futures).
  ›  max_liquidation_commission	number	Maximal liquidation trade commission for instrument (only for futures).
  ›  min_trade_amount	number	Minimum amount for trading. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  option_type	string	The option type (only for options).
  ›  price_index	string	Name of price index that is used for this instrument
  ›  quote_currency	string	The currency in which the instrument prices are quoted.
  ›  settlement_currency	string	Optional (not added for spot). Settlement currency for the instrument.
  ›  settlement_period	string	Optional (not added for spot). The settlement period.
  ›  strike	number	The strike value (only for options).
  ›  taker_commission	number	Taker commission for instrument.
  ›  tick_size	number	Specifies minimal price change and, as follows, the number of decimal places for instrument prices.
  ›  tick_size_steps	object	
  ›    ›  above_price	number	The price from which the increased tick size applies
  ›    ›  tick_size	number	Tick size to be used above the price. It must be multiple of the minimum tick size.
/public/get_instruments
Retrieves available trading instruments. This method can be used to see which instruments are available for trading, or which instruments have recently expired. Note - This endpoint has distinct API rate limiting requirements: 1 request per 10 seconds, with a burst of 5. To avoid rate limits, we recommend using either the REST requests for server-cached data or the WebSocket subscription to instrument_state.{kind}.{currency} for real-time updates. For more information, see Rate Limits.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR
any	The currency symbol or "any" for all
kind	false	string	future
option
spot
future_combo
option_combo	Instrument kind, if not provided instruments of all kinds are considered
expired	false	boolean		Set to true to show recently expired instruments instead of active ones.
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  base_currency	string	The underlying currency being traded.
  ›  block_trade_commission	number	Block Trade commission for instrument.
  ›  block_trade_min_trade_amount	number	Minimum amount for block trading.
  ›  block_trade_tick_size	number	Specifies minimal price change for block trading.
  ›  contract_size	integer	Contract size for instrument.
  ›  counter_currency	string	Counter currency for the instrument.
  ›  creation_timestamp	integer	The time when the instrument was first created (milliseconds since the UNIX epoch).
  ›  expiration_timestamp	integer	The time when the instrument will expire (milliseconds since the UNIX epoch).
  ›  future_type	string	Future type (only for futures)(field is deprecated and will be removed in the future, instrument_type should be used instead).
  ›  instrument_id	integer	Instrument ID
  ›  instrument_name	string	Unique instrument identifier
  ›  instrument_type	string	Type of the instrument. linear or reversed
  ›  is_active	boolean	Indicates if the instrument can currently be traded.
  ›  kind	string	Instrument kind: "future", "option", "spot", "future_combo", "option_combo"
  ›  maker_commission	number	Maker commission for instrument.
  ›  max_leverage	integer	Maximal leverage for instrument (only for futures).
  ›  max_liquidation_commission	number	Maximal liquidation trade commission for instrument (only for futures).
  ›  min_trade_amount	number	Minimum amount for trading. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  option_type	string	The option type (only for options).
  ›  price_index	string	Name of price index that is used for this instrument
  ›  quote_currency	string	The currency in which the instrument prices are quoted.
  ›  settlement_currency	string	Optional (not added for spot). Settlement currency for the instrument.
  ›  settlement_period	string	Optional (not added for spot). The settlement period.
  ›  strike	number	The strike value (only for options).
  ›  taker_commission	number	Taker commission for instrument.
  ›  tick_size	number	Specifies minimal price change and, as follows, the number of decimal places for instrument prices.
  ›  tick_size_steps	object	
  ›    ›  above_price	number	The price from which the increased tick size applies
  ›    ›  tick_size	number	Tick size to be used above the price. It must be multiple of the minimum tick size.
/public/get_last_settlements_by_currency
Retrieves historical settlement, delivery and bankruptcy events coming from all instruments within a given currency.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
type	false	string	settlement
delivery
bankruptcy	Settlement type
count	false	integer		Number of requested items, default - 20, maximum - 1000
continuation	false	string		Continuation token for pagination
search_start_timestamp	false	integer		The latest timestamp to return result from (milliseconds since the UNIX epoch)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  continuation	string	Continuation token for pagination.
  ›  settlements	array of object	
  ›    ›  funded	number	funded amount (bankruptcy only)
  ›    ›  funding	number	funding (in base currency ; settlement for perpetual product only)
  ›    ›  index_price	number	underlying index price at time of event (in quote currency; settlement and delivery only)
  ›    ›  instrument_name	string	instrument name (settlement and delivery only)
  ›    ›  mark_price	number	mark price for at the settlement time (in quote currency; settlement and delivery only)
  ›    ›  position	number	position size (in quote currency; settlement and delivery only)
  ›    ›  profit_loss	number	profit and loss (in base currency; settlement and delivery only)
  ›    ›  session_bankruptcy	number	value of session bankruptcy (in base currency; bankruptcy only)
  ›    ›  session_profit_loss	number	total value of session profit and losses (in base currency)
  ›    ›  session_tax	number	total amount of paid taxes/fees (in base currency; bankruptcy only)
  ›    ›  session_tax_rate	number	rate of paid taxes/fees (in base currency; bankruptcy only)
  ›    ›  socialized	number	the amount of the socialized losses (in base currency; bankruptcy only)
  ›    ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›    ›  type	string	The type of settlement. settlement, delivery or bankruptcy.
/public/get_last_settlements_by_instrument
Retrieves historical public settlement, delivery and bankruptcy events filtered by instrument name.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
type	false	string	settlement
delivery
bankruptcy	Settlement type
count	false	integer		Number of requested items, default - 20, maximum - 1000
continuation	false	string		Continuation token for pagination
search_start_timestamp	false	integer		The latest timestamp to return result from (milliseconds since the UNIX epoch)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  continuation	string	Continuation token for pagination.
  ›  settlements	array of object	
  ›    ›  funded	number	funded amount (bankruptcy only)
  ›    ›  funding	number	funding (in base currency ; settlement for perpetual product only)
  ›    ›  index_price	number	underlying index price at time of event (in quote currency; settlement and delivery only)
  ›    ›  instrument_name	string	instrument name (settlement and delivery only)
  ›    ›  mark_price	number	mark price for at the settlement time (in quote currency; settlement and delivery only)
  ›    ›  position	number	position size (in quote currency; settlement and delivery only)
  ›    ›  profit_loss	number	profit and loss (in base currency; settlement and delivery only)
  ›    ›  session_bankruptcy	number	value of session bankruptcy (in base currency; bankruptcy only)
  ›    ›  session_profit_loss	number	total value of session profit and losses (in base currency)
  ›    ›  session_tax	number	total amount of paid taxes/fees (in base currency; bankruptcy only)
  ›    ›  session_tax_rate	number	rate of paid taxes/fees (in base currency; bankruptcy only)
  ›    ›  socialized	number	the amount of the socialized losses (in base currency; bankruptcy only)
  ›    ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›    ›  type	string	The type of settlement. settlement, delivery or bankruptcy.
/public/get_last_trades_by_currency
Retrieve the latest trades that have occurred for instruments in a specific currency symbol.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
kind	false	string	future
option
spot
future_combo
option_combo
combo
any	Instrument kind, "combo" for any combo or "any" for all. If not provided instruments of all kinds are considered
start_id	false	string		The ID of the first trade to be returned. Number for BTC trades, or hyphen name in ex. "ETH-15" # "ETH_USDC-16"
end_id	false	string		The ID of the last trade to be returned. Number for BTC trades, or hyphen name in ex. "ETH-15" # "ETH_USDC-16"
start_timestamp	false	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch). When param is provided trades are returned from the earliest
end_timestamp	false	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch). Only one of params: start_timestamp, end_timestamp is truly required
count	false	integer		Number of requested items, default - 10, maximum - 1000
sorting	false	string	asc
desc
default	Direction of results sorting (default value means no sorting, results will be returned in order in which they left the database)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  has_more	boolean	
  ›  trades	array of object	
  ›    ›  amount	number	Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›    ›  block_rfq_id	integer	ID of the Block RFQ - when trade was part of the Block RFQ
  ›    ›  block_trade_id	string	Block trade id - when trade was part of a block trade
  ›    ›  block_trade_leg_count	integer	Block trade leg count - when trade was part of a block trade
  ›    ›  combo_id	string	Optional field containing combo instrument name if the trade is a combo trade
  ›    ›  combo_trade_id	number	Optional field containing combo trade identifier if the trade is a combo trade
  ›    ›  contracts	number	Trade size in contract units (optional, may be absent in historical trades)
  ›    ›  direction	string	Direction: buy, or sell
  ›    ›  index_price	number	Index Price at the moment of trade
  ›    ›  instrument_name	string	Unique instrument identifier
  ›    ›  iv	number	Option implied volatility for the price (Option only)
  ›    ›  liquidation	string	Optional field (only for trades caused by liquidation): "M" when maker side of trade was under liquidation, "T" when taker side was under liquidation, "MT" when both sides of trade were under liquidation
  ›    ›  mark_price	number	Mark Price at the moment of trade
  ›    ›  price	number	Price in base currency
  ›    ›  tick_direction	integer	Direction of the "tick" (0 = Plus Tick, 1 = Zero-Plus Tick, 2 = Minus Tick, 3 = Zero-Minus Tick).
  ›    ›  timestamp	integer	The timestamp of the trade (milliseconds since the UNIX epoch)
  ›    ›  trade_id	string	Unique (per currency) trade identifier
  ›    ›  trade_seq	integer	The sequence number of the trade within instrument
/public/get_last_trades_by_currency_and_time
Retrieve the latest trades that have occurred for instruments in a specific currency symbol and within a given time range.

Scope: trade:read

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
kind	false	string	future
option
spot
future_combo
option_combo
combo
any	Instrument kind, "combo" for any combo or "any" for all. If not provided instruments of all kinds are considered
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch). When param is provided trades are returned from the earliest
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch). Only one of params: start_timestamp, end_timestamp is truly required
count	false	integer		Number of requested items, default - 10, maximum - 1000
sorting	false	string	asc
desc
default	Direction of results sorting (default value means no sorting, results will be returned in order in which they left the database)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  has_more	boolean	
  ›  trades	array of object	
  ›    ›  amount	number	Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›    ›  block_rfq_id	integer	ID of the Block RFQ - when trade was part of the Block RFQ
  ›    ›  block_trade_id	string	Block trade id - when trade was part of a block trade
  ›    ›  block_trade_leg_count	integer	Block trade leg count - when trade was part of a block trade
  ›    ›  combo_id	string	Optional field containing combo instrument name if the trade is a combo trade
  ›    ›  combo_trade_id	number	Optional field containing combo trade identifier if the trade is a combo trade
  ›    ›  contracts	number	Trade size in contract units (optional, may be absent in historical trades)
  ›    ›  direction	string	Direction: buy, or sell
  ›    ›  index_price	number	Index Price at the moment of trade
  ›    ›  instrument_name	string	Unique instrument identifier
  ›    ›  iv	number	Option implied volatility for the price (Option only)
  ›    ›  liquidation	string	Optional field (only for trades caused by liquidation): "M" when maker side of trade was under liquidation, "T" when taker side was under liquidation, "MT" when both sides of trade were under liquidation
  ›    ›  mark_price	number	Mark Price at the moment of trade
  ›    ›  price	number	Price in base currency
  ›    ›  tick_direction	integer	Direction of the "tick" (0 = Plus Tick, 1 = Zero-Plus Tick, 2 = Minus Tick, 3 = Zero-Minus Tick).
  ›    ›  timestamp	integer	The timestamp of the trade (milliseconds since the UNIX epoch)
  ›    ›  trade_id	string	Unique (per currency) trade identifier
  ›    ›  trade_seq	integer	The sequence number of the trade within instrument
/public/get_last_trades_by_instrument
Retrieve the latest trades that have occurred for a specific instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_seq	false	integer		The sequence number of the first trade to be returned
end_seq	false	integer		The sequence number of the last trade to be returned
start_timestamp	false	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch). When param is provided trades are returned from the earliest
end_timestamp	false	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch). Only one of params: start_timestamp, end_timestamp is truly required
count	false	integer		Number of requested items, default - 10, maximum - 1000
sorting	false	string	asc
desc
default	Direction of results sorting (default value means no sorting, results will be returned in order in which they left the database)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  has_more	boolean	
  ›  trades	array of object	
  ›    ›  amount	number	Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›    ›  block_rfq_id	integer	ID of the Block RFQ - when trade was part of the Block RFQ
  ›    ›  block_trade_id	string	Block trade id - when trade was part of a block trade
  ›    ›  block_trade_leg_count	integer	Block trade leg count - when trade was part of a block trade
  ›    ›  combo_id	string	Optional field containing combo instrument name if the trade is a combo trade
  ›    ›  combo_trade_id	number	Optional field containing combo trade identifier if the trade is a combo trade
  ›    ›  contracts	number	Trade size in contract units (optional, may be absent in historical trades)
  ›    ›  direction	string	Direction: buy, or sell
  ›    ›  index_price	number	Index Price at the moment of trade
  ›    ›  instrument_name	string	Unique instrument identifier
  ›    ›  iv	number	Option implied volatility for the price (Option only)
  ›    ›  liquidation	string	Optional field (only for trades caused by liquidation): "M" when maker side of trade was under liquidation, "T" when taker side was under liquidation, "MT" when both sides of trade were under liquidation
  ›    ›  mark_price	number	Mark Price at the moment of trade
  ›    ›  price	number	Price in base currency
  ›    ›  tick_direction	integer	Direction of the "tick" (0 = Plus Tick, 1 = Zero-Plus Tick, 2 = Minus Tick, 3 = Zero-Minus Tick).
  ›    ›  timestamp	integer	The timestamp of the trade (milliseconds since the UNIX epoch)
  ›    ›  trade_id	string	Unique (per currency) trade identifier
  ›    ›  trade_seq	integer	The sequence number of the trade within instrument
/public/get_last_trades_by_instrument_and_time
Retrieve the latest trades that have occurred for a specific instrument and within a given time range.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch). When param is provided trades are returned from the earliest
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch). Only one of params: start_timestamp, end_timestamp is truly required
count	false	integer		Number of requested items, default - 10, maximum - 1000
sorting	false	string	asc
desc
default	Direction of results sorting (default value means no sorting, results will be returned in order in which they left the database)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  has_more	boolean	
  ›  trades	array of object	
  ›    ›  amount	number	Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›    ›  block_rfq_id	integer	ID of the Block RFQ - when trade was part of the Block RFQ
  ›    ›  block_trade_id	string	Block trade id - when trade was part of a block trade
  ›    ›  block_trade_leg_count	integer	Block trade leg count - when trade was part of a block trade
  ›    ›  combo_id	string	Optional field containing combo instrument name if the trade is a combo trade
  ›    ›  combo_trade_id	number	Optional field containing combo trade identifier if the trade is a combo trade
  ›    ›  contracts	number	Trade size in contract units (optional, may be absent in historical trades)
  ›    ›  direction	string	Direction: buy, or sell
  ›    ›  index_price	number	Index Price at the moment of trade
  ›    ›  instrument_name	string	Unique instrument identifier
  ›    ›  iv	number	Option implied volatility for the price (Option only)
  ›    ›  liquidation	string	Optional field (only for trades caused by liquidation): "M" when maker side of trade was under liquidation, "T" when taker side was under liquidation, "MT" when both sides of trade were under liquidation
  ›    ›  mark_price	number	Mark Price at the moment of trade
  ›    ›  price	number	Price in base currency
  ›    ›  tick_direction	integer	Direction of the "tick" (0 = Plus Tick, 1 = Zero-Plus Tick, 2 = Minus Tick, 3 = Zero-Minus Tick).
  ›    ›  timestamp	integer	The timestamp of the trade (milliseconds since the UNIX epoch)
  ›    ›  trade_id	string	Unique (per currency) trade identifier
  ›    ›  trade_seq	integer	The sequence number of the trade within instrument
/public/get_mark_price_history
Public request for 5min history of markprice values for the instrument. For now the markprice history is available only for a subset of options which take part in the volatility index calculations. All other instruments, futures and perpetuals will return an empty list.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch)
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array	Markprice history values as an array of arrays with 2 values each. The inner values correspond to the timestamp in ms and the markprice itself.
/public/get_order_book
Retrieves the order book, along with other market values for a given instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		The instrument name for which to retrieve the order book, see public/get_instruments to obtain instrument names.
depth	false	integer	1
5
10
20
50
100
1000
10000	The number of entries to return for bids and asks, maximum - 10000.
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  ask_iv	number	(Only for option) implied volatility for best ask
  ›  asks	array of [price, amount]	List of asks
  ›  best_ask_amount	number	It represents the requested order size of all best asks
  ›  best_ask_price	number	The current best ask price, null if there aren't any asks
  ›  best_bid_amount	number	It represents the requested order size of all best bids
  ›  best_bid_price	number	The current best bid price, null if there aren't any bids
  ›  bid_iv	number	(Only for option) implied volatility for best bid
  ›  bids	array of [price, amount]	List of bids
  ›  current_funding	number	Current funding (perpetual only)
  ›  delivery_price	number	The settlement price for the instrument. Only when state = closed
  ›  funding_8h	number	Funding 8h (perpetual only)
  ›  greeks	object	Only for options
  ›    ›  delta	number	(Only for option) The delta value for the option
  ›    ›  gamma	number	(Only for option) The gamma value for the option
  ›    ›  rho	number	(Only for option) The rho value for the option
  ›    ›  theta	number	(Only for option) The theta value for the option
  ›    ›  vega	number	(Only for option) The vega value for the option
  ›  index_price	number	Current index price
  ›  instrument_name	string	Unique instrument identifier
  ›  interest_rate	number	Interest rate used in implied volatility calculations (options only)
  ›  last_price	number	The price for the last trade
  ›  mark_iv	number	(Only for option) implied volatility for mark price
  ›  mark_price	number	The mark price for the instrument
  ›  max_price	number	The maximum price for the future. Any buy orders you submit higher than this price, will be clamped to this maximum.
  ›  min_price	number	The minimum price for the future. Any sell orders you submit lower than this price will be clamped to this minimum.
  ›  open_interest	number	The total amount of outstanding contracts in the corresponding amount units. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  settlement_price	number	Optional (not added for spot). The settlement price for the instrument. Only when state = open
  ›  state	string	The state of the order book. Possible values are open and closed.
  ›  stats	object	
  ›    ›  high	number	Highest price during 24h
  ›    ›  low	number	Lowest price during 24h
  ›    ›  price_change	number	24-hour price change expressed as a percentage, null if there weren't any trades
  ›    ›  volume	number	Volume during last 24h in base currency
  ›    ›  volume_usd	number	Volume in usd (futures only)
  ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  underlying_index	number	Name of the underlying future, or index_price (options only)
  ›  underlying_price	number	Underlying price for implied volatility calculations (options only)
/public/get_order_book_by_instrument_id
Retrieves the order book, along with other market values for a given instrument ID.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_id	true	integer		The instrument ID for which to retrieve the order book, see public/get_instruments to obtain instrument IDs.
depth	false	integer	1
5
10
20
50
100
1000
10000	The number of entries to return for bids and asks, maximum - 10000.
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  ask_iv	number	(Only for option) implied volatility for best ask
  ›  asks	array of [price, amount]	List of asks
  ›  best_ask_amount	number	It represents the requested order size of all best asks
  ›  best_ask_price	number	The current best ask price, null if there aren't any asks
  ›  best_bid_amount	number	It represents the requested order size of all best bids
  ›  best_bid_price	number	The current best bid price, null if there aren't any bids
  ›  bid_iv	number	(Only for option) implied volatility for best bid
  ›  bids	array of [price, amount]	List of bids
  ›  current_funding	number	Current funding (perpetual only)
  ›  delivery_price	number	The settlement price for the instrument. Only when state = closed
  ›  funding_8h	number	Funding 8h (perpetual only)
  ›  greeks	object	Only for options
  ›    ›  delta	number	(Only for option) The delta value for the option
  ›    ›  gamma	number	(Only for option) The gamma value for the option
  ›    ›  rho	number	(Only for option) The rho value for the option
  ›    ›  theta	number	(Only for option) The theta value for the option
  ›    ›  vega	number	(Only for option) The vega value for the option
  ›  index_price	number	Current index price
  ›  instrument_name	string	Unique instrument identifier
  ›  interest_rate	number	Interest rate used in implied volatility calculations (options only)
  ›  last_price	number	The price for the last trade
  ›  mark_iv	number	(Only for option) implied volatility for mark price
  ›  mark_price	number	The mark price for the instrument
  ›  max_price	number	The maximum price for the future. Any buy orders you submit higher than this price, will be clamped to this maximum.
  ›  min_price	number	The minimum price for the future. Any sell orders you submit lower than this price will be clamped to this minimum.
  ›  open_interest	number	The total amount of outstanding contracts in the corresponding amount units. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  settlement_price	number	Optional (not added for spot). The settlement price for the instrument. Only when state = open
  ›  state	string	The state of the order book. Possible values are open and closed.
  ›  stats	object	
  ›    ›  high	number	Highest price during 24h
  ›    ›  low	number	Lowest price during 24h
  ›    ›  price_change	number	24-hour price change expressed as a percentage, null if there weren't any trades
  ›    ›  volume	number	Volume during last 24h in base currency
  ›    ›  volume_usd	number	Volume in usd (futures only)
  ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  underlying_index	number	Name of the underlying future, or index_price (options only)
  ›  underlying_price	number	Underlying price for implied volatility calculations (options only)
/public/get_supported_index_names
Retrieves the identifiers of all supported Price Indexes

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
type	false	string	all
spot
derivative	Type of a cryptocurrency price index
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  future_combo_creation_enabled	boolean	Whether future combo creation is enabled for this index (only present when extended=true)
  ›  name	string	Index name
  ›  option_combo_creation_enabled	boolean	Whether option combo creation is enabled for this index (only present when extended=true)
/public/get_trade_volumes
Retrieves aggregated 24h trade volumes for different instrument types and currencies.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
extended	false	boolean		Request for extended statistics. Including also 7 and 30 days volumes (default false)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	array of object	
  ›  calls_volume	number	Total 24h trade volume for call options.
  ›  calls_volume_30d	number	Total 30d trade volume for call options.
  ›  calls_volume_7d	number	Total 7d trade volume for call options.
  ›  currency	string	Currency, i.e "BTC", "ETH", "USDC"
  ›  futures_volume	number	Total 24h trade volume for futures.
  ›  futures_volume_30d	number	Total 30d trade volume for futures.
  ›  futures_volume_7d	number	Total 7d trade volume for futures.
  ›  puts_volume	number	Total 24h trade volume for put options.
  ›  puts_volume_30d	number	Total 30d trade volume for put options.
  ›  puts_volume_7d	number	Total 7d trade volume for put options.
  ›  spot_volume	number	Total 24h trade for spot.
  ›  spot_volume_30d	number	Total 30d trade for spot.
  ›  spot_volume_7d	number	Total 7d trade for spot.
/public/get_tradingview_chart_data
Publicly available market data used to generate a TradingView candle chart.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch)
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch)
resolution	true	string	1
3
5
10
15
30
60
120
180
360
720
1D	Chart bars resolution given in full minutes or keyword 1D (only some specific resolutions are supported)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  close	array of number	List of prices at close (one per candle)
  ›  cost	array of number	List of cost bars (volume in quote currency, one per candle)
  ›  high	array of number	List of highest price levels (one per candle)
  ›  low	array of number	List of lowest price levels (one per candle)
  ›  open	array of number	List of prices at open (one per candle)
  ›  status	string	Status of the query: ok or no_data
  ›  ticks	array of integer	Values of the time axis given in milliseconds since UNIX epoch
  ›  volume	array of number	List of volume bars (in base currency, one per candle)
/public/get_volatility_index_data
Public market data request for volatility index candles.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
currency	true	string	BTC
ETH
USDC
USDT
EURR	The currency symbol
start_timestamp	true	integer		The earliest timestamp to return result from (milliseconds since the UNIX epoch)
end_timestamp	true	integer		The most recent timestamp to return result from (milliseconds since the UNIX epoch)
resolution	true	string	1
60
3600
43200
1D	Time resolution given in full seconds or keyword 1D (only some specific resolutions are supported)
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	Volatility index candles.
  ›  continuation	integer	Continuation - to be used as the end_timestamp parameter on the next request. NULL when no continuation.
  ›  data	array	Candles as an array of arrays with 5 values each. The inner values correspond to the timestamp in ms, open, high, low, and close values of the volatility index correspondingly.
/public/ticker
Get ticker for an instrument.

Try in API console

Parameters
Parameter	Required	Type	Enum	Description
instrument_name	true	string		Instrument name
Response
Name	Type	Description
id	integer	The id that was sent in the request
jsonrpc	string	The JSON-RPC version (2.0)
result	object	
  ›  ask_iv	number	(Only for option) implied volatility for best ask
  ›  best_ask_amount	number	It represents the requested order size of all best asks
  ›  best_ask_price	number	The current best ask price, null if there aren't any asks
  ›  best_bid_amount	number	It represents the requested order size of all best bids
  ›  best_bid_price	number	The current best bid price, null if there aren't any bids
  ›  bid_iv	number	(Only for option) implied volatility for best bid
  ›  current_funding	number	Current funding (perpetual only)
  ›  delivery_price	number	The settlement price for the instrument. Only when state = closed
  ›  estimated_delivery_price	number	Estimated delivery price for the market. For more details, see Contract Specification > General Documentation > Expiration Price
  ›  funding_8h	number	Funding 8h (perpetual only)
  ›  greeks	object	Only for options
  ›    ›  delta	number	(Only for option) The delta value for the option
  ›    ›  gamma	number	(Only for option) The gamma value for the option
  ›    ›  rho	number	(Only for option) The rho value for the option
  ›    ›  theta	number	(Only for option) The theta value for the option
  ›    ›  vega	number	(Only for option) The vega value for the option
  ›  index_price	number	Current index price
  ›  instrument_name	string	Unique instrument identifier
  ›  interest_rate	number	Interest rate used in implied volatility calculations (options only)
  ›  interest_value	number	Value used to calculate realized_funding in positions (perpetual only)
  ›  last_price	number	The price for the last trade
  ›  mark_iv	number	(Only for option) implied volatility for mark price
  ›  mark_price	number	The mark price for the instrument
  ›  max_price	number	The maximum price for the future. Any buy orders you submit higher than this price, will be clamped to this maximum.
  ›  min_price	number	The minimum price for the future. Any sell orders you submit lower than this price will be clamped to this minimum.
  ›  open_interest	number	The total amount of outstanding contracts in the corresponding amount units. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
  ›  settlement_price	number	Optional (not added for spot). The settlement price for the instrument. Only when state = open
  ›  state	string	The state of the order book. Possible values are open and closed.
  ›  stats	object	
  ›    ›  high	number	Highest price during 24h
  ›    ›  low	number	Lowest price during 24h
  ›    ›  price_change	number	24-hour price change expressed as a percentage, null if there weren't any trades
  ›    ›  volume	number	Volume during last 24h in base currency
  ›    ›  volume_usd	number	Volume in usd (futures only)
  ›  timestamp	integer	The timestamp (milliseconds since the Unix epoch)
  ›  underlying_index	number	Name of the underlying future, or index_price (options only)
  ›  underlying_price	number	Underlying price for implied volatility calculations (options only)
