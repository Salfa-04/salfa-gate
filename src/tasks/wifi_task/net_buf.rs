//!
//! # Network Buffer
//!
//! This module contains the `NetBuffer` struct.
//!

use crate::hal;
use hal::rng::Rng;

use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Stack, dns::DnsSocket};
use reqwless::client::{HttpClient, TlsConfig, TlsVerify};

///
/// # Buffer struct.
///
/// This struct is used to store the seed, read and write buffers, and the TCP state.
///
/// ## Notes:
/// > This will cost `6144 + 2 * N * SIZE` bytes of memory at least.
///
/// ## Fields
/// * `tls_seed`            - Seed for the TLS configuration.
/// * `tls_read_buffer`     - Read buffer for the TLS configuration.
/// * `tls_write_buffer`    - Write buffer for the TLS configuration.
/// * `tcp_state`           - TCP state.
///
/// ## Generic parameters
/// * `N`       - Number of TCP clients.
/// * `SIZE`    - Size of the TCP client buffers.
/// * `BUF`     - Size of the HTTP Read buffer.
///
/// ## Example
///
/// ```
/// let mut buffer: Buffer<8, 1024> = Buffer::new(&mut rng);
/// let (client, dns, config) = buffer.configure(stack);
/// let mut client = HttpClient::new_with_tls(&client, &dns, config);
/// ```
///
#[non_exhaustive]
pub struct NetBuffer<'t, const N: usize, const SIZE: usize, const RX: usize = 1024> {
    tls_seed: u64,
    tls_read_buffer: [u8; 3072],
    tls_write_buffer: [u8; 3072],
    tcp_state: TcpClientState<N, SIZE, SIZE>,
    http_rxbuf: [u8; RX],
    tcp_client: Option<TcpClient<'t, N, SIZE, SIZE>>,
    dns_socket: Option<DnsSocket<'t>>,
}

impl<'t, const N: usize, const SIZE: usize, const RX: usize> NetBuffer<'t, N, SIZE, RX> {
    ///
    /// # Create a new buffer.
    ///
    /// This function creates a new buffer with the given random number generator.
    ///
    /// ## Parameters
    /// * `rng` - Random number generator.
    ///
    /// ## Generic parameters
    /// * `N`       - Number of TCP clients.
    /// * `SIZE`    - Size of the TCP client buffers.
    /// * `RX`      - Size of the HTTP Read buffer.
    ///
    /// ## Returns
    /// * A new `NetBuffer`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut buffer: Buffer<8, 1024, 512> = Buffer::new(rng);
    /// ```
    ///
    pub fn new(mut rng: Rng) -> NetBuffer<'t, N, SIZE, RX> {
        let seed = (rng.random() as u64) << 32 | rng.random() as u64;

        Self {
            tls_seed: seed,
            tls_read_buffer: [0; 3072],
            tls_write_buffer: [0; 3072],
            tcp_state: TcpClientState::new(),
            http_rxbuf: [0; RX],
            tcp_client: None,
            dns_socket: None,
        }
    }

    ///
    /// # Configure the buffer.
    ///
    /// This function configures the buffer with the given stack.
    ///
    /// ## Parameters
    /// * `stack` - Network stack.
    ///
    /// ## Returns
    /// * A tuple with the `HttpClient` and the HTTP Read buffer.
    ///     * The `HttpClient` is used to make requests.
    ///     * The HTTP Read buffer is used to read the response.
    ///         * The buffer is with the size of `RX`.
    ///
    /// ## Panics
    /// * If the configuration is done more than once.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut buffer: Buffer<8, 1024, 512> = Buffer::new(rng);
    /// let (client, buf) = buffer.configure(stack);
    ///
    /// ....
    ///
    /// let _ = request.send(buf).await;
    /// ```
    ///
    /// ## Note: `RNG` and `Stack` are `Copy` and `Clone` respectively.
    ///
    ///
    pub fn configure(
        &'t mut self,
        stack: Stack<'t>,
    ) -> (
        HttpClient<'t, TcpClient<'t, N, SIZE, SIZE>, DnsSocket<'t>>,
        &'t mut [u8],
    ) {
        if self.tcp_client.is_some() || self.dns_socket.is_some() {
            panic!("Configuration can only be done once.");
        }

        self.tcp_client = Some(TcpClient::new(stack, &self.tcp_state));
        self.dns_socket = Some(DnsSocket::new(stack));

        let config = TlsConfig::new(
            self.tls_seed,
            &mut self.tls_read_buffer,
            &mut self.tls_write_buffer,
            TlsVerify::None,
        );

        (
            // safety: The `TcpClient` and `DnsSocket` are initialized before calling this.
            HttpClient::new_with_tls(
                self.tcp_client.as_ref().unwrap(),
                self.dns_socket.as_ref().unwrap(),
                config,
            ),
            &mut self.http_rxbuf,
        )
    }
}
