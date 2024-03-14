pub const PATTERNS: &'static [(&'static str, &'static str)] = &[
("aws::secret_access_key", r#"\<([0-9a-zA-Z+/]{40})\>"#),
("aws::access_key_id", r#"\<((?:A3T[A-Z0-9]|AKIA|AGPA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16})\>"#),
("aliyun::access_key_secret", r#"\<([0-9a-zA-Z]{30})\>"#),
("aliyun::access_key_id", r#"\<(LTAI[0-9a-zA-Z]{20})\>"#),
("_::email_address", r#"\<((?:[a-z0-9]+\.)*[a-z0-9]+@(?:[a-z0-9]+(?:\.|-))*[a-z0-9]+\.[a-z0-9]{2,10})\>"#),
];