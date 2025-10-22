# Chapa API Endpoints (Summary)

Curated list of common Chapa API endpoints with required params and minimal sample responses. Source: https://developer.chapa.co/ (docs last referenced Oct 2025). Always consult the official docs for latest details and edge cases.

Notes
- Base URL: https://api.chapa.co/v1
- Auth: All endpoints require Authorization: Bearer <SECRET_KEY>
- Content types vary by endpoint (JSON or form). Use the example under each endpoint.
- tx_ref (your reference) vs ref_id (Chapa reference) differ; follow each endpoint’s path/param.

## Transactions (Standard Checkout)

### 1) Initialize Payment
- Method: POST
- URL: /transaction/initialize
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/x-www-form-urlencoded
- Body (required): amount (string), currency (string), email (string), tx_ref (string)
- Body (optional): first_name, last_name, phone_number, callback_url, return_url, customization[title, description, logo]
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Hosted Link",
  "data": { "checkout_url": "https://checkout.chapa.co/pay/xyz" }
}
```

### 2) Verify Payment
- Method: GET
- URL: /transaction/verify/{tx_ref}
- Path: tx_ref = your transaction reference
- Headers: Authorization: Bearer <SECRET>
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Transaction fetched",
  "data": {
    "tx_ref": "your-tx-ref",
    "reference": "APqDvY...",
    "amount": "10",
    "currency": "ETB",
    "status": "success"
  }
}
```

### 3) All Transactions
- Method: GET
- URL: /transactions
- Headers: Authorization: Bearer <SECRET>

### 4) Transaction Timeline (Events)
- Method: GET
- URL: /transaction/events/{ref_id}
- Path: ref_id = Chapa reference id for the transaction
- Headers: Authorization: Bearer <SECRET>

## Transfers

### 5) Initiate Transfer
- Method: POST
- URL: /transfers
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body (required): account_number (string), amount (string|number), bank_code (number)
- Body (common): account_name (string), currency (string), reference (string, optional)
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Transfer queued",
  "data": { "id": 123, "reference": "MYMER3434989", "status": "pending" }
}
```

### 6) Verify Transfer
- Method: GET
- URL: /transfers/verify/{tx_ref}
- Headers: Authorization: Bearer <SECRET>

### 7) Bulk Transfers
- Method: POST
- URL: /bulk-transfers
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body: title (string), currency (string), bulk_data (array of transfer objects same as single transfer)
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Bulk transfer queued",
  "data": { "id": 98765, "title": "This Month Salary!" }
}
```

### 8) All Transfers
- Method: GET
- URL: /transfers
- Headers: Authorization: Bearer <SECRET>

### 9) All Transfers (filter by batch)
- Method: GET
- URL: /transfers?batch_id={id}
- Headers: Authorization: Bearer <SECRET>

### 10) List Banks
- Method: GET
- URL: /banks
- Headers: Authorization: Bearer <SECRET>

### 11) Get Balance (all currencies)
- Method: GET
- URL: /balances
- Headers: Authorization: Bearer <SECRET>
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Balances fetched",
  "data": [
    { "currency": "ETB", "available_balance": 1000, "ledger_balance": 1200 },
    { "currency": "USD", "available_balance": 50, "ledger_balance": 50 }
  ]
}
```

### 12) Get Balance (by currency)
- Method: GET
- URL: /balances/{currency_code}
- Headers: Authorization: Bearer <SECRET>

### 13) Swap Currency (USD -> ETB)
- Method: POST
- URL: /swap
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body (required): amount (number, min 1), from ("USD"), to ("ETB")
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Swap completed",
  "data": { "from": "USD", "to": "ETB", "amount": 100, "rate": 57.1 }
}
```

## Direct Charge

