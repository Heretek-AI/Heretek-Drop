module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      [
        "feat", // New feature
        "fix", // Bug fix
        "docs", // Documentation only
        "style", // Formatting, no code change
        "refactor", // Code change that neither fixes a bug nor adds a feature
        "perf", // Performance
        "test", // Add or fix tests
        "build", // Build system / dependencies
        "ci", // CI configuration
        "chore", // Other (no src/test changes)
        "revert", // Revert a previous commit
      ],
    ],
    "scope-enum": [
      2,
      "always",
      [
        "desktop",
        "protocol",
        "auth",
        "db",
        "download",
        "process",
        "ui",
        "lint",
        "deps",
        "release",
        "ci",
        "agents",
        "skills",
        "docs",
      ],
    ],
    "subject-case": [2, "always", "lower-case"],
    "subject-max-length": [2, "always", 72],
    "body-leading-blank": [2, "always"],
    "footer-leading-blank": [2, "always"],
  },
};
