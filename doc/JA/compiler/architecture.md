# Architecture of `ergc`

## 1. Scan an Erg script (.er) and generate a `TokenStream` (parser/lex.rs)

* parser/lexer/Lexer generates `TokenStream` (this is an iterator of Token, TokenStream can be generated by lexer.collect())
  * `Lexer` is constructed from `Lexer::new` or `Lexer::from_str`, where `Lexer::new` reads the code from a file or command option.
  * `Lexer` can generate tokens sequentially as an iterator; if you want to get a `TokenStream` all at once, use `Lexer::lex`.
  * `Lexer` outputs `LexError`s as errors, but `LexError` does not have enough information to display itself. If you want to display the error, use the `LexerRunner` to convert the error.
  * `LexerRunner` can also be used if you want to use `Lexer` as standalone; `Lexer` is just an iterator and does not implement the `Runnable` trait.
    * `Runnable` is implemented by `LexerRunner`, `ParserRunner`, `Compiler`, and `VirtualMachine`.

## 2. Convert `TokenStream` -> `AST` (parser/parse.rs)

* `Parser`, like `Lexer`, has two constructors, `Parser::new` and `Parser::from_str`, and `Parser::parse` will give the `AST`.
* `AST` is the wrapper type of `Vec<Expr>`. It is for "Abstract Syntax Tree".

### 2.5 Desugaring `AST`

* expand nested vars (`Desugarer::desugar_nest_vars_pattern`)
* desugar multiple pattern definition syntax (`Desugarer::desugar_multiple_pattern_def`)

## 3. Type checking & inference, Convert `AST` -> `HIR` (compiler/lower.rs)

* `HIR` has every variable's type information. It is for "High-level Intermediate Representation".
* `HIR` only holds the type of the variable, but that's enough. In extreme cases, this is because Erg has only conversion (or operator) applications. If we know the type of the conversion, we have already know the type of the object of the argument.
* `ASTLowerer` can be constructed in the same way as `Parser` and `Lexer`.
* `ASTLowerer::lower` will output a tuple of `HIR` and `CompileWarnings` if no errors occur.
* `ASTLowerer` is owned by `Compiler`. Unlike conventional structures, `ASTLowerer` handles code contexts and is not a one-time disposable.
* If the result of type inference is incomplete (if there is an unknown type variable), an error will occur during name resolution.

## 4. Check side-effects (compiler/effectcheck.rs)

## 4. Check ownerships (compiler/memcheck.rs)

## 5. Generate Bytecode (`CodeObj`) from `HIR` (compiler/codegen.rs)

* From the type information of the expression, name resolution of the quantified subroutines will be performed.

## (6. (Future plans) Convert Bytecode -> LLVM IR)

* Bytecode is stack-based, whereas LLVM IR is register-based.
  There will be several more layers of intermediate processes for this conversion process.
