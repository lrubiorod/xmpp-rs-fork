// Copyright (c) 2017 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::message::MessagePayload;

generate_empty_element!(
    /// Requests that this message is acked by the final recipient once
    /// received.
    Request,
    "request",
    RECEIPTS
);

impl MessagePayload for Request {}

generate_element!(
    /// Notes that a previous message has correctly been received, it is
    /// referenced by its 'id' attribute.
    Received, "received", RECEIPTS,
    attributes: [
        /// The 'id' attribute of the received message.
        id: Required<String> = "id",
    ]
);

impl MessagePayload for Received {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ns;
    use crate::util::error::Error;
    use crate::Element;
    use std::convert::TryFrom;

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_size() {
        assert_size!(Request, 0);
        assert_size!(Received, 12);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_size() {
        assert_size!(Request, 0);
        assert_size!(Received, 24);
    }

    #[test]
    fn test_simple() {
        let elem: Element = "<request xmlns='urn:xmpp:receipts'/>".parse().unwrap();
        Request::try_from(elem).unwrap();

        let elem: Element = "<received xmlns='urn:xmpp:receipts' id='coucou'/>"
            .parse()
            .unwrap();
        Received::try_from(elem).unwrap();
    }

    #[test]
    fn test_missing_id() {
        let elem: Element = "<received xmlns='urn:xmpp:receipts'/>".parse().unwrap();
        let error = Received::try_from(elem).unwrap_err();
        let message = match error {
            Error::ParseError(string) => string,
            _ => panic!(),
        };
        assert_eq!(message, "Required attribute 'id' missing.");
    }

    #[test]
    fn test_serialise() {
        let receipt = Request;
        let elem: Element = receipt.into();
        assert!(elem.is("request", ns::RECEIPTS));
        assert_eq!(elem.attrs().count(), 0);

        let receipt = Received {
            id: String::from("coucou"),
        };
        let elem: Element = receipt.into();
        assert!(elem.is("received", ns::RECEIPTS));
        assert_eq!(elem.attr("id"), Some("coucou"));
    }
}
