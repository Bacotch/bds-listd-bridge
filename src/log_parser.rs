static ACTION_MESSAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(ACTION_MESSAGE_PREFIX).expect("Failed to init ACTION_MESSAGE_REGEX"));
