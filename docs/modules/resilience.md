# Resilience

The `resilience` module provides advanced error handling and reliability features for Feather scripts. It enables your automation to gracefully handle failures, retries, timeouts, fallbacks, and circuit breaking, making scripts robust and production-ready.

## Features

- **Retry:** Automatically re-attempt failed operations, with support for exponential backoff, delay, and jitter to avoid resource contention.
- **Timeout:** Limit the maximum execution time for commands, ensuring scripts do not hang indefinitely.
- **Fallback:** Specify alternative actions or endpoints if the primary operation fails.
- **Circuit Breaker:** Temporarily disable failing operations after repeated errors, with configurable thresholds and cool-down periods.
- **Error Evaluation:** Retry or handle failures based on specific error codes or conditions.

## Usage Examples

### Retry with Backoff and Error Evaluation

```sky
resilience retry
  times '3'
  backoff 'exponential'
  delay '500' ms
  jitter '20' percent
  evaluate ['E_TIMEOUT','E_5XX']
  do ( http get 'https://api.example.com/status' into ::res )
into ::attempt.

if ::attempt:ok
  say 'ok'.
else
  failure ::attempt:message.
end
```

### Timeout

```sky
resilience timeout '2s'
  do ( http post 'https://slow.example.com' with @payload into ::res )
into ::timed.

if ::timed:ok
  say 'posted'.
else
  failure ::timed:code.         ~ likely 'E_TIMEOUT'
end
```

### Fallback

```sky
resilience fallback
  primary ( http get 'https://primary/api' into ::r1 )
  secondary ( http get 'https://backup/api' into ::r2 )
into ::final.

if ::final:ok
  say 'used primary or backup'.
else
  failure 'both failed'.
end
```

### Circuit Breaker

```sky
resilience circuit 'payments-api'
  failure-threshold 5 in '30s'
  cool-down '60s'
  do ( http get 'https://pay.example.com' into ::res )
into ::cb.

if ::cb:code is 'E_OPEN_CIRCUIT'
  failure 'temporarily disabled'.
end
```

## Best Practices

- Use retries with backoff and jitter for transient errors and network operations.
- Set timeouts for all external or long-running commands to prevent stuck scripts.
- Provide fallback logic for critical operations to improve reliability.
- Use circuit breakers to protect downstream services and avoid cascading failures.
- Evaluate error codes to handle only the failures you care about.
