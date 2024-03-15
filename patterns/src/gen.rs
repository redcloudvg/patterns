pub const PATTERNS: &'static [(&'static str, &'static str)] = &[
("aws::secret_access_key", r#"\<([0-9a-zA-Z+/]{40})\>"#),
("aws::access_key_id", r#"\<((?:A3T[A-Z0-9]|AKIA|AGPA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16})\>"#),
("pypi::api_key", r#"\<(pypi-AgEIcHlwaS5vcmc[0-9a-zA-Z_\-]{50,200})\>"#),
("aliyun::access_key_secret", r#"\<([0-9a-zA-Z]{30})\>"#),
("aliyun::access_key_id", r#"\<(LTAI[0-9a-zA-Z]{20})\>"#),
("sendgrid::api_key", r#"\<(SG\.[0-9a-zA-Z_\-]{22}\.[0-9a-zA-Z_\-]{43})\>"#),
("telegram::bot_token", r#"\<(\d+:AA[0-9a-zA-Z_\-]{32,33})\>"#),
("_::email_address", r#"\<((?:[a-z0-9]+\.)*[a-z0-9]+@(?:[a-z0-9]+(?:\.|-))*[a-z0-9]+\.[a-z0-9]{2,10})\>"#),
];