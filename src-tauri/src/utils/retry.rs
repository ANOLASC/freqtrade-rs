use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries (milliseconds)
    pub base_delay_ms: u64,
    /// Maximum delay between retries (milliseconds)
    pub max_delay_ms: u64,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
    /// Whether to retry on transient errors
    pub retry_on_transient: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            retry_on_transient: true,
        }
    }
}

/// Execute an async operation with retry logic
///
/// # Arguments
/// * `operation` - The async operation to execute
/// * `config` - Retry configuration
/// * `is_retryable` - Function to determine if an error is retryable
///
/// # Returns
/// Result of the operation
pub async fn with_retry<T, F, Fut, E>(
    mut operation: F,
    config: RetryConfig,
    is_retryable: impl Fn(&E) -> bool,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_error: Option<E> = None;
    let mut delay_ms = config.base_delay_ms;

    for attempt in 0..=config.max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if !is_retryable(&e) || attempt == config.max_retries {
                    return Err(e);
                }
                last_error = Some(e);

                // Check for transient errors if configured
                if !config.retry_on_transient {
                    return Err(last_error.expect("Last error should be Some"));
                }

                eprintln!(
                    "Attempt {}/{} failed, retrying in {}ms...",
                    attempt + 1,
                    config.max_retries + 1,
                    delay_ms
                );

                sleep(Duration::from_millis(delay_ms)).await;

                // Exponential backoff with cap
                delay_ms = ((delay_ms as f64) * config.backoff_multiplier) as u64;
                delay_ms = delay_ms.min(config.max_delay_ms);
            }
        }
    }

    // All retries exhausted - return the last error
    // This is only reachable if max_retries is 0 and first attempt fails with retryable error
    // but retry_on_transient is false
    Err(last_error.expect("Loop must have stored an error"))
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    /// Circuit breaker is closed, allowing requests
    Closed,
    /// Circuit breaker is half-open, testing if service is recovered
    HalfOpen,
    /// Circuit breaker is open, blocking requests
    Open,
}

/// Circuit breaker for protecting against cascade failures
#[derive(Debug)]
pub struct CircuitBreaker {
    state: tokio::sync::RwLock<CircuitBreakerState>,
    failure_count: tokio::sync::RwLock<u32>,
    success_count: tokio::sync::RwLock<u32>,
    /// Number of failures before opening the circuit
    failure_threshold: u32,
    /// Number of successes in half-open state before closing
    success_threshold: u32,
    /// Timeout before trying half-open state (seconds)
    timeout_secs: u64,
    last_failure: tokio::sync::RwLock<Option<std::time::Instant>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    ///
    /// # Arguments
    /// * `failure_threshold` - Number of failures before opening the circuit
    /// * `success_threshold` - Number of successes in half-open state before closing
    /// * `timeout_secs` - Timeout before trying half-open state (seconds)
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout_secs: u64) -> Self {
        Self {
            state: tokio::sync::RwLock::new(CircuitBreakerState::Closed),
            failure_count: tokio::sync::RwLock::new(0),
            success_count: tokio::sync::RwLock::new(0),
            failure_threshold,
            success_threshold,
            timeout_secs,
            last_failure: tokio::sync::RwLock::new(None),
        }
    }

    /// Check if the operation should be allowed
    ///
    /// This method uses a write lock to atomically check and update state,
    /// preventing TOCTOU race conditions.
    pub async fn can_proceed(&self) -> bool {
        let mut state = self.state.write().await;
        match *state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::HalfOpen => true,
            CircuitBreakerState::Open => {
                // Check if timeout has elapsed
                let last_failure = self.last_failure.read().await;
                if let Some(last) = *last_failure {
                    if last.elapsed().as_secs() >= self.timeout_secs {
                        // Transition to half-open atomically
                        *state = CircuitBreakerState::HalfOpen;
                        drop(last_failure); // Release read lock before acquiring write
                        *self.success_count.write().await = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Record a successful operation
    pub async fn record_success(&self) {
        let state = *self.state.read().await;
        match state {
            CircuitBreakerState::HalfOpen => {
                let mut success_count = self.success_count.write().await;
                *success_count += 1;
                if *success_count >= self.success_threshold {
                    *self.state.write().await = CircuitBreakerState::Closed;
                    *self.failure_count.write().await = 0;
                }
            }
            CircuitBreakerState::Closed => {
                // Reset failure count on success
                *self.failure_count.write().await = 0;
            }
            CircuitBreakerState::Open => {}
        }
    }

    /// Record a failed operation
    pub async fn record_failure(&self) {
        let state = *self.state.read().await;
        match state {
            CircuitBreakerState::HalfOpen => {
                // Any failure in half-open goes back to open
                *self.state.write().await = CircuitBreakerState::Open;
                *self.last_failure.write().await = Some(std::time::Instant::now());
            }
            CircuitBreakerState::Closed => {
                let mut failure_count = self.failure_count.write().await;
                *failure_count += 1;
                if *failure_count >= self.failure_threshold {
                    *self.state.write().await = CircuitBreakerState::Open;
                    *self.last_failure.write().await = Some(std::time::Instant::now());
                }
            }
            CircuitBreakerState::Open => {
                *self.last_failure.write().await = Some(std::time::Instant::now());
            }
        }
    }

    /// Get current state
    pub async fn get_state(&self) -> CircuitBreakerState {
        *self.state.read().await
    }
}

/// Execute an operation with circuit breaker protection
pub async fn with_circuit_breaker<T, F, Fut>(
    mut operation: F,
    circuit_breaker: &CircuitBreaker,
) -> Result<T, CircuitBreakerError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, crate::error::AppError>>,
{
    if !circuit_breaker.can_proceed().await {
        return Err(CircuitBreakerError::Open);
    }

    match operation().await {
        Ok(result) => {
            circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            circuit_breaker.record_failure().await;
            Err(CircuitBreakerError::OperationFailed(e))
        }
    }
}

/// Error from circuit breaker
#[derive(Debug)]
pub enum CircuitBreakerError {
    /// Circuit breaker is open
    Open,
    /// Operation failed
    OperationFailed(crate::error::AppError),
}

impl std::fmt::Display for CircuitBreakerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::Open => write!(f, "Circuit breaker is open"),
            CircuitBreakerError::OperationFailed(e) => write!(f, "Operation failed: {}", e),
        }
    }
}

impl std::error::Error for CircuitBreakerError {}
