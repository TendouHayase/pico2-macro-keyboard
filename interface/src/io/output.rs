use crate::ErrorType;

pub trait Output: ErrorType {
    type Item;

    /// Sends the full set of currently pressed keys to the host.
    ///
    /// If `pressed` holds more keys than the implementation can represent
    /// at once, the implementation sends the protocol-defined
    /// "cannot represent" state instead. The caller does not need to know
    /// the capacity.
    ///
    /// # Errors
    ///
    /// - [`ErrorKind::NotConnected`](crate::ErrorKind::NotConnected):
    ///   returned immediately, without waiting, if the host is not connected.
    fn send_report(
        &mut self,
        pressed: &[Self::Item],
    ) -> impl Future<Output = Result<(), Self::Error>>;
}
