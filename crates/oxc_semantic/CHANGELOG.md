# Changelog

All notable changes to this package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0).


## [0.77.2] - 2025-07-17

### 🚜 Refactor

- 3c6b88b semantic: Refactor `check_binding_identifier` (#12303) (overlookmotel)

### ⚡ Performance

- b60d63b semantic: Reduce match arms in `check_binding_identifier` (#12304) (overlookmotel)


## [0.77.1] - 2025-07-16

### 🚀 Features

- fb8289c minifier: Remove unused variable declaration (#11796) (Boshen)
- c995fe2 semantic: Build semantic for `.d.ts` files (#12193) (camc314)

### 🐛 Bug Fixes

- 089f7b1 semantic: Allow `arguments`/`eval` as name of `TSTypeAliasDeclaration`, `TSInterfaceDeclaration` (#12291) (camc314)
- 1bfd44f semantic: Allow `arguments`/`eval` as spread argument name inside `TSMethodSignature`, `TSFunctionType` (#12290) (camc314)
- a740f3f semantic: Allow `arguments`/`eval` as argument name inside `TSMethodSignature` (#12289) (camc314)
- cd98426 semantic: Handle var hoisting in catch block with same catch parameter name (#12313) (Dunqing)
- e82f758 semantic: Allow `arguments`/`eval` as argument name inside `TSFunctionType` (#12288) (camc314)
- 2a1e805 semantic: Allow assigning to `eval` and `arguments` in ambient context (#12208) (camc314)

### 🚜 Refactor

- ee761de ast: Remove `AstKind` for `AssignmentTarget` (#12252) (Tyler Earls)
- c025868 ast: Remove `AstKind` for `TSFunctionType` (#12287) (camc314)

### ⚡ Performance

- 6f58abc semantic: Inline `SemanticBuilder::pop_ast_node` (#12280) (Boshen)


## [0.77.0] - 2025-07-12

### 🐛 Bug Fixes

- e095e99 semantic: Panics when function is the part of `IfStatement` (#12190) (Dunqing)

### 🚜 Refactor

- 8a7c9e8 semantic: Streamline handling of no side effects for function (#12221) (Dunqing)
- c868ee3 semantic: Rename `AstNodeIdAncestorsIter` and add comments (#12136) (overlookmotel)
- 8814c53 ast: Remove `AstKind` for `PropertyKey` (#12108) (camchenry)
- 228cff5 semantic,linter: Assert that Program is always the first node (#12123) (Ulrich Stark)

### ⚡ Performance

- c7889c3 semantic,linter: Simplify implementation and uses of ancestors iterators (#12164) (Ulrich Stark)
- 6cfcb7e semantic: Dereference `parent_id`s to slice only once (#12137) (overlookmotel)


## [0.76.0] - 2025-07-08

### 🚜 Refactor

- e8e2a25 ast: Remove `AstKind` for `AssignmentTargetPattern` (#12105) (camchenry)
- 54cf5cb semantic: Remove Option from parent_* methods (#12087) (Ulrich Stark)

### ⚡ Performance

- 2bf1d7e semantic: Implement `FusedIterator` for `AstNodeParentIter` (#12094) (overlookmotel)


## [0.75.1] - 2025-07-03

### 🚀 Features

- c9b6513 semantic/example: Add ability to print symbols and references in example (#12017) (camchenry)

### 🐛 Bug Fixes

- 2e79d01 semantic: Panic when brackets are unmatched (#12001) (camc314)
- 404cb14 semantic: Remove needless ts type param empty check (#11899) (camc314)

### 🚜 Refactor

- 07e14a4 semantic: Remove redundant code handling reference flags (#12043) (Dunqing)
- f1d4086 ast: Remove `AstKind` for `ModuleDeclaration` (#12022) (camchenry)
- 754c05a ast: Remove `AstKind` for `TSTypeName` (#11990) (camchenry)
- d6563f8 semantic: Derive `PartialEq`, `Eq` for `JSDocCommentPart` (#12000) (camc314)
- f7a2ae4 ast: Add `AstKind` for `AssignmentTargetPropertyIdentifier`, `AssignmentTargetPropertyProperty` (#11985) (camc314)
- 54582cb ast: Add `AstKind` for `BindingProperty` (#11974) (camc314)
- 9f6784a ast: Add `AstKind` for `TSInterfaceBody` (#11967) (camc314)
- 3f50cef ast: Add `AstKind` for `TSIndexSignature` (#11966) (camc314)
- 03bce3f ast: Add `AstKind` for `TSConstructorType` (#11965) (camc314)
- 0cef370 ast: Add `AstKind::TemplateElement` (#11955) (camchenry)

### 📚 Documentation

- 9bf52f9 semantic: Add docs for `JSDocCommentPart` (#11999) (camc314)
- 5845a24 semantic: Add docs for `JSDocTagTypePart` (#11998) (camc314)
- 1a612ad semantic: Add docs for `JSDoctagTypeNamePart` (#11997) (camc314)


## [0.75.0] - 2025-06-25

### 🚜 Refactor

- 87b8496 ast: Remove `AstKind` for `MemberExpression` and replace with `StaticMemberExpression` and `PrivateFieldExpression` (#11767) (camchenry)
- 190e390 ast: Add `AstKind` for `ComputedMemberExpression` (#11766) (camchenry)


## [0.74.0] - 2025-06-23

### 💥 BREAKING CHANGES

- e81be6e semantic: [**BREAKING**] Rename `symbol_is_used` to `symbol_is_unused` (#11802) (Boshen)


## [0.73.2] - 2025-06-18

### 🚀 Features

- 4ad86ff sema/check: Check for reserved type names (#11694) (Don Isaac)

### 🚜 Refactor

- eccac91 semantic: Rename function params (#11775) (overlookmotel)


## [0.73.1] - 2025-06-17

### 🚀 Features

- 371473c ast, semantic: Implement `GetAddress` for `AstKind` and `AstNode` (#11758) (overlookmotel)
- 6095438 ast: Add AstKind to TSRestType node (#11752) (therewillbecode)
- b949ece ast: Add AstKind to TSTupleType node (#11749) (therewillbecode)
- d74c4af ast: Add AstKind to `TSTypePredicate` node (#11726) (therewillbecode)
- c25b153 ast: Add AstKind to ` TSCallSignatureDeclaration` node (#11725) (therewillbecode)
- 866470a ast: Add AstKind to `TSIndexSignature` node (#11724) (therewillbecode)

### 🚜 Refactor

- 5ca3d04 ast: Add `TSArrayType` as `AstKind` (#11745) (camchenry)
- 4fbe4b1 ast: Remove AstKind from `TSModuleReference` node (#11732) (therewillbecode)
- 219adcc ast: Don't generate AstKind for ArrayExpressionElement (#11684) (Ulrich Stark)
- b0a1561 parser: Move empty ts type parameter checks to parser (#11696) (Don Isaac)
- ff19684 semantic: Shorten Span construction (#11687) (Ulrich Stark)


## [0.73.0] - 2025-06-13

### 💥 BREAKING CHANGES

- f3eaefb ast: [**BREAKING**] Add `value` field to `BigIntLiteral` (#11564) (overlookmotel)

### 🚀 Features

- 3b03fd3 parser: Produce correct syntax error for `interface I extends (typeof T)` (#11610) (Boshen)

### 🚜 Refactor

- d41fb13 ast: Get jsx types out of AstKind exceptions (#11535) (Ulrich Stark)

### ⚡ Performance

- b34c6f6 parser,semantic: Improve handling of diagnostics (#11641) (Boshen)

### 🎨 Styling

- 24fd4bc ast: Import `Span` at top level (#11635) (overlookmotel)


# Changelog

All notable changes to this package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project does not adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) until v1.0.0.

## [0.72.0] - 2025-05-24

### Features

- 2398906 parser: Check mixed coalesce and new exponential with `preserveParens:false` (#11264) (Boshen)

### Bug Fixes

- c4f8690 parser: Fix "Enum member must have initializer." false positive (#11265) (Boshen)

## [0.71.0] - 2025-05-20

- 65a6419 mangler: [**BREAKING**] `Mangler::build_with_semantic` take mut ref to `Semantic` (#11132) (overlookmotel)

- 5d9344f rust: [**BREAKING**] Clippy avoid-breaking-exported-api = false (#11088) (Boshen)

### Performance


### Refactor

- bb8bde3 various: Update macros to use `expr` fragment specifier (#11113) (overlookmotel)

## [0.70.0] - 2025-05-15

### Bug Fixes

- 126ae75 semantic: Distinguish class private elements (#11044) (magic-akari)

## [0.69.0] - 2025-05-09

- 2b5d826 ast: [**BREAKING**] Fix field order for `TSTypeAssertion` (#10906) (overlookmotel)

- 1f35910 ast: [**BREAKING**] Fix field order for `TSNamedTupleMember` (#10905) (overlookmotel)

- 0139793 ast: [**BREAKING**] Re-order fields of `TaggedTemplateExpression` (#10889) (overlookmotel)

- cc2ed21 ast: [**BREAKING**] Fix field order for `JSXElement` and `JSXFragment` (#10881) (overlookmotel)

### Bug Fixes

- 773d0de semantic: Correctly handle nested brackets in jsdoc parsing (#10922) (camc314)
- b215b6c semantic: Dont parse `@` as jsdoc tags inside `[`/`]` (#10919) (camc314)
- 6e47699 semantic, syntax: TS value module cannot be referenced as type (#10918) (Dunqing)

### Refactor

- faf0a95 syntax: Rename `NameSpaceModule` to `NamespaceModule` (#10917) (Dunqing)

## [0.68.0] - 2025-05-03

### Bug Fixes

- d1d05d3 parser: Check comma in JSX expr lazily (#10739) (Yuji Sugiura)

### Performance

- 3de1b31 semantic: Reuse `Atom` (#10771) (overlookmotel)

## [0.67.0] - 2025-04-27

- a2b7e9d semantic: [**BREAKING**] Do not need to return old name in `Scoping::set_symbol_name` (#10613) (Dunqing)

### Features


### Performance

- 075d4b5 semantic: Optimize `Scoping::rename_symbol` method (#10612) (Dunqing)

### Refactor

- f35efd3 traverse, semantic: Move `rename_symbol` from `TraverseScoping` to `Scoping` (#10611) (Dunqing)

## [0.65.0] - 2025-04-21

- 99d82db ast: [**BREAKING**] Move `type_parameters` field to before `extends` in `TSInterfaceDeclaration` (#10476) (overlookmotel)

- 7212803 ast: [**BREAKING**] Change `TSInterfaceDeclaration::extends` from `Option<Vec>` to `Vec` (#10472) (overlookmotel)

- d6b7982 ast: [**BREAKING**] Improve pluralization of `TSClassImplements` (#10489) (overlookmotel)

### Features

- 588da69 parser: A rest parameter cannot have an initializer (#10467) (Boshen)

### Refactor


## [0.64.0] - 2025-04-17

- 771d50f ast: [**BREAKING**] Change `Class::implements` to `Vec<TSClassImplements>` (#10430) (Boshen)

- 49732ff ast: [**BREAKING**] Re-introduce `TSEnumBody` AST node (#10284) (Yuji Sugiura)

### Features

- 1a37e3f semantic: Add `SymbolFlags::Ambient` for declaration with `declare` modifier (#10355) (Dunqing)

### Bug Fixes

- dd2aef0 semantic: Incorrect `SymbolFlags` of `TSModuleDeclaration` (#10350) (Dunqing)
- c37f048 semantic: Do not declare a `Symbol` for an `ModuleDeclaration` where the `id` is not a `BindingIdentifier` (#10343) (Dunqing)

## [0.63.0] - 2025-04-08

### Bug Fixes

- f48f895 transfomer/using: Remove use of child ids (#9961) (camc314)

### Styling

- 66a0001 all: Remove unnecessary semi-colons (#10198) (overlookmotel)

## [0.62.0] - 2025-04-01

- cd1f035 semantic: [**BREAKING**] Store symbol information as the first entry in `symbol_declarations` when it is redeclared (#10062) (Dunqing)

### Features

- f6d181f semantic: Add `symbol_id` for `declare class` binding (#10095) (Dunqing)
- 9b92919 semantic: Check redeclaration of function declaration and class declaration (#10093) (Dunqing)
- 84a3490 semantic: Add `symbol_id` for declare function binding (#10078) (Dunqing)
- 370266c semantic: Check redeclaration of variable declaration and function declaration in the block scope (#10074) (Dunqing)
- b804f7c semantic: Introduce `Redeclaraion` for `Scoping::symbol_declarations` (#10059) (Dunqing)
- 528469e semantic/scoping: Add `declaration` and `flags` fields for Redeclaraion (#10060) (Dunqing)

### Bug Fixes

- 765ef4c semantic: Function declaration is not allowed in `LabeledStatement` (#10149) (Dunqing)
- be08766 semantic: False positive checking redeclaration of function in script code (#10148) (Dunqing)
- bcd9a03 semantic: Use `FormalParameter::has_modifier` to detect parameter properties (#10096) (Ulrich Stark 🦀)
- 6687f5b semantic: Define a variable in a function expression whose name is the same as the function name which is considered a redeclaration (#10077) (Dunqing)

### Performance

- 84e167e semantic: Use `FxHashMap` to replace `IndexVec` for `symbol_redeclarations` (#10058) (Dunqing)

### Documentation

- 5983760 semantic: Fix instructions for running semantic example (#10016) (overlookmotel)

### Refactor

- dc3602c semantic: Align handling of declaring symbol for `class` with TypeScript (#10094) (Dunqing)
- 5d829c2 semantic: Align handling of declaring symbol for function with TypeScript (#10086) (Dunqing)
- 0fa58d7 semantic: Always use `SymbolFlags::Function` for function id (#7479) (Dunqing)

## [0.61.2] - 2025-03-23

### Features

- b4f3d00 oxc_semantic: Make `Scoping` cloneable (#9953) (IWANABETHATGUY)

## [0.61.0] - 2025-03-20

### Features

- 38ad787 data_structures: Add `assert_unchecked!` macro (#9885) (overlookmotel)

### Bug Fixes

- 3d4c5f3 semantic: Correctly visit `IfStmt` `test` when building cfg (#9864) (camc314)

### Performance

- f7d078c semantic: Use `reserve_exact` instead of `reserve` to save memory in pre-reserve (#9910) (Dunqing)

## [0.60.0] - 2025-03-18

- b3ce925 data_structures: [**BREAKING**] Put all parts behind features (#9849) (overlookmotel)

### Features


### Bug Fixes

- bc8bc08 semantic: Use correct scope flags for using declarations (#9751) (camc314)

### Performance

- 3f808cb semantic: Speed up creating redeclarations (#9851) (overlookmotel)

### Refactor

- 78d0713 semantic: Remove dead code (#9852) (overlookmotel)

## [0.59.0] - 2025-03-18

- ce6808a parser: [**BREAKING**] Rename `type_parameters` to `type_arguments` where needed  (#9815) (hi-ogawa)

### Bug Fixes


## [0.57.0] - 2025-03-11

- b0d979d semantic: [**BREAKING**] Make `Scoping::references` private (#9629) (Boshen)

- 3c6f140 semantic: [**BREAKING**] Make `Scoping` methods consistent (#9628) (Boshen)

- ef6e0cc semantic: [**BREAKING**] Combine `SymbolTable` and `ScopeTree` into `Scoping` (#9615) (Boshen)

- 7331656 semantic: [**BREAKING**] Rename `SymbolTable` and `ScopeTree` methods (#9613) (Boshen)

- 23738bf semantic: [**BREAKING**] Introduce `Scoping` (#9611) (Boshen)

### Bug Fixes


### Refactor

- a43c341 semantic: Add `Scoping` to `Semantic` (#9614) (Boshen)

## [0.56.5] - 2025-03-07

### Refactor

- 5103322 semantic: Move `IsGlobalReference` to its own file (#9592) (Boshen)
- b7484b8 semantic: Remove `CompactStr` usage from `ClassTable` (#9591) (Boshen)

## [0.56.3] - 2025-03-07

### Features

- 6b95d25 parser: Disallow `TSInstantiationExpression` in `SimpleAssignmentTarget` (#9586) (Boshen)

## [0.56.0] - 2025-03-06

- 48a0394 ast: [**BREAKING**] Add `scope_id` to `TSFunctionType` (#9559) (camc314)

### Features


### Bug Fixes

- bbb4f98 semantic: Insert binding into correct scope for TSInferType (#9567) (camc314)

## [0.55.0] - 2025-03-05

### Features

- 803f061 minifier: Apply `__NO_SIDE_EFFECTS__` (#9533) (Boshen)

## [0.54.0] - 2025-03-04

- 098f652 codegen: [**BREAKING**] Add `CommentAnnotation` to avoid parsing comments again (#9506) (Boshen)

- a5cde10 visit_ast: [**BREAKING**] Add `oxc_visit_ast` crate (#9428) (Boshen)

### Features


### Performance


## [0.53.0] - 2025-02-26

### Refactor

- 7427900 ast: Re-order `ExportDefaultDeclaration` fields (#9348) (overlookmotel)

## [0.52.0] - 2025-02-21

### Features

- dde05e3 mangler: Opt-out of direct eval (#9191) (Boshen)

### Refactor

- 9f36181 rust: Apply `cllippy::nursery` rules (#9232) (Boshen)
- 9c3549d traverse: Apply `clippy::redundant_pub_crate` (#9258) (Boshen)

## [0.49.0] - 2025-02-10

### Refactor

- 85b8ea4 minifier: Extract `symbols().symbol_is_used(symbol_id)` (#8995) (sapphi-red)

### Styling

- a4a8e7d all: Replace `#[allow]` with `#[expect]` (#8930) (overlookmotel)

## [0.48.2] - 2025-02-02

### Features

- 256ae78 semantic: Report error for empty JSX attribute expression (#8815) (Dunqing)

### Refactor

- 6aa2dde codegen: Accept SymbolTable instead of Mangler (#8829) (Daniel Bulant)

## [0.48.1] - 2025-01-26

### Features

- 6589c3b mangler: Reuse variable names (#8562) (翠 / green)

### Refactor

- c792068 semantic: Simplify `ScopeTree::iter_bindings` (#8723) (Dunqing)

## [0.48.0] - 2025-01-24

### Bug Fixes

- d1c5dc4 semantic: Fix const assertions in `UnresolvedReferencesStack` (#8653) (overlookmotel)

### Documentation

- 5029547 semantic: Fix and reformat doc comments (#8652) (overlookmotel)

### Refactor

- a3dc4c3 crates: Clean up snapshot files (#8680) (Boshen)
- e66da9f isolated_declarations, linter, minifier, prettier, semantic, transformer: Remove unnecessary `ref` / `ref mut` syntax (#8643) (overlookmotel)
- b8d9a51 span: Deal only in owned `Atom`s (#8641) (overlookmotel)
- ac4f98e span: Derive `Copy` on `Atom` (#8596) (branchseer)

## [0.47.0] - 2025-01-18

- 4ce6329 semantic: [**BREAKING**] Ensure program outlives semantic (#8455) (Valentinas Janeiko)

### Bug Fixes


### Performance

- 30a869e semantic: Use `oxc_allocator::HashMap` in `ScopeTree` (#8554) (overlookmotel)

### Refactor

- b5ed58e span: All methods take owned `Span` (#8297) (overlookmotel)

## [0.45.0] - 2025-01-11

### Features

- 55744fd semantic: Allow getting mutable reference to symbols table (#8189) (Max Stoumen)

### Bug Fixes

- 79a8fc6 semantic: Report error for super property appearing in function body (#8376) (Dunqing)
- 79af100 semantic: Reference flags not correctly resolved when after an export stmt (#8134) (camc314)

### Refactor

- 0903501 semantic: Check super usage based on scope (#8377) (Dunqing)

## [0.44.0] - 2024-12-25

### Features

- e632a7b transformer: Remove typescript symbols after transform (#8069) (Boshen)

### Bug Fixes

- de82492 parser: Report syntax errors for missing constructor implementations (#8081) (camc314)
- 708e9cf semantic: Report errors for missing class method impls (#8082) (camc314)

## [0.43.0] - 2024-12-21

- ed75e42 semantic: [**BREAKING**] Make SymbolTable fields `pub(crate)` instead of `pub` (#7999) (Boshen)

### Features

- 8547e02 ast: Implement `allocator_api2` for `Allocator` (#8043) (Boshen)

### Performance

- 2736657 semantic: Allocate `UnresolvedReferences` in allocator (#8046) (Boshen)
- 2e8872c semantic: Allocate child scope in allocator (#8045) (Boshen)
- 414e828 semantic: Allocate symbol data in Allocator (#8012) (Boshen)
- 7aebed0 semantic: Allocate `Bindings` in allocator (#8021) (Boshen)

### Refactor

- f1adf9f semantic: `ScopeTree::rename_binding` remove old binding first (#8020) (overlookmotel)
- 02f968d semantic: Change `Bindings` to a plain `FxHashMap` (#8019) (Boshen)
- e7476a1 semantic: Remove `serialize` (#8015) (Boshen)
- 1cf7b83 semantic: Simplify handling namespace stack (#7987) (Dunqing)
- 48cb52b semantic: Remove resetting `current_reference_flags` in visit functions (#7986) (Dunqing)
- 3250a47 semantic: Remove unused current_symbol_flags (#7985) (Dunqing)
- efe96ec semantic: Use `Stack` for function stack node ids (#7984) (Dunqing)

### Testing

- bcb33c0 semantic: Add a test for catch parameters reference (#7988) (Dunqing)

## [0.42.0] - 2024-12-18

- c071494 semantic: [**BREAKING**] Remove `SymbolTable::rename` method (#7868) (overlookmotel)

### Features

- b9322c6 semantic: Re-export all flags and ID types (#7886) (overlookmotel)

### Bug Fixes

- 3659e6d cfg: Include export default code in CFG instructions (#7862) (Jan Olaf Martin)
- 14c51ff semantic: Remove inherting `ScopeFlags::Modifier` from parent scope (#7932) (Dunqing)
- 596aead semantic: Reset references flags when resolved (#7923) (Dunqing)
- 4924073 semantic: `ScopeTree::rename_binding` preserve order of bindings (#7870) (overlookmotel)

### Refactor

- 3858221 global: Sort imports (#7883) (overlookmotel)
- 6551dfe semantic: Pass `&str` instead of `Cow` (#7972) (overlookmotel)
- b8d2bd2 semantic: Move determining references flags for export specifier to `visit_export_named_declaration` (#7924) (Dunqing)
- 98d7946 semantic: Import flags and ID types from `oxc_syntax` (#7887) (overlookmotel)
- 1cf8f8f semantic: `SymbolTable::set_name` return old name (#7869) (overlookmotel)
- 5d42df8 semantic: Use `Expression::is_super` (#7851) (overlookmotel)
- 8cf9766 semantic, syntax, wasm: Remove `#![expect(non_snake_case)]` (#7863) (overlookmotel)

### Styling

- 7fb9d47 rust: `cargo +nightly fmt` (#7877) (Boshen)

### Testing

- 523d48c transformer: Move named test to exports folder (#7922) (Dunqing)

## [0.41.0] - 2024-12-13

- 96a26d3 ast: [**BREAKING**] Rename `is_strict` methods to `has_use_strict_directive` (#7783) (overlookmotel)

### Bug Fixes

- 9479e2b semantic: Missing references when `export {}` references a type-only binding and a normal (#7812) (Yunfei He)
- 7a83230 semantic: Missing reference when `export default` references a type alias binding (#7813) (Dunqing)
- 4a3bca8 semantic: Fix identifying strict mode arrow functions (#7785) (overlookmotel)

### Refactor

- 0f367e5 semantic: Improve the logic of resolving references to be cleaner (#7829) (Dunqing)
- 5710950 semantic: Move export-related reference flags logic to visit functions (#7828) (Dunqing)

## [0.40.0] - 2024-12-10

- 72eab6c parser: [**BREAKING**] Stage 3 `import source` and `import defer` (#7706) (Boshen)

- ebc80f6 ast: [**BREAKING**] Change 'raw' from &str to Option<Atom> (#7547) (Song Gao)

### Features

- 78dff7d semantic: Add `SymbolTable::symbol_is_mutated` method (#7755) (overlookmotel)

### Refactor


## [0.39.0] - 2024-12-04

- 0be5233 semantic: [**BREAKING**] Remove `ModuleRecord` from `Semantic` (#7548) (Boshen)

- 8a788b8 parser: [**BREAKING**] Build `ModuleRecord` directly in parser (#7546) (Boshen)

### Features

- d2767be semantic: Syntax error for `delete object?.#a` (#7636) (Boshen)
- 206de91 semantic: Add `SymbolTable::add_resolved_reference` method (#7608) (overlookmotel)

### Bug Fixes

- 1486849 semantic: Syntax error for undeclared private field access (#7635) (Boshen)
- f3850eb semantic: Correctly resolve binding for return type of functions (#6388) (overlookmotel)

### Refactor

- d21448b semantic, transformer: Simplify `FxIndexMap` type aliases (#7524) (overlookmotel)

### Testing

- fed8327 semantic: Add a test for `UpdateExpression` (#7495) (Dunqing)
- 4b0720a semantic: Re-organize snapshot-based tests (#7494) (Dunqing)

## [0.38.0] - 2024-11-26

- 27b2268 semantic: [**BREAKING**] Remove `SymbolFlags::Export` (#7414) (Dunqing)

- 6f0fe38 semantic: [**BREAKING**] Correct all `ReferenceFlags::Write` according to the spec (#7388) (Dunqing)

### Bug Fixes


### Refactor


## [0.37.0] - 2024-11-21

### Features

- 3dcac1a linter: React/exhaustive-deps (#7151) (camc314)
- 8cfea3c oxc_cfg: Add implicit return instruction (#5568) (IWANABETHATGUY)

### Bug Fixes

- 7d75130 transformer/async-to-generator: `arguments` isn't correct after transformation (#7234) (Dunqing)

### Refactor

- 84038ee semantic: Shorten code (#7277) (overlookmotel)
- 9e85cc1 semantic: Remove duplicated code (#7276) (overlookmotel)

### Testing

- 080a67b semantic: Semantic checker store conformance pass/fail info in a separate file (#7235) (Boshen)

## [0.36.0] - 2024-11-09

- b11ed2c ast: [**BREAKING**] Remove useless `ObjectProperty::init` field (#7220) (Boshen)

- 0e4adc1 ast: [**BREAKING**] Remove invalid expressions from `TSEnumMemberName` (#7219) (Boshen)

- d1d1874 ast: [**BREAKING**] Change `comment.span` to real position that contain `//` and `/*` (#7154) (Boshen)

- 843bce4 ast: [**BREAKING**] `IdentifierReference::reference_id` return `ReferenceId` (#7126) (overlookmotel)

### Features


### Refactor

- c5485ae semantic: Add `ancestor_kinds` iterator function (#7217) (camchenry)
- abf1602 semantic: Rename `iter_parents` to `ancestors` (#7216) (camchenry)
- 42171eb semantic: Rename `ancestors` to `ancestor_ids` (#7215) (camchenry)

## [0.35.0] - 2024-11-04

- 9a6a2f9 semantic: [**BREAKING**] Remove `SymbolTable::get_symbol_id_from_span` API (#6955) (Boshen)

### Bug Fixes

- da199c7 ecmascript: Allow getting PropName for object methods (#6967) (camchenry)

### Refactor


## [0.34.0] - 2024-10-26

### Refactor

- 423d54c rust: Remove the annoying `clippy::wildcard_imports` (#6860) (Boshen)

## [0.33.0] - 2024-10-24

- 1248557 ast: [**BREAKING**] Remove `AstKind::FinallyClause` (#6744) (Boshen)

- 202c7f6 ast: [**BREAKING**] Remove `AstKind::ExpressionArrayElement` and `AstKind::ClassHeritage` (#6740) (Boshen)

### Features

- 5928f04 semantic: Add `move_binding` API in ` ScopeTree` (#6808) (Dunqing)
- e7e60da semantic: Add `change_parent_id` API in ScopeTree (#6807) (Dunqing)
- 10484cd transformer: Class static block transform (#6733) (overlookmotel)

### Bug Fixes


### Refactor


## [0.32.0] - 2024-10-19

- 5200960 oxc: [**BREAKING**] Remove passing `Trivias` around (#6446) (Boshen)

- 2808973 ast: [**BREAKING**] Add `Program::comments` (#6445) (Boshen)

### Features

- e310e52 parser: Generate `Serialize` impls in ast_tools (#6404) (ottomated)

### Bug Fixes

- 834ee2a semantic: `TSConditionalType` scope enter/exit locations (#6351) (DonIsaac)

### Refactor

- 073b02a ast: Type params field before params in TS function declaration types (#6391) (overlookmotel)
- 435a89c oxc: Remove useless `allocator.alloc(program)` calls (#6571) (Boshen)

## [0.31.0] - 2024-10-08

- 01b878e parser: [**BREAKING**] Use `BindingIdentifier` for `namespace` declaration names (#6003) (DonIsaac)

- 95ca01c cfg: [**BREAKING**] Make BasicBlock::unreachable private (#6321) (DonIsaac)

### Features

- 14275b1 cfg: Color-code edges in CFG dot diagrams (#6314) (DonIsaac)
- 9e62396 syntax_operations: Add crate `oxc_ecmascript` (#6202) (Boshen)

### Bug Fixes

- 6159560 parser: String `ImportSpecifier`s for type imports (#6352) (DonIsaac)

### Refactor

- 40932f7 cfg: Use IndexVec for storing basic blocks (#6323) (DonIsaac)
- bdd9e92 semantic: Rename vars from `ast_node_id` to `node_id` (#6304) (overlookmotel)
- d110700 semantic: Dereference IDs as quickly as possible (#6303) (overlookmotel)

### Testing

- d4f2ee9 transformer: Tidy up transform checker (#6287) (overlookmotel)
- 0f5afd7 transformer: Transform checker output symbol name for mismatches (#6286) (overlookmotel)

## [0.30.4] - 2024-09-28

### Refactor

- 2090fce semantic: Fix lint warning in nightly (#6110) (overlookmotel)

## [0.30.3] - 2024-09-27

### Bug Fixes

- 933a743 semantic: Add interfaces and functions to `SymbolFlags::ClassExcludes`  (#6057) (DonIsaac)

## [0.30.2] - 2024-09-27

### Features

- f866781 semantic: Check for type annotations on left side of `for..in` and `for..of` iterators (#6043) (DonIsaac)
- 8b2e9aa semantic: Check for JSDoc types in TS type annotations (#6042) (DonIsaac)

### Bug Fixes

- b1af73d semantic: Do not create a `global` symbol for `declare global {}` (#6040) (DonIsaac)
- c8682e9 semantic,codegen,transformer: Handle definite `!` operator in variable declarator (#6019) (Boshen)

### Documentation

- efabfc8 semantic: Improve doc comments on `Reference` methods (#6076) (overlookmotel)

### Testing

- 93575cd semantic: Add comprehensive regression test suite (#5976) (DonIsaac)

## [0.30.1] - 2024-09-24

### Performance

- 2b17003 linter, prettier, diagnostics: Use `FxHashMap` instead of `std::collections::HashMap` (#5993) (camchenry)

### Documentation

- 1abfe8f semantic: Document `SymbolTable` (#5998) (DonIsaac)
- f5eee72 semantic: Correct docs for `Reference` (#5992) (overlookmotel)

## [0.30.0] - 2024-09-23

- c96b712 syntax: [**BREAKING**] Remove `SymbolFlags::ArrowFunction` (#5857) (overlookmotel)

### Features

- a111bb6 oxc_wasm: Add `verbse` option to `debug_dot`  (#5879) (IWANABETHATGUY)
- 74d8714 semantic: Add help message for invalid `let x?: number` (#5969) (DonIsaac)
- 3230ae5 semantic: Add `SemanticBuilder::with_excess_capacity` (#5762) (overlookmotel)
- a07f03a transformer: Sync `Program::source_type` after transform (#5887) (Boshen)

### Bug Fixes

- f1551d6 semantic: `?` on variable declaration type annotations is a syntax error (#5956) (DonIsaac)
- a23879c semantic: Analyze `ReferenceFlags` incorrectly when there are nested `AssignmentTarget` (#5847) (Dunqing)

### Performance

- c3e0fb6 semantic: Simplify resetting ReferenceFlags in `AssignmentExpression` (#5846) (Dunqing)

### Documentation

- 1ccf290 semantic: Document `AstNode` and `AstNodes` (#5872) (DonIsaac)

### Refactor

- 6dd6f7c ast: Change `Comment` struct (#5783) (Boshen)
- d910304 semantic: Rename lifetime on `impl IntoIterator for &AstNodes` (#5881) (overlookmotel)
- f360e2c semantic: Remove redundunt is_leading check for JSDoc (#5877) (leaysgur)
- 9115dd9 semantic: Use `Comment::attached_to` for jsdoc attachment (#5876) (Boshen)
- db4f16a semantic: Call `with_trivias` before `build_with_jsdoc` (#5875) (Boshen)
- 3d13c6d semantic: Impl `IntoIterator` for `&AstNodes` (#5873) (DonIsaac)
- 47d9ad8 semantic: Remove unused vars warning in release mode (#5803) (overlookmotel)

## [0.29.0] - 2024-09-13

### Features

- 805fbac oxc_cfg: Better control flow graph dot dot repr (#5731) (IWANABETHATGUY)
- f3baa49 semantic: Add `SemanticBuilder::with_stats` (#5757) (overlookmotel)
- 7fa0cb3 semantic: Expose `Stats` (#5755) (overlookmotel)

### Refactor

- 4bdc202 rust: Remove some #[expect(unused)] (#5716) (Boshen)
- a35fb14 semantic: `Stats::assert_accurate` take `self` (#5758) (overlookmotel)
- 4b896f1 semantic: Make `Stats` `Copy` (#5756) (overlookmotel)
- b4b460f semantic: `Stats` store counts as `u32` (#5754) (overlookmotel)
- 667170c semantic: Rename `Counts` to `Stats` (#5753) (overlookmotel)
- cc0408b semantic: S/AstNodeId/NodeId (#5740) (Boshen)
- 7dfcdfc semantic: Remove `more-asserts` dependency (#5739) (overlookmotel)
- 6436524 semantic: Fix dead code warning in release mode (#5728) (overlookmotel)
- e02621d semantic: Re-order use statements (#5712) (overlookmotel)
- ac6203c semantic: Move `Counts` code into counter module (#5710) (overlookmotel)
- 339fcfc semantic: Rename `Counts` in transform checker (#5709) (overlookmotel)
- d8ec781 semantic: Remove `record_ast_node` call for `Program` (#5701) (overlookmotel)

### Styling

- 1857ff0 semantic: Rename vars for node IDs (#5699) (overlookmotel)

## [0.28.0] - 2024-09-11

- 1fa3e56 semantic: [**BREAKING**] Rename `SymbolTable::iter` to `symbol_ids` (#5621) (overlookmotel)

- 96a1552 semantic: [**BREAKING**] Remove `SymbolTable::iter_rev` (#5620) (overlookmotel)

- 4a8aec1 span: [**BREAKING**] Change `SourceType::js` to `SourceType::cjs` and `SourceType::mjs` (#5606) (Boshen)

- 603817b oxc: [**BREAKING**] Add `SourceType::Unambiguous`; parse `.js` as unambiguous (#5557) (Boshen)

- b060525 semantic: [**BREAKING**] Remove `source_type` argument from `SemanticBuilder::new` (#5553) (Boshen)

### Features

- 86256ea minifier: Constant fold `typeof` (#5666) (Boshen)
- 642295c semantic: Add `SymbolTable::delete_resolved_reference` method (#5558) (overlookmotel)

### Bug Fixes

- f9e3a41 semantic: Bind `SymbolId` to function name in `if (foo) function id() {}` (#5673) (Boshen)
- 36d864a transformer/react: Don't transform if the variable does not have a value reference (#5528) (Dunqing)

### Refactor

- 0ac420d linter: Use meaningful names for diagnostic parameters (#5564) (Don Isaac)
- 731ffaa semantic: Compare nodes by pointer equality (#5686) (overlookmotel)
- 067f9b5 semantic: Introduce `IsGlobalReference` trait (#5672) (Boshen)
- d22a9b7 semantic: `SymbolTable::is_empty` use `is_empty` (#5622) (overlookmotel)

### Testing
- dc92489 Add trailing line breaks to conformance fixtures (#5541) (overlookmotel)

## [0.27.0] - 2024-09-06

- bd820f9 semantic: [**BREAKING**] Remove `SymbolTable::get_symbol_id_from_name` and `SymbolTable::get_scope_id_from_name` (#5480) (overlookmotel)

### Features

- 0f50b1e semantic: Check for initializers in ambient `VariableDeclaration`s (#5463) (DonIsaac)
- 62f7fff semantic: Check for non-declared, non-abstract object accessors without bodies (#5461) (DonIsaac)
- 5407143 semantic: Check for non-declared, non-abstract class accessors without bodies (#5460) (DonIsaac)
- 052e94c semantic: Check for parameter properties in constructor overloads (#5459) (DonIsaac)

### Bug Fixes

- 7a797ac semantic: Incorrect reference when `MemberExpression` used in `TSPropertySignature` (#5525) (Dunqing)
- d8b9909 semantic: `IdentifierReference` within `TSPropertySignature` cannot reference type-only import binding (#5441) (Dunqing)

### Refactor

- e4ed41d semantic: Change the reference flag to `ReferenceFlags::Type` if it is used within a `TSTypeQuery` (#5444) (Dunqing)

### Styling

- 2a43fa4 linter: Introduce the writing style from PR #5491 and reduce the if nesting (#5512) (dalaoshu)

### Testing

- 340b535 linter/no-unused-vars: Arrow functions in tagged templates (#5510) (Don Isaac)

## [0.26.0] - 2024-09-03

- 01cc2ce semantic: [**BREAKING**] Add `ScopeTree:get_child_ids` API behind a runtime flag (#5403) (Boshen)

- 32f7300 ast: [**BREAKING**] Add `JSXElementName::IdentifierReference` and `JSXMemberExpressionObject::IdentifierReference` (#5223) (Dunqing)

- 234a24c ast: [**BREAKING**] Merge `UsingDeclaration` into `VariableDeclaration` (#5270) (Kevin Deng 三咲智子)

- c100826 semantic: [**BREAKING**] Always create a scope for `for` statements (#5110) (overlookmotel)

- d304d6f semantic: [**BREAKING**] Always create a scope for `CatchClause` (#5109) (overlookmotel)

### Features

- c2fa725 ast,parser: Parse `TSTypeAnnotations` on `AccessorProperty` (#5179) (DonIsaac)
- be4642f semantic: Transform checker check child scope IDs (#5410) (overlookmotel)
- 6e969f9 semantic: Add `ScopeTree::delete_root_unresolved_reference` (#5305) (overlookmotel)

### Bug Fixes

- 293413f semantic: Implicit return `UpdateExpression` in `ArrowFunctionExpression` does not as read reference (#5161) (Dunqing)
- f8bb022 transformer/arrow-functions: Remove `SymbolFlags::ArrowFunction` (#5190) (Dunqing)
- d594818 traverse: `insert_scope_below` update child scopes records (#5409) (overlookmotel)

### Refactor

- 3ae94b8 semantic: Change `build_module_record` to accept &Path instead of PathBuf (Boshen)
- 05d25e2 semantic: Combine add scope methods (#5262) (overlookmotel)
- fdedc0f semantic: Transform checker: rename `SemanticData` to `Scoping` (#5261) (overlookmotel)
- 1086109 semantic: Transform checker do not output spans in errors (#5260) (overlookmotel)
- af5713e semantic: Transform checker continue checks if missing IDs (#5259) (overlookmotel)
- 943454f semantic: Update transform checker for no conditional scopes (#5252) (overlookmotel)
- 892a7fa semantic: Replace `ref` syntax (#5253) (overlookmotel)
- cbb4725 semantic: Add comment to transform checker (#5250) (overlookmotel)
- a17cf33 semantic: Remove `ScopeTree::child_ids` (#5232) (Boshen)
- d5a4940 semantic: Rewrite handling of label statement errors (#5138) (Boshen)

### Testing

- be7b8c6 semantic: Add `JSXIdentifierReference`-related tests (#5224) (Dunqing)

## [0.25.0] - 2024-08-23

- f2b8d82 semantic: [**BREAKING**] `ScopeTree::get_child_ids` + `get_child_ids_mut` return value not `Option` (#5058) (overlookmotel)

- 5f4c9ab semantic: [**BREAKING**] Rename `SymbolTable::get_flag` to `get_flags` (#5030) (overlookmotel)

- 58bf215 semantic: [**BREAKING**] Rename `Reference::flag` and `flag_mut` methods to plural (#5025) (overlookmotel)

- d262a58 syntax: [**BREAKING**] Rename `ReferenceFlag` to `ReferenceFlags` (#5023) (overlookmotel)

- c30e2e9 semantic: [**BREAKING**] `Reference::flag` method return `ReferenceFlag` (#5019) (overlookmotel)

- f88970b ast: [**BREAKING**] Change order of fields in CallExpression (#4859) (Burlin)

### Bug Fixes

- 1bd9365 coverage: Correctly check semantic data after transform (#5035) (Boshen)
- ad2be97 semantic: Incorrect semantic check for label has same name (#5041) (heygsc)
- d5de97d semantic: Transform checker check reference flags (#5092) (overlookmotel)
- 90c74ee semantic: Transform checker check reference symbol IDs (#5090) (overlookmotel)
- a8005b9 semantic: Transform checker check symbol redeclarations (#5089) (overlookmotel)
- 205bff7 semantic: Transform checker check symbol references (#5088) (overlookmotel)
- 4a57086 semantic: Transform checker check symbol IDs (#5078) (overlookmotel)
- ea7d216 semantic: Transform checker check symbol spans (#5076) (overlookmotel)
- 1b6b27a semantic: Transform checker check symbol flags (#5074) (overlookmotel)
- 6d87b0f semantic: Fix error message for duplicated label (#5071) (Boshen)
- 05fff16 semantic: Transform checker compare binding symbol IDs (#5057) (overlookmotel)
- f187b71 semantic: Transform checker compare scope children (#5056) (overlookmotel)
- b52c6a4 semantic: Transform checker compare scope parents (#5055) (overlookmotel)
- da64014 semantic: Transform checker catch more scope flags mismatches (#5054) (overlookmotel)
- 67d1a96 semantic: Transform checker compare scope flags (#5052) (overlookmotel)
- 863b9cb semantic: Transform checker handle conditional scopes (#5040) (overlookmotel)
- 47029c4 semantic: Transform checker output symbol names in errors (#5038) (overlookmotel)

### Refactor

- ca70cc7 linter, mangler, parser, semantic, transformer, traverse, wasm: Rename various `flag` vars to `flags` (#5028) (overlookmotel)
- 9da6a21 semantic: Rename transform checker output for reference symbol mismatches (#5091) (overlookmotel)
- fb46eaf semantic: Add remap functions to transform checker (#5082) (overlookmotel)
- a00bf18 semantic: Add `IdMapping` to transform checker (#5079) (overlookmotel)
- b14a302 semantic: Transform checker: change symbol name mismatch error (#5075) (overlookmotel)
- b8c6ce5 semantic: Rename vars in transform checker (#5072) (overlookmotel)
- 7156fd2 semantic: Transform checker `Pair` structure (#5053) (overlookmotel)
- 0ba6f50 semantic: Simplify raising errors in transform checker (#5051) (overlookmotel)
- ee7ac8b semantic: Store all data in `PostTransformChecker` in transform checker (#5050) (overlookmotel)
- 4e1f4ab semantic: Add `SemanticIds` to transformer checker (#5048) (overlookmotel)
- c1da574 semantic: Add comments to transformer checker (#5045) (overlookmotel)
- 8cded08 semantic: Rename error labels in transformer checker snapshots (#5044) (overlookmotel)
- 602244f semantic: Rename vars in transformer checker (#5043) (overlookmotel)
- ae94b9a semantic: Remove unused function params in transformer checker (#5042) (overlookmotel)
- 586e15c semantic: Reformat transform checker errors (#5039) (overlookmotel)
- d69e34e semantic: Fix indentation (#5037) (overlookmotel)
- 4336a32 semantic: Rename fields in snapshots from `flag` to `flags` (#5032) (overlookmotel)
- 83dfb14 semantic: Rename vars from `flag` to `flags` (#5031) (overlookmotel)
- 3b7de18 semantic: Rename `SemanticBuilder::current_reference_flags` field (#5027) (overlookmotel)
- 0bacdd8 semantic: Rename `Reference::flag` field to `flags` (#5026) (overlookmotel)
- 896b92f semantic: Correct typo in doc comment (#5009) (overlookmotel)
- d677b8e semantic: Do not reserve space in `resolved_references` (#4962) (overlookmotel)
- a7ef30d semantic: `UnresolvedReferencesStack` contain only `ReferenceId` (#4960) (overlookmotel)
- 59d15c7 semantic: `root_unresolved_references` contain only `ReferenceId` (#4959) (overlookmotel)

### Testing

- 0df1a94 semantic: Add more symbol and reference checks to `PostTransformChecker` (Boshen)

## [0.24.3] - 2024-08-18

### Features

- 80d0d1f semantic: Check for invalid interface heritage clauses (#4928) (DonIsaac)
- 48821c0 semantic,syntax: Add SymbolFlags::ArrowFunction (#4946) (DonIsaac)

### Documentation

- 0a01a47 semantic: Improve documentation (#4850) (DonIsaac)

### Refactor

- ea1e64a semantic: Make SemanticBuilder opaque (#4851) (DonIsaac)

## [0.24.0] - 2024-08-08

### Features

- fd2d9da ast: Improve `AstKind::debug_name` (#4553) (DonIsaac)
- 33f1312 semantic: Impl GetSpan for AstNode (#4717) (DonIsaac)

### Bug Fixes

- 03c643a semantic: Incorrect `scope_id` for catch parameter symbols (#4659) (Dunqing)
- 6c612d1 semantic/jsdoc: Handle whitespace absence (#4642) (leaysgur)
- 0d2c41a semantic/jsdoc: Panic on parsing `type_name_comment`. (#4632) (rzvxa)

### Refactor

- 09d9822 semantic: Simplify setting scope flags (#4674) (overlookmotel)
- 6e453db semantic: Simplify inherit scope flags from parent scope (#4664) (Dunqing)

## [0.23.1] - 2024-08-06

### Features

- fd2d9da ast: Improve `AstKind::debug_name` (#4553) (DonIsaac)

### Bug Fixes

- 03c643a semantic: Incorrect `scope_id` for catch parameter symbols (#4659) (Dunqing)
- 6c612d1 semantic/jsdoc: Handle whitespace absence (#4642) (leaysgur)
- 0d2c41a semantic/jsdoc: Panic on parsing `type_name_comment`. (#4632) (rzvxa)

### Refactor

- 09d9822 semantic: Simplify setting scope flags (#4674) (overlookmotel)
- 6e453db semantic: Simplify inherit scope flags from parent scope (#4664) (Dunqing)

## [0.23.0] - 2024-08-01

### Features

- b952942 linter: Add eslint/no-unused-vars (⭐ attempt 3.2) (#4445) (DonIsaac)
- cf1854b semantic: Remove `ReferenceFlags::Value` from non-type-only exports that referenced type binding (#4511) (Dunqing)

### Bug Fixes

- d5c4b19 parser: Fix enum member parsing (#4543) (DonIsaac)

### Refactor

- 16c7b98 semantic: Move CatchClause scope binding logic to visit_block_statement (#4505) (Dunqing)
- d6974d4 semantic: `AstNodeParentIter` fetch nodes lazily (#4533) (overlookmotel)
- d914b14 semantic: Reusing the same reference (#4529) (Dunqing)
- 7b5e1f5 semantic: Use `is_empty()` instead of `len() == 0` (#4532) (overlookmotel)
- 9db4259 semantic: Inline trivial methods (#4531) (overlookmotel)

## [0.22.1] - 2024-07-27

### Features

- 2477330 ast: Add `AstKind::TSExportAssignment` (#4501) (Dunqing)
- aaee07e ast: Add `AstKind::AssignmentTargetPattern`, `AstKind::ArrayAssignmentTarget` and `AstKind::ObjectAssignmentTarget` (#4456) (Dunqing)

### Bug Fixes

- 36bb680 semantic: `TSExportAssignment` cannot reference type binding (#4502) (Dunqing)
- cb2fa49 semantic: `typeof` operator cannot reference type-only import (#4500) (Dunqing)
- ef0e953 semantic: Generic passed to typeof not counted as a reference (#4499) (Dunqing)
- 40cafb8 semantic: Params in `export default (function() {})` flagged as `SymbolFlags::Export` (#4480) (Dunqing)
- 2e01a45 semantic: Non-exported namespace member symbols flagged as exported (#4493) (Don Isaac)
- e4ca06a semantic: Incorrect symbol’s scope_id after var hoisting (#4458) (Dunqing)
- 77bd5f1 semantic: Use correct span for namespace symbols (#4448) (Don Isaac)

### Performance

- 348c1ad semantic: Remove `span` field from `Reference` (#4464) (overlookmotel)
- 6a9f4db semantic: Reduce storage size for symbol redeclarations (#4463) (overlookmotel)

### Documentation

- 871b3d6 semantic: Add doc comments for SymbolTester and SemanticTester (#4433) (DonIsaac)

### Refactor

- ccb1835 semantic: Methods take `Span` as param, not `&Span` (#4470) (overlookmotel)
- f17254a semantic: Populate `declarations` field in `SymbolTable::create_symbol` (#4461) (overlookmotel)
- a49f491 semantic: Re-order `SymbolTable` fields (#4459) (overlookmotel)
- 7cd53f3 semantic: Var hoisting (#4379) (Dunqing)
- 4f5a7cb semantic: Mark SemanticTester and SymbolTester as must_use (#4430) (DonIsaac)
- c99b3eb syntax: Give `ScopeId` a niche (#4468) (overlookmotel)

### Testing

- 4b274a8 semantic: Add more test cases for symbol references (#4429) (DonIsaac)

## [0.22.0] - 2024-07-23

- 85a7cea semantic: [**BREAKING**] Remove name from `reference` (#4329) (Dunqing)

### Bug Fixes

- ac08de8 linter/react_perf: Allow new objects, array, fns, etc in top scope (#4395) (DonIsaac)
- bc8d4e5 semantic: Correct comment (#4410) (overlookmotel)
- 6ffce86 semantic: Align `visit_arrow_function_expression` field visit order with ast (#4366) (Dunqing)
- f8565ae transformer/typescript: Unexpectedly removed class binding from ExportNamedDeclaration (#4351) (Dunqing)

### Performance

- 1b51511 semantic: Use `Atom` instead of `CompactStr` for `UnresolvedReferencesStack` (#4401) (Dunqing)
- 40f9356 semantic: Calculate number of nodes, scopes, symbols, references before visiting AST (#4367) (Dunqing)
- da13d93 semantic: Remove bounds checks on unresolved references stack (#4390) (overlookmotel)
- e70c67b semantic: Remove a branch from `add_scope` (#4384) (overlookmotel)
- 402006f semantic: Simplify logic in `enter_scope` + `leave_scope` (#4383) (overlookmotel)
- 7469e01 semantic: Remove branch from `Nodes::add_node` (#4361) (overlookmotel)- a207923 Replace some CompactStr usages with Cows (#4377) (DonIsaac)

### Refactor

- 58f6ec2 ast: Enter node before scope (#4347) (Dunqing)
- 5d77b36 semantic: `visit_program` visit `hashbang` field (#4370) (overlookmotel)
- f7b9ada semantic: `Program` visitor leave scope before node (#4369) (overlookmotel)
- 729b288 semantic: Shorten code (#4358) (overlookmotel)
- 21d0eee semantic: Use error codes for ts diagnostics (#4336) (DonIsaac)

## [0.21.0] - 2024-07-18

- d7ab0b8 semantic: [**BREAKING**] Simplify node creation (#4226) (lucab)

### Features

- af4dc01 ast: Align ts ast scope with typescript (#4253) (Dunqing)
- 20cdb1f semantic: Align class scope with typescript (#4195) (Dunqing)
- 92ee774 semantic: Add `ScopeFlags::CatchClause` for use in CatchClause (#4205) (Dunqing)

### Bug Fixes

- 9badac0 semantic: Avoid var hosting insert the var variable to the `CatchClause` scope (#4337) (Dunqing)
- 95e15b6 semantic: Incorrect resolve references for `ExportSpecifier` (#4320) (Dunqing)
- c362bf7 semantic: Incorrect resolve references for `TSInterfaceHeritage` (#4311) (Dunqing)
- 351ecf2 semantic: Incorrect resolve references for `TSTypeQuery` (#4310) (Dunqing)
- 1108f2a semantic: Resolve references to the incorrect symbol (#4280) (Dunqing)
- 22d56bd semantic: Do not resolve references after `FormalParameters` in TS type (#4241) (overlookmotel)

### Performance

- f9d3f2e semantic: Inline ast record functions (#4272) (overlookmotel)
- 23743db semantic: Do not record ast nodes for cfg if cfg disabled (#4263) (overlookmotel)
- da69076 semantic: Reduce overhead of cfg recording ast nodes (#4262) (overlookmotel)
- cb15303 semantic: Reduce memory copies (#4216) (overlookmotel)
- ef4c1f4 semantic: Reduce lookups (#4214) (overlookmotel)
- f23e54f semantic: Recycle unresolved references hash maps (#4213) (overlookmotel)
- 2602ce2 semantic: Reuse existing map of unresolved refs (#4206) (lucab)

### Refactor

- 2c7bb9f ast: Pass final `ScopeFlags` into `visit_function` (#4283) (overlookmotel)
- 3e099fe ast: Move `enter_scope` after `visit_binding_identifier` (#4246) (Dunqing)
- aab7aaa ast/visit: Fire node events as the outermost one. (#4203) (rzvxa)
- c5731a5 semantic: Remove defunct code setting ScopeFlags twice (#4286) (overlookmotel)
- 16698bc semantic: Move function/class-specific code into specific visitors (#4278) (overlookmotel)
- ee16668 semantic: Rename function param (#4277) (overlookmotel)
- 25f0771 semantic: Alter syntax of `control_flow!` macro (#4275) (overlookmotel)
- 639fd48 semantic: Comment why extra CFG enabled check (#4274) (overlookmotel)
- c418bf5 semantic: Directly record `current_node_id` when adding a scope (#4265) (Dunqing)
- ace4f1f semantic: Update the order of `visit_function` and `Visit` fields in the builder to be consistent (#4248) (Dunqing)
- 8bfeabf semantic: Simplify adding `SymbolFlags::Export` (#4249) (Dunqing)
- dc2b3c4 semantic: Add strict mode in scope flags for class definitions (#4156) (Dunqing)
- 81ed588 semantic: Convert scope fields to IndexVecs (#4208) (lucab)
- bbe5ded semantic: Set `current_scope_id` to `scope_id` in `enter_scope` (#4193) (Dunqing)
- 7f1addd semantic: Correct scope in CatchClause (#4192) (Dunqing)
- fc0b17d syntax: Turn the `AstNodeId::dummy` into a constant field. (#4308) (rzvxa)

## [0.20.0] - 2024-07-11

- 5731e39 ast: [**BREAKING**] Store span details inside comment struct (#4132) (Luca Bruno)

### Features

- 67fe75e ast, ast_codegen: Pass the `scope_id` to the `enter_scope` event. (#4168) (rzvxa)

### Performance

- 2203143 semantic: Store unresolved refs in a stack (#4162) (lucab)
- fca9706 semantic: Faster search for leading comments (#4140) (Boshen)

### Refactor

- 03ad1e3 semantic: Tweak comment argument type (#4157) (lucab)

## [0.18.0] - 2024-07-09

### Features

- 2f53bdf semantic: Check for abstract ClassElements in non-abstract classes (#4127) (DonIsaac)
- c4ee9f8 semantic: Check for abstract initializations and implementations (#4125) (Don Isaac)

## [0.17.2] - 2024-07-08

### Features

- e386b62 semantic: Check for invalid type import assignments (#4097) (DonIsaac)

### Bug Fixes

- 0f02608 semantic: Bind `TSImportEqualsDeclaration`s (#4100) (Don Isaac)

### Performance

- 9114c8e semantic: Keep a single map of unresolved references (#4107) (Luca Bruno)

## [0.17.0] - 2024-07-05

- 1df6ac0 ast: [**BREAKING**] Rename `visit_enum_memeber` to `visit_ts_enum_member`. (#4000) (rzvxa)

- 4a0eaa0 ast: [**BREAKING**] Rename `visit_enum` to `visit_ts_enum_declaration`. (#3998) (rzvxa)

- c98d8aa ast: [**BREAKING**] Rename `visit_arrow_expression` to `visit_arrow_function_expression`. (#3995) (rzvxa)

### Refactor


## [0.16.3] - 2024-07-02

### Bug Fixes

- d995f94 semantic: Resolve reference incorrectly when a parameter references a parameter that hasn't been defined yet (#4004) (Dunqing)

## [0.16.2] - 2024-06-30

### Performance

- b234ddd semantic: Only check for jsdoc if jsdoc building is enabled (Boshen)
- 1eac3d2 semantic: Use `Atom<'a>` for `Reference`s (#3972) (Don Isaac)

## [0.16.1] - 2024-06-29

### Features

- f64ad4b semantic: Make jsdoc building optional (turned off by default) (#3955) (Boshen)

### Refactor

- 2705df9 linter: Improve diagnostic labeling (#3960) (DonIsaac)
- 15ec254 semantic: Remove the unused `Semantic::build2` function (Boshen)

## [0.16.0] - 2024-06-26

- 6796891 ast: [**BREAKING**] Rename all instances of `BigintLiteral` to `BigIntLiteral`. (#3898) (rzvxa)

- ae09a97 ast: [**BREAKING**] Remove `Modifiers` from ts nodes (#3846) (Boshen)

- 1af5ed3 ast: [**BREAKING**] Replace `Modifiers` with `declare` and `const` on `EnumDeclaration` (#3845) (Boshen)

- 0673677 ast: [**BREAKING**] Replace `Modifiers` with `declare` on `Function` (#3844) (Boshen)

- ee6ec4e ast: [**BREAKING**] Replace `Modifiers` with `declare` and `abstract` on `Class` (#3841) (Boshen)

- 9b38119 ast: [**BREAKING**] Replace `Modifiers` with `declare` on `VariableDeclaration` (#3839) (Boshen)

- 4456034 ast: [**BREAKING**] Add `IdentifierReference` to `ExportSpecifier` (#3820) (Boshen)

### Features

- d5f6aeb semantic: Check for illegal symbol modifiers (#3838) (Don Isaac)

### Bug Fixes

- 8c9fc63 semantic: Apply strict mode scope flag for strict mode TS Modules (#3861) (overlookmotel)
- 99a40ce semantic: `export default foo` should have `ExportLocalName::Default(NameSpan)` entry (#3823) (Boshen)
- 17ad8f7 transformer: Create new scopes for new blocks in TS transform (#3908) (overlookmotel)

### Performance

- 10d1de5 semantic: Remove uneccessary allocation in builder (#3867) (DonIsaac)- 4f7ff7e Do not pass `&Atom` to functions (#3818) (overlookmotel)

### Refactor

- 187f078 parser: Improve parsing of `parse_function_or_constructor_type` (#3892) (Boshen)- d6437fe Clean up some usages of `with_labels` (#3854) (Boshen)

## [0.15.0] - 2024-06-18

- 0537d29 cfg: [**BREAKING**] Move control flow to its own crate. (#3728) (rzvxa)

- 4bce59d semantic/cfg: [**BREAKING**] Re-export `petgraph` as `control_flow::graph`. (#3722) (rzvxa)

- 0578ece ast: [**BREAKING**] Remove `ExportDefaultDeclarationKind::TSEnumDeclaration` (#3666) (Dunqing)

### Features

- 046ff3f linter/eslint: Add `no_unreachable` rule. (#3238) (rzvxa)
- 9c31ed9 semantic/cfg: Propagate unreachable edges through subgraphs. (#3648) (rzvxa)
- d9c5b33 semantic/cfg: Add `Condition` instruction. (#3567) (Ali Rezvani)
- f2dfd66 semantic/cfg: Add iteration instructions. (#3566) (rzvxa)

### Bug Fixes

- 70fc69b semantic: Add Eq to CtxFlags (#3651) (Yuji Sugiura)
- 7a58fec semantic/cfg: Issue in unlabeled `Ctx`s. (#3678) (rzvxa)
- abd6ac8 semantic/cfg: Discrete finalization path after `NewFunction`s. (#3671) (rzvxa)
- e148a32 semantic/cfg: Correct unreachability propagation in try-finally. (#3667) (Ali Rezvani)

### Performance

- 2717a1a semantic/cfg: Lower the visits in `neighbors_filtered_by_edge_weight`. (#3676) (rzvxa)

### Refactor

- 7ec44f8 semantic: Rename `cfg` macro to `control_flow`. (#3742) (rzvxa)
- d8ad321 semantic: Make control flow generation optional. (#3737) (rzvxa)
- a94a72d semantic: Expose 1 checker function instead of 2 (#3694) (Boshen)
- bd8d115 semantic/cfg: Remove unused types. (#3677) (rzvxa)
- f702fb9 semantic/cfg: Cleanup control flow and it's builder. (#3650) (rzvxa)

## [0.14.0] - 2024-06-12

### Refactor

- 84304b4 linter: Add a `ctx.module_record()` method (#3637) (Boshen)
- 7d61832 semantic: Pass `Rc` by value (#3586) (overlookmotel)
- 5793ff1 transformer: Replace `&’a Trivias` with `Rc<Trivias>` (#3580) (Dunqing)
- 60cbdec traverse: `generate_uid_in_root_scope` method (#3611) (overlookmotel)

## [0.13.4] - 2024-06-07

### Bug Fixes

- c00598b transformer: JSX set `reference_id` on refs to imports (#3524) (overlookmotel)

### Refactor

- 6978269 transformer/typescript: Replace reference collector with symbols references (#3533) (Dunqing)

## [0.13.3] - 2024-06-04

### Refactor

- 7bbd3da traverse: `generate_uid` return `SymbolId` (#3520) (overlookmotel)

## [0.13.2] - 2024-06-03

### Features

- 3a5f088 linter/jsdoc: Implement require-returns rule (#3218) (Yuji Sugiura)
- bcdc658 transformer: Add `TraverseCtx::generate_uid` (#3394) (overlookmotel)

### Bug Fixes

- 5e06298 linter: Memorize visited block id in `neighbors_filtered_by_edge_weight` (#3407) (mysteryven)
- 6f71541 semantic: Set program scope_id for TS definition files (#3496) (overlookmotel)
- d4371e8 transformer: Use UIDs in TS namespace transforms (#3395) (overlookmotel)

### Refactor

- 55bbde2 ast: Move scope from `TSModuleBlock` to `TSModuleDeclaration` (#3488) (overlookmotel)
- 9c58231 semantic: Use a simpler way to resolve reference for ReferenceFlag::Type (#3430) (Dunqing)- de75fb2 Compile less test binaries to speed up CI (#3414) (Boshen)

## [0.13.1] - 2024-05-22

### Refactor

- 6f3b1c8 semantic: Semantic populate `scope_id` fields in AST (#3303) (overlookmotel)
- 78e6326 semantic/cfg: Alias petgraph's `NodeIndex` as `BasicBlockId`. (#3380) (rzvxa)

## [0.13.0] - 2024-05-14

### Features

- 44b16ef linter/eslint: Implement max-classes-per-file (#3241) (Jelle van der Waa)
- 5866086 linter/jsdoc: Implement no-defaults rule (#3098) (Yuji Sugiura)
- 1f135ce linter/react: Add the `rules_of_hooks` rule. (#3071) (rzvxa)
- c3d8a85 semantic: Report that enum member must have initializer (#3113) (Dunqing)
- 2dd96df semantic: Report namespace related errors (#3093) (Dunqing)

### Bug Fixes

- 5e36e0d semantic: Add `cfg` nodes for `ConditionalExpression`s. (#3127) (Ali Rezvani)
- c91d261 semantic: Connect `test` expression of `for` statements to the cfg. (#3122) (Ali Rezvani)
- dcb2528 semantic: Revert test code pushed to the main by accident. (#3085) (Ali Rezvani)
- 8d17ab3 semantic: Allow `root_node` to be empty for empty trees. (#3084) (Ali Rezvani)

### Refactor

- 7e1fe36 ast: Squash nested enums (#3115) (overlookmotel)
- dbde5b3 diagnostics: Remove export of `miette` (Boshen)
- 312f74b diagnostics: S/OxcDiagnostic::new/OxcDiagnostic::error (Boshen)
- c5588c9 semantic: Clean up redeclaration diagnostic (Boshen)
- 09f34fc semantic: Unify diagnostic in checker (Boshen)
- a8af5de syntax: Move number related functions to number module (#3130) (Boshen)- 893af23 Clean up more diagnostics usages (Boshen)- d8173e1 Remove all usages of `Into<Error>` (Boshen)

## [0.12.5] - 2024-04-22

### Features

- 92d709b ast: Add `CatchParameter` node (#3049) (Boshen)
- 57ad6c4 semantic: Add root node to the `AstNodes` structure. (#3032) (Ali Rezvani)

### Bug Fixes

- 84c43c8 semantic: Correctly resolve identifiers inside catch parameter initializers (#3050) (Boshen)
- 1f7033e semantic: Correctly resolve identifiers inside parameter initializers (#3046) (Boshen)

## [0.12.4] - 2024-04-19

### Features

- 40af2b1 semantic/jsdoc: Handle optional type syntax for type name part (#2960) (Yuji Sugiura)

### Bug Fixes

- 2c325ef semantic/jsdoc: Skip parsing `@` inside of backticks (#3017) (Yuji Sugiura)

## [0.12.3] - 2024-04-11

### Refactor

- 0a77d62 semantic/jsdoc: Rework JSDoc struct for better Span handling (#2917) (Yuji Sugiura)

## [0.12.2] - 2024-04-08

### Features

- aa63b64 linter: Implement jsdoc/check-access (#2642) (Yuji Sugiura)

### Bug Fixes

- 1ea24ea semantic: Symbols inside functions and classes incorrectly flagged as exported (#2896) (Don Isaac)

## [0.12.1] - 2024-04-03

### Bug Fixes

- d3eb1c3 semantic: Flag function expressions with `SymbolFlags::Function` (#2891) (Don Isaac)

## [0.11.0] - 2024-03-30

### Features

- 712b3d2 semantic: Distinguish type imports in ModuleRecord (#2785) (Dunqing)
- df744b2 semantic/jsdoc: Add `Span` for JSDoc, JSDocTag (#2815) (Yuji Sugiura)

### Bug Fixes

- df62828 linter/import: Ignore export declaration in no-duplicates (#2863) (Dunqing)
- 947a9f0 semantic: Missing SymbolFlags::Export when identifier used in ExportDefaultDeclaration (#2837) (Dunqing)
- b28b617 semantic: Incorrect ExportEntry span for ExportAllDeclaration in ModuleRecord (#2793) (Dunqing)
- b6e493b semantic: ModuleRecord's indirect_export_entires missing reexported imports (#2792) (Dunqing)

### Refactor

- 1b5e544 semantic: Distinguish whether requested_modules is type imports/exports (#2848) (Dunqing)
- 4a42c5f semantic/jsdoc: JSDocTag parser rework (#2765) (Yuji Sugiura)

## [0.10.0] - 2024-03-14

### Features

- 57ce737 semantic: Move redeclare varaibles to symbol table (#2614) (Dunqing)
- 4f9dd98 span: Remove `From<String>` and `From<Cow>` API because they create memory leak (#2628) (Boshen)- 697b6b7 Merge features `serde` and `wasm` to `serialize` (#2716) (Boshen)

### Bug Fixes

- b00d4b8 semantic/jsdoc: Support multibyte chars (#2694) (Yuji Sugiura)
- 2609e90 semantic/jsdoc: Fix up builder (#2623) (Yuji Sugiura)

### Refactor

- 0f86333 ast: Refactor `Trivias` API - have less noise around it (#2692) (Boshen)
- cba1e2f ast: Import `Tsify` to shorten code (#2665) (overlookmotel)
- 6b5723c ast: Shorten manual TS defs (#2638) (overlookmotel)- cbc2f5f Remove unused dependencies (#2718) (Boshen)- 3c1e0db Reduce `cfg_attr` boilerplate with `SerAttrs` derive (#2669) (overlookmotel)- d76ee6b "wasm" feature enable "serde" feature (#2639) (overlookmotel)- 8001b2f Make `CompactStr` immutable (#2620) (overlookmotel)- 0646bf3 Rename `CompactString` to `CompactStr` (#2619) (overlookmotel)

## [0.9.0] - 2024-03-05

### Features

- d41dcc3 linter: Remove all commonjs logic for import plugin (#2537) (Boshen)
- f760108 transformer: Call build module record (#2529) (Dunqing)

### Bug Fixes

- 37de80d semantic: Jsx reference with an incorrect node id (#2546) (Dunqing)
- 1519b90 semantic: Incorrect scope for switch statement (#2513) (Dunqing)

### Refactor

- 1391e4a semantic/jsdoc: Misc fixes for JSDoc related things (#2531) (Yuji Sugiura)- c56b6cb Replace InlinableString with CompactString for `Atom` (#2517) (Boshen)

## [0.8.0] - 2024-02-26

### Features

- f5aadc7 linter: Handle cjs `module.exports = {} as default export (#2493) (Boshen)
- f64c7e0 linter: Handle cjs `module.exports.foo = bar` and `exports.foo = bar` (#2492) (Boshen)
- d0a9c46 linter: Handle top-level `require` for import plugin (#2491) (Boshen)
- 197fa16 semantic: Add check for duplicate class elements in checker (#2455) (Dunqing)
- 950298d semantic: Add static property, ElementKind::Getter, ElementKind::Setter in ClassTable (#2445) (Dunqing)

### Bug Fixes

- fba66dc linter: Improve import/no-named-as-default (#2494) (Boshen)
- 4c2e2bd semantic: Add export symbol flag to identifiers in export declarations (#2508) (Dunqing)
- 04f4621 semantic: Should return nearest JSDoc (#2490) (Yuji Sugiura)
- bc22ae5 semantic: Refactor jsdoc finding (#2437) (Yuji Sugiura)
- 5bd2ce6 semantic: Incorrect reference flag for MemberExpression assign (#2433) (Dunqing)

### Performance

- 8110288 semantic: Reduce visit parent nodes in resolve_reference_usages (#2419) (Dunqing)

### Refactor

- d08abc6 ast: S/NumberLiteral/NumericLiteral to align with estree (Boshen)
- e6b391a ast: S/ArrowExpression/ArrowFunctionExpression to align estree (Boshen)
- 7c2d868 semantic: Delete the redundant code in binder (#2423) (Dunqing)
- c6767fa semantic: Reduce allocation in resolve_references_for_current_scope (#2414) (Dunqing)
- 28ba28f semantic: Check directive by current_scope_id (#2411) (Dunqing)- a2c173d Remove `panic!` from examples (#2454) (Boshen)

## [0.7.0] - 2024-02-09

### Features

- d571839 ast: Enter AstKind::ExportDefaultDeclaration, AstKind::ExportNamedDeclaration and AstKind::ExportAllDeclaration (#2317) (Dunqing)
- 40e9541 semantic: Add export binding for ExportDefaultDeclarations in module record (#2329) (Dunqing)
- a3570d4 semantic: Report parameter related errors for setter/getter (#2316) (Dunqing)
- 9ca13d0 semantic: Report type parameter list cannot be empty (#2315) (Dunqing)
- f53c54c semantic: Report unexpected type annotation in ArrayPattern (#2309) (Dunqing)
- f3035f1 semantic: Apply ImportSpecifier's binder and remove ModuleDeclaration's binder (#2307) (Dunqing)
- 6002560 span: Fix memory leak by implementing inlineable string for oxc_allocator (#2294) (Boshen)

### Bug Fixes

- 540b2a0 semantic: Remove unnecessary SymbolFlags::Import (#2311) (Dunqing)
- cb17a83 semantic: Remove ignore cases (#2300) (Dunqing)

## [0.6.0] - 2024-02-03

### Features

- f673e41 ast: Remove serde skip for symbol_id and reference_id (#2220) (Dunqing)
- cd5026c ast: TypeScript definition for wasm target (#2158) (Nicholas Roberts)
- 721a869 linter: Improve no_redeclare rule implementation (#2084) (Dunqing)
- 2768195 oxc_semantic: Improve sample visualization (#2251) (Tzvi Melamed)
- 28daf83 semantic: Report no class name error (#2273) (Boshen)
- da2ffdf semantic: Check parameters property (#2264) (Dunqing)
- d71175e semantic: Check optional parameters (#2263) (Dunqing)
- 8d99a15 semantic: Report error on optional variable declaration in TypeScript (#2261) (Boshen)
- e561457 semantic: Track cfg index per ast node (#2210) (Tzvi Melamed)
- 8898377 semantic: Cfg prototype (#2019) (Boshen)
- ead4e8d transformer/typescript: Remove import if only have type reference (#2001) (Dunqing)

### Bug Fixes

- 989ab88 codegen: Print `Directive` original string (#2157) (underfin)
- f4674f3 oxc_semantic: Handle short-circuiting operators in CFG (#2252) (Tzvi Melamed)
- 73ccf8a oxc_semantic: Proper traversal of try statements (#2250) (Tzvi Melamed)
- 972be83 semantic: Fix incorrect semantic example (#2198) (Dunqing)
- 122abd5 semantic: Replace ClassStatickBlockAwait with ClassStaticBlockAwait (#2179) (Dunqing)
- 24ac957 semantic: Incorrect reference flag (#2057) (Dunqing)

### Refactor

- 766ca63 ast: Rename RestElement to BindingRestElement (#2116) (Dunqing)
- 2924258 semantic: Adding binder for ImportSpecifier replaces the ModuleDeclaration's binder (#2230) (Dunqing)
- c62495d semantic: Get function by scope_id in set_function_node_flag (#2208) (Dunqing)
- f59e87f semantic: Checking label in ContinueStatement based on LabelBuilder (#2202) (Dunqing)
- 56adfb1 semantic: Use LabelBuilder instead of UnusedLabeled (#2184) (Dunqing)
- fc1592b semantic: Remove all #[dead_code[ from tester (Boshen)
- 8bccdab semantic: Add binder for FormalParameters and RestElement, replacing the binder for FormalParameters (#2114) (Dunqing)
- 8e43eef semantic: Improve declare symbol logic in FormalParameters (#2088) (Dunqing)- 87b9978 Move all miette usages to `oxc_diagnostics` (Boshen)

## [0.5.0] - 2024-01-12

### Features

- f45a3cc linter: Support eslint/no-unused-private-class-members rule (#1820) (Dunqing)
- f1b433b playground: Visualize symbol (#1886) (Dunqing)
- 45a7985 playground: Visualize scope (#1882) (Dunqing)
- 3b4fe0e semantic: Allow reserved keyword defined in ts module block (#1907) (Dunqing)
- b0569bc semantic: Add current_scope_flags function in SemanticBuilder (#1906) (Dunqing)
- b9bdf36 semantic: Improve check super implementation, reduce access nodes (#1827) (Dunqing)
- f7b7f0a semantic: Support get node id by scope id (#1826) (Dunqing)
- ca04312 semantic: Add ClassTable (#1793) (Dunqing)
- edc6fa4 semantic: Add SymbolFlags::Function for FunctionDeclaration (#1713) (Dunqing)
- 78b427b transform: Support es2015 new target (#1967) (underfin)

### Bug Fixes

- 9c9d882 semantic: Remove duplicate errors in ModuleDeclaration::ImportDeclaration (#1846) (Dunqing)

### Performance

- 0080638 linter/react: Find class node by symbols in get_parent_es6_component (#1657) (Dunqing)
- dae5f62 semantic: Check duplicate parameters in Binder of FormalParameters (#1840) (Dunqing)
- a743d06 semantic: Just need to find the AstKind::FormalParameter in is_in_formal_parameters (#1852) (Dunqing)
- 0e0f258 semantic: Reduce calls to span() (#1851) (Dunqing)

### Refactor

- 6c5b22f semantic: Improve ClassTable implmention and merge properties and methods to elements (#1902) (Dunqing)
- bfd5cd9 semantic: Improve check function declaration implementation (#1854) (Dunqing)
- 497a0b8 semantic: Rename `add_node_id` to `add_current_node_id_to_current_scope` (#1847) (Dunqing)
- d63c50a semantic: Improve check private identifier implementation (#1794) (Dunqing)
- da67fe1 semantic: Remove unused methods from `AstNode` (Boshen)

## [0.4.0] - 2023-12-08

### Features

- 446ba16 ast: Add to_string function to VariableDelcartionKind (#1303) (Dunqing)
- 0115314 ast/semantic: Parse jsdoc on `PropertyDefinition` (#1517) (Shannon Rothe)
- 5f31662 prettier: Add the basics of comment printing (#1313) (Boshen)
- c6ad660 semantic: Support scope descendents starting from a certain scope. (#1629) (Miles Johnson)

### Refactor

- be043c3 ast: VariableDeclarationKind::to_string -> as_str (#1321) (Boshen)
- 1a576f6 rust: Move to workspace lint table (#1444) (Boshen)

## [0.3.0] - 2023-11-06

### Features

- 2453954 linter: Add no-redeclare rule. (#683) (cin)
- ef8aaa7 minifier: Re-enable mangler (#972) (Boshen)
- 55b2f03 minifier: Partially re-enable minifier (#963) (Boshen)
- a442fad semantic: Bind function expression name (#1049) (Boshen)
- 1661385 semantic: Check non-simple lhs expression of assignment expression (#994) (Boshen)
- af1a76b transformer: Implement some of needs_explicit_esm for typescript (#1047) (Boshen)
- dfee853 transformer: Add utils to make logical_assignment_operators pass (#1017) (Boshen)
- 678db1d transformer: ES2020 Nullish Coalescing Operator (#1004) (Boshen)
- 0f72066 transformer: Finish 2016 exponentiation operator (#996) (Boshen)
- 203cf37 transformer/react: Read comment pragma @jsxRuntime classic / automatic (#1133) (Boshen)

### Bug Fixes

- 0f02d37 semantic: Make ExportDeclaration span accurate (#928) (Wenzhe Wang)

### Refactor

- 903854d ast: Fix the lifetime annotations around Vist and VisitMut (#973) (Boshen)
- 69150d8 transformer: Move Semantic into Transformer (#1130) (Boshen)

### Testing

- b4b39b8 semantic: Add scoping test cases (#954) (Don Isaac)

## [0.2.0] - 2023-09-14

### Features

- e7c2313 ast: Add `SymbolId` and `ReferenceId` (#755) (Yunfei He)
- 4e5f63a linter: Implement re-exports (#877) (Boshen)
- ee54575 linter: Add runner for import-plugin (#858) (Boshen)
- c5ff534 semantic: Add `node_id` to `Reference` (#689) (Makoto Tateno)
- 75d928a syntax: Add loaded_modules to ModuleRecord (Boshen)

### Bug Fixes

- 2f48bdf parser,semantic: Make semantic own `Trivias` (#711) (Boshen)
- 815db57 semantic: Symbol of identifier of top level function declaration should be in the root scope (#843) (Yunfei He)
- d3accc1 semantic: Nested references (#661) (Don Isaac)

### Performance

- babbc47 parser: Lazily build trivia map instead of build in-place (#903) (Boshen)

### Testing

- 38fb4c2 semantic: Test harness (#679) (Don Isaac)

