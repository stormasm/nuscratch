
```
alias ctno='cargo test -- --nocapture'
```

cd crates/nu-parser/src

## Working version of what LiteBlock should look like...

https://github.com/stormasm/nushell/blob/test_parenthesis_in_lex/crates/nu-parser/src/lex.rs#L921

```
ctno command_on_one
```

[

 * Token { contents: EOL, span: Span { start: 0, end: 1 } },
 * Token { contents: Baseline("def"), span: Span { start: 1, end: 4 } },
 * Token { contents: Baseline("my_echo"), span: Span { start: 5, end: 12 } },
 * Token { contents: Baseline("[arg]"), span: Span { start: 13, end: 18 } },
 * Token { contents: Baseline("{ echo $arg }"), span: Span { start: 19, end: 32 } },
 * Token { contents: EOL, span: Span { start: 32, end: 33 } }   

]

LiteBlock { block: [LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 1, end: 4 },
 * item: "def" }, Spanned { span: Span { start: 5, end: 12 },
 * item: "my_echo" }, Spanned { span: Span { start: 13, end: 18 },
 * item: "[arg]" }, Spanned { span: Span { start: 19, end: 32 },
 * item: "{ echo $arg }" }], comments: None }] }] }] }

```
ctno command_on_two
```

 [

* Token { contents: EOL, span: Span { start: 0, end: 1 } },
* Token { contents: Baseline("def"), span: Span { start: 1, end: 4 } },
* Token { contents: Baseline("my_echo"), span: Span { start: 5, end: 12 } },
* Token { contents: Baseline("[arg]"), span: Span { start: 13, end: 18 } },
* Token { contents: Baseline("{\necho $arg }"), span: Span { start: 19, end: 32 } },
* Token { contents: EOL, span: Span { start: 32, end: 33 } }

 ]

 LiteBlock { block: [LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 1, end: 4 },
* item: "def" }, Spanned { span: Span { start: 5, end: 12 },
* item: "my_echo" }, Spanned { span: Span { start: 13, end: 18 },
* item: "[arg]" }, Spanned { span: Span { start: 19, end: 32 },
* item: "{\necho $arg }" }], comments: None }] }] }] }

```
ctno command_on_three
```

[

* Token { contents: EOL, span: Span { start: 0, end: 1 } },
* Token { contents: Baseline("def"), span: Span { start: 1, end: 4 } },
* Token { contents: Baseline("my_echo"), span: Span { start: 5, end: 12 } },
* Token { contents: Baseline("[arg]"), span: Span { start: 13, end: 18 } },
* Token { contents: Baseline("{\necho $arg\n}"), span: Span { start: 19, end: 32 } },
* Token { contents: EOL, span: Span { start: 32, end: 33 } }

]

LiteBlock { block: [LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 1, end: 4 },
* item: "def" }, Spanned { span: Span { start: 5, end: 12 },
* item: "my_echo" }, Spanned { span: Span { start: 13, end: 18 },
* item: "[arg]" }, Spanned { span: Span { start: 19, end: 32 },
* item: "{\necho $arg\n}" }], comments: None }] }] }] }


## Broken LiteBlock for left parens on a new line

```
let code = r#"
def my_echo [arg]
{ echo $arg }
"#;
```

as you can see it throws off a 2nd LiteGroup incorrectly for

```
"{ echo $arg }"
```

```
ctno parens_broken
```

[

* Token { contents: EOL, span: Span { start: 0, end: 1 } },
* Token { contents: Baseline("def"), span: Span { start: 1, end: 4 } },
* Token { contents: Baseline("my_echo"), span: Span { start: 5, end: 12 } },
* Token { contents: Baseline("[arg]"), span: Span { start: 13, end: 18 } },
* Token { contents: EOL, span: Span { start: 18, end: 19 } },
* Token { contents: Baseline("{ echo $arg }"), span: Span { start: 19, end: 32 } },
* Token { contents: EOL, span: Span { start: 32, end: 33 } }]

LiteBlock { block: [LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 1, end: 4 },
* item: "def" }, Spanned { span: Span { start: 5, end: 12 },
* item: "my_echo" }, Spanned { span: Span { start: 13, end: 18 },
* item: "[arg]" }], comments: None }] }] },

LiteGroup { pipelines: [LitePipeline { commands: [LiteCommand { parts: [Spanned { span: Span { start: 19, end: 32 },
* item: "{ echo $arg }" }], comments: None }] }] }] }
