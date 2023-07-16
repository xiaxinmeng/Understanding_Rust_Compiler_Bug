json
"rust-analyzer.completion.snippets": {
    "ready!": {
      "postfix": "ready",
      "body": [
        "ready!(${receiver})",
      ],
      "requires": "std::task::ready",
      "description": "ready!()",
      "scope": "expr",
    }
}
