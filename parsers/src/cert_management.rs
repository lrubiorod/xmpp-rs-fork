// Copyright (c) 2019 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::iq::{IqGetPayload, IqResultPayload, IqSetPayload};
use crate::util::helpers::Base64;

generate_elem_id!(
    /// The name of a certificate.
    Name,
    "name",
    SASL_CERT
);

generate_element!(
    /// An X.509 certificate.
    Cert, "x509cert", SASL_CERT,
    text: (
        /// The BER X.509 data.
        data: Base64<Vec<u8>>
    )
);

generate_element!(
    /// For the client to upload an X.509 certificate.
    Append, "append", SASL_CERT,
    children: [
        /// The name of this certificate.
        name: Required<Name> = ("name", SASL_CERT) => Name,

        /// The X.509 certificate to set.
        cert: Required<Cert> = ("x509cert", SASL_CERT) => Cert,

        /// This client is forbidden from managing certificates.
        no_cert_management: Present<_> = ("no-cert-management", SASL_CERT) => bool
    ]
);

impl IqSetPayload for Append {}

generate_empty_element!(
    /// Client requests the current list of X.509 certificates.
    ListCertsQuery,
    "items",
    SASL_CERT
);

impl IqGetPayload for ListCertsQuery {}

generate_elem_id!(
    /// One resource currently using a certificate.
    Resource,
    "resource",
    SASL_CERT
);

generate_element!(
    /// A list of resources currently using this certificate.
    Users, "users", SASL_CERT,
    children: [
        /// Resources currently using this certificate.
        resources: Vec<Resource> = ("resource", SASL_CERT) => Resource
    ]
);

generate_element!(
    /// An X.509 certificate being set for this user.
    Item, "item", SASL_CERT,
    children: [
        /// The name of this certificate.
        name: Required<Name> = ("name", SASL_CERT) => Name,

        /// The X.509 certificate to set.
        cert: Required<Cert> = ("x509cert", SASL_CERT) => Cert,

        /// This client is forbidden from managing certificates.
        no_cert_management: Present<_> = ("no-cert-management", SASL_CERT) => bool,

        /// List of resources currently using this certificate.
        users: Option<Users> = ("users", SASL_CERT) => Users
    ]
);

generate_element!(
    /// Server answers with the current list of X.509 certificates.
    ListCertsResponse, "items", SASL_CERT,
    children: [
        /// List of certificates.
        items: Vec<Item> = ("item", SASL_CERT) => Item
    ]
);

impl IqResultPayload for ListCertsResponse {}

generate_element!(
    /// Client disables an X.509 certificate.
    Disable, "disable", SASL_CERT,
    children: [
        /// Name of the certificate to disable.
        name: Required<Name> = ("name", SASL_CERT) => Name
    ]
);

impl IqSetPayload for Disable {}

generate_element!(
    /// Client revokes an X.509 certificate.
    Revoke, "revoke", SASL_CERT,
    children: [
        /// Name of the certificate to revoke.
        name: Required<Name> = ("name", SASL_CERT) => Name
    ]
);

