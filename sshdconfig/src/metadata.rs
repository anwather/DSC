// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// TODO: ensure lists are complete

// keywords that can be repeated over multiple lines and should be represented as arrays
pub const REPEATABLE_KEYWORDS: [&str; 6] = [
    "hostkey",
    "include",
    "listenaddress",
    "port",
    "setenv",
    "subsystem"
];

// keywords that can have multiple argments per line and should be represented as arrays
// but cannot be repeated over multiple lines, as subsequent entries are ignored
pub const MULTI_ARG_KEYWORDS: [&str; 7] = [
    "casignaturealgorithms",
    "ciphers",
    "hostbasedacceptedalgorithms",
    "hostkeyalgorithms",
    "kexalgorithms",
    "macs",
    "pubkeyacceptedalgorithms"
];
