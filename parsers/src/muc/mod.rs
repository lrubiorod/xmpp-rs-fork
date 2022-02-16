// Copyright (c) 2017 Maxime “pep” Buquet <pep@bouah.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// The http://jabber.org/protocol/muc protocol.
pub mod muc;

/// The http://jabber.org/protocol/muc#user protocol.
pub mod user;

pub use self::muc::Muc;
pub use self::user::MucUser;
