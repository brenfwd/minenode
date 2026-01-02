use tokio_util::sync::CancellationToken;

pub type StopResult<T> = Result<T, ()>;

pub async fn stopped<T, U: Future<Output = T>>(
    stop: &CancellationToken,
    future: U,
) -> StopResult<T> {
    tokio::select! {
        _ = stop.cancelled() => {
            Err(())
        }
        t = future => {
            Ok(t)
        }
    }
}
