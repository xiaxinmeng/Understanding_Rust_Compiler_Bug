javascript
    {
      "children": [],
      "code": null,
      "level": "help",
      "message": "call `Into::into` on this expression to convert `String` into `Box<dyn std::error::Error>`",
      "rendered": null,
      "spans": [
        {
          "byte_end": 85,
          "byte_start": 85,
          "column_end": 22,
          "column_start": 22,
          "expansion": null,
          "file_name": "src/lib.rs",
          "is_primary": true,
          "label": null,
          "line_end": 2,
          "line_start": 2,
          "suggested_replacement": ".into()",
          "suggestion_applicability": "MaybeIncorrect",
          "text": [
            {
              "highlight_end": 22,
              "highlight_start": 22,
              "text": "    Err(String::new())"
            }
          ]
        }
      ]
    }
