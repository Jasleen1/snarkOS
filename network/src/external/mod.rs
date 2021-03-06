// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

pub mod channel;
pub use channel::*;

pub mod message;
pub use message::*;

/// Messages are serialized into bytes for transmission, and deserialized into a message payload when received.
pub mod message_types;
#[doc(inline)]
pub use message_types::*;

pub mod propagate;
pub use propagate::*;

pub mod protocol;
pub use protocol::*;