### 14) Initiate Direct Charge
- Method: POST
- URL: /charges?type={payment_method}
- Headers: Authorization: Bearer <SECRET>, Content-Type: multipart/form-data or application/x-www-form-urlencoded
- Query: type = telebirr | mpesa | cbebirr | ebirr | enat_bank | amole | awashbirr (see docs for supported types)
- Body (common required): amount, currency (ETB), tx_ref, mobile (for mobile money)
- Note: Some providers need additional fields; see provider-specific docs.
- Sample response (USSD pattern, trimmed):
```
{ "status": "success", "message": "Charge initiated", "data": { "reference": "CHcuKj...", "status": "pending" } }
```

### 15) Authorize Direct Charge
- Method: POST
- URL: /validate?type={payment_method}
- Headers: Authorization: Bearer <SECRET>, Content-Type: multipart/form-data
- Body (typical): reference (string), client (string; encrypted payload per 3DES instructions)
- Sample response (trimmed):
```
{ "status": "success", "message": "Charge authorized" }
```

## Subaccounts (Split Payments)

### 16) Create Subaccount
- Method: POST
- URL: /subaccount
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body (required): business_name (string), account_name (string), bank_code (number), account_number (string), split_type ("percentage"|"flat"), split_value (number)
- Notes: bank_code comes from List Banks; splits settle in ETB by default; you can override split per transaction.
- Sample response (trimmed):
```
{ "status": "success", "message": "Subaccount created", "data": { "id": "3380b03b-..." } }
```

## Virtual Accounts

### 17) Create Virtual Account
- Method: POST
- URL: /virtual-account
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body: account_name (string, required), initial_deposit (number, optional), account_alias (string, optional)
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Virtual account created",
  "data": { "account_number": "100798347", "account_name": "SEMER NUR", "balance": 20, "currency": "ETB" }
}
```

### 18) Get Virtual Account
- Method: GET
- URL: /virtual-account/{accountNumber}
- Headers: Authorization: Bearer <SECRET>

### 19) Credit Virtual Account
- Method: POST
- URL: /virtual-account/credit
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body: account_number (string), amount (number), tx_ref (string, optional), note (string, optional)
- Sample response (trimmed):
```
{
  "status": "success",
  "message": "Account credited",
  "data": {
    "account": { "account_number": "100314252", "balance": 120, "currency": "ETB" },
    "transaction": { "type": "credit", "amount": 20, "tx_ref": "VTkVnh5ScB" }
  }
}
```

### 20) Debit Virtual Account
- Method: POST
- URL: /virtual-account/deduct
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/json
- Body: account_number (string), amount (number), tx_ref (string, optional), note (string, optional)

### 21) List Business Virtual Accounts
- Method: GET
- URL: /virtual-accounts
- Headers: Authorization: Bearer <SECRET>
- Response: Paginated list (25 per page) of virtual accounts with balances and status.

### 22) Get Virtual Account Debit History
- Method: GET
- URL: /virtual-account/history/debit/{account_number}
- Headers: Authorization: Bearer <SECRET>
- Path: account_number (string)
- Response: Paginated debit transactions for the account.

### 23) Get Virtual Account Credit History
- Method: GET
- URL: /virtual-account/history/credit/{account_number}
- Headers: Authorization: Bearer <SECRET>
- Path: account_number (string)
- Response: Paginated credit transactions for the account.

## Refunds

### 24) Refund Payment
- Method: POST
- URL: /refund/{tx_ref}
- Headers: Authorization: Bearer <SECRET>, Content-Type: application/x-www-form-urlencoded
- Body (optional): reason (string), amount (string), reference (string), meta[custom]=value
- Notes: If amount is omitted, the full transaction amount is refunded. Refund deducts from your available balance.
- Sample response (trimmed):
```
{ "status": "success", "message": "Refund queued", "data": { "tx_ref": "APezQ1K...", "amount": "1000" } }
```

---

Extras
- Receipt (not an API endpoint): https://chapa.link/payment-receipt/{chapa_reference_id}
- Verify after payment or authorization; prefer webhooks for reliability.

Tips
- Always verify payments after redirection or direct charge authorization (Verify Payment endpoint).
- For bulk operations, respect rate limits (e.g., <= 100 items per batch, ~5s between batches).
- Use webhooks for reliable post-payment and post-authorization signals (configure in Dashboard).