impl IqSetPayload for Revoke {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ns;
    use crate::Element;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_size() {
        assert_size!(Append, 28);
        assert_size!(Disable, 12);
        assert_size!(Revoke, 12);
        assert_size!(ListCertsQuery, 0);
        assert_size!(ListCertsResponse, 12);
        assert_size!(Item, 40);
        assert_size!(Resource, 12);
        assert_size!(Users, 12);
        assert_size!(Cert, 12);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_size() {
        assert_size!(Append, 56);
        assert_size!(Disable, 24);
        assert_size!(Revoke, 24);
        assert_size!(ListCertsQuery, 0);
        assert_size!(ListCertsResponse, 24);
        assert_size!(Item, 80);
        assert_size!(Resource, 24);
        assert_size!(Users, 24);
        assert_size!(Cert, 24);
    }

    #[test]
    fn simple() {
        let elem: Element = "<append xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name><x509cert>AAAA</x509cert></append>".parse().unwrap();
        let append = Append::try_from(elem).unwrap();
        assert_eq!(append.name.0, "Mobile Client");
        assert_eq!(append.cert.data, b"\0\0\0");

        let elem: Element =
            "<disable xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name></disable>"
                .parse()
                .unwrap();
        let disable = Disable::try_from(elem).unwrap();
        assert_eq!(disable.name.0, "Mobile Client");

        let elem: Element =
            "<revoke xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name></revoke>"
                .parse()
                .unwrap();
        let revoke = Revoke::try_from(elem).unwrap();
        assert_eq!(revoke.name.0, "Mobile Client");
    }

    #[test]
    fn list() {
        let elem: Element = r#"
        <items xmlns='urn:xmpp:saslcert:1'>
          <item>
            <name>Mobile Client</name>
            <x509cert>AAAA</x509cert>
            <users>
              <resource>Phone</resource>
            </users>
          </item>
          <item>
            <name>Laptop</name>
            <x509cert>BBBB</x509cert>
          </item>
        </items>"#
            .parse()
            .unwrap();
        let mut list = ListCertsResponse::try_from(elem).unwrap();
        assert_eq!(list.items.len(), 2);

        let item = list.items.pop().unwrap();
        assert_eq!(item.name.0, "Laptop");
        assert_eq!(item.cert.data, [4, 16, 65]);
        assert!(item.users.is_none());

        let item = list.items.pop().unwrap();
        assert_eq!(item.name.0, "Mobile Client");
        assert_eq!(item.cert.data, b"\0\0\0");
        assert_eq!(item.users.unwrap().resources.len(), 1);
    }

    #[test]
    fn test_serialise() {
        let append = Append {
            name: Name::from_str("Mobile Client").unwrap(),
            cert: Cert {
                data: b"\0\0\0".to_vec(),
            },
            no_cert_management: false,
        };
        let elem: Element = append.into();
        assert!(elem.is("append", ns::SASL_CERT));

        let disable = Disable {
            name: Name::from_str("Mobile Client").unwrap(),
        };
        let elem: Element = disable.into();
        assert!(elem.is("disable", ns::SASL_CERT));
        let elem = elem.children().cloned().collect::<Vec<_>>().pop().unwrap();
        assert!(elem.is("name", ns::SASL_CERT));
        assert_eq!(elem.text(), "Mobile Client");
    }

    #[test]
    fn test_serialize_item() {
        let reference: Element = "<item xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name><x509cert>AAAA</x509cert></item>"
        .parse()
        .unwrap();

        let item = Item {
            name: Name::from_str("Mobile Client").unwrap(),
            cert: Cert {
                data: b"\0\0\0".to_vec(),
            },
            no_cert_management: false,
            users: None,
        };

        let serialized: Element = item.into();
        assert_eq!(serialized, reference);
    }

    #[test]
    fn test_serialize_append() {
        let reference: Element = "<append xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name><x509cert>AAAA</x509cert></append>"
        .parse()
        .unwrap();

        let append = Append {
            name: Name::from_str("Mobile Client").unwrap(),
            cert: Cert {
                data: b"\0\0\0".to_vec(),
            },
            no_cert_management: false,
        };

        let serialized: Element = append.into();
        assert_eq!(serialized, reference);
    }

    #[test]
    fn test_serialize_disable() {
        let reference: Element =
            "<disable xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name></disable>"
                .parse()
                .unwrap();

        let disable = Disable {
            name: Name::from_str("Mobile Client").unwrap(),
        };

        let serialized: Element = disable.into();
        assert_eq!(serialized, reference);
    }

    #[test]
    fn test_serialize_revoke() {
        let reference: Element =
            "<revoke xmlns='urn:xmpp:saslcert:1'><name>Mobile Client</name></revoke>"
                .parse()
                .unwrap();

        let revoke = Revoke {
            name: Name::from_str("Mobile Client").unwrap(),
        };

        let serialized: Element = revoke.into();
        assert_eq!(serialized, reference);
    }
}
