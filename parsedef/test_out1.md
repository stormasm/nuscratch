
alias ctno='cargo test -- --nocapture'

cd crates/nu-parser/src

```
ctno parens_on_one
```

[
 * Token { contents: EOL, span: Span { start: 0, end: 1 } },
 * Token { contents: Baseline("def"), span: Span { start: 1, end: 4 } },
 * Token { contents: Baseline("my_echo"), span: Span { start: 5, end: 12 } },
 * Token { contents: Baseline("[arg]"), span: Span { start: 13, end: 18 } },
 * Token { contents: Baseline("{ echo $arg }"), span: Span { start: 19, end: 32 } },
 * Token { contents: EOL, span: Span { start: 32, end: 33 } }]

LiteBlock { block: [LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 1, end: 4 },
 * item: "def" }, Spanned { span: Span { start: 5, end: 12 },
 * item: "my_echo" }, Spanned { span: Span { start: 13, end: 18 },
 * item: "[arg]" }, Spanned { span: Span { start: 19, end: 32 },
 * item: "{ echo $arg }" }], comments: None }] }] }] }
