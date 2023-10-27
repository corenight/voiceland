//! # Multiverse
//! ## The connection and portal manager
//!
//! This module manages the packet routing between users, providing a portal abstraction to segment all connections.

use std::net::SocketAddr;

#[derive(Clone)]
pub struct Multiverse {
    pub users: Vec<SocketAddr>,
}

impl Multiverse {
    /// Init multiverse
    pub fn new() -> Multiverse {
        Multiverse { users: vec![] }
    }
    /// Add a connection to the user vector
    pub fn add(&mut self, conn: SocketAddr) {
        self.users.push(conn);
    }

    /// Removes a connection from the user vector
    pub fn rm(&mut self, conn: SocketAddr) {
        if let Some(index) = self.users.iter().position(|p| *p == conn) {
            self.users.remove(index);
        };
    }
}
