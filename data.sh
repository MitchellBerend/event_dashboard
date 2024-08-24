#!/bin/bash

# Example 1: User Signup Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "user_signup",
  "timestamp": "2024-08-24T12:34:56Z",
  "user_id": "12345",
  "metadata": {
    "source": "web",
    "campaign": "summer_sale"
  }
}'

# Example 2: Order Placed Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "order_placed",
  "timestamp": "2024-08-24T12:34:56Z",
  "order_id": "98765",
  "amount": 150.75,
  "currency": "USD",
  "items": [
    {
      "product_id": "A1",
      "quantity": 2
    },
    {
      "product_id": "B2",
      "quantity": 1
    }
  ]
}'

# Example 3: Page View Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "page_view",
  "timestamp": "2024-08-24T13:00:01Z",
  "user_id": "56789",
  "page_url": "/home",
  "referrer": "google.com"
}'

# Example 4: Button Click Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "button_click",
  "timestamp": "2024-08-24T13:05:23Z",
  "user_id": "abc123",
  "button_id": "submit_order",
  "page_url": "/checkout"
}'

# Example 5: Product View Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "product_view",
  "timestamp": "2024-08-24T13:15:12Z",
  "user_id": "user789",
  "product_id": "P1234",
  "product_category": "electronics"
}'

# Example 6: Cart Addition Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "cart_addition",
  "timestamp": "2024-08-24T13:20:45Z",
  "user_id": "user987",
  "product_id": "B567",
  "quantity": 3,
  "price_per_unit": 29.99
}'

# Example 7: Checkout Start Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "checkout_start",
  "timestamp": "2024-08-24T13:30:05Z",
  "user_id": "user456",
  "cart_total": 159.95,
  "currency": "USD"
}'

# Example 8: Payment Success Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "payment_success",
  "timestamp": "2024-08-24T13:40:32Z",
  "user_id": "user321",
  "order_id": "O20240824",
  "amount": 99.99,
  "payment_method": "credit_card"
}'

# Example 9: Search Query Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "search_query",
  "timestamp": "2024-08-24T13:50:10Z",
  "user_id": "user789",
  "query": "wireless headphones",
  "results_count": 25
}'

# Example 10: Profile Update Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "profile_update",
  "timestamp": "2024-08-24T14:00:45Z",
  "user_id": "user123",
  "updated_fields": ["email", "phone_number"]
}'

# Example 11: Newsletter Signup Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "newsletter_signup",
  "timestamp": "2024-08-24T14:10:15Z",
  "user_id": "user654",
  "email": "user654@example.com",
  "source": "blog"
}'

# Example 12: Item Removed from Cart Event
curl -X POST http://localhost:8081/event \
-H "Content-Type: application/json" \
-d '{
  "event": "item_removed_from_cart",
  "timestamp": "2024-08-24T14:20:30Z",
  "user_id": "user111",
  "product_id": "C789",
  "quantity": 1
}'
