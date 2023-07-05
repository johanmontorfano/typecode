TypeCode is a mark-up language which allows to build types that can be shared
across multiple codebases, with no regards upon their programming language.

It's meant to ease the process of sharing similar types across multiple
programming languages.

⚠️ This documentation needs to be bettered.

# Roadmap

- [X] Reading files.
- [X] Interpreting files into a set of tokens.
- [X] Outputting tokens into other languages.
- [X] Checking for custom types.
- [ ] Support for optional types.
- [ ] Support for external types, as aliases, with per-language rules.
- [ ] Better the documentation.

# Transpiler installation

## From releases

Download the latest TypeCode transpiler from the GitHub releases store. Please
notice that for MacOS systems, building the transpiler from the source code is
required.

## From source code

First, clone this repository. To build an executable from the source code, you
should use `rustc` version 1.67 or later. 

# Writing types declaration in TypeCode

TypeCode types files respect a concise hierarchy. Contained inside of `module`s,
`struct`s or `enum`s contains fields. Such as:

```
module
├─ struct
│  ├─ type
│  ├─ type
├─ enum
│  ├─ constant
```

It's important to note that every name HAS TO BE WRITTEN in the `UpperCamelCase`
format to allow the transpiler to easily output names to the targeted language's
casing rules.

## Declaring a module

To declare a module, use the syntax `module UpperCamelCaseName`

## Declaring a struct/enum

To declare a struct/enum, use the syntax `[struct | enum] UpperCamelCaseName`

## Declaring a struct's type

To declare a struct's inner type, use the syntax `[type] [options] [UCCName]`

### Types and options

This section is about what are the `[type]` and `[options]` field of a struct's.

#### Types

| Type      | Description                                                                                                                  |
|-----------|------------------------------------------------------------------------------------------------------------------------------|
| `string`  | A chain of characters.                                                                                                       |
| `char`    | A single character.                                                                                                          |
| `int_u8`  | An unsigned 8-bit integer (same type as a char in some languages, but considered as an integer by the TypeScript generator.) |
| `int_u16` | An unsigned 16-bit integer.                                                                                                  |
| `int_u32` | An unsigned 32-bit integer.                                                                                                  |
| `int_u64` | An unsigned 64-bit integer.                                                                                                  |
| `int_i8`  | A 8-bit integer.                                                                                                             |
| `int_i16` | A 16-bit integer.                                                                                                            |
| `int_i32` | A 32-bit integer.                                                                                                            |
| `int_i64` | A 64-bit integer.                                                                                                            |
| `bool`    | A boolean. This type may be interpolated to another type or implementation with upcoming generators such as the C generator. |

#### Options

Multiple options can be used at the same time.

| Option     | Description                                                                        |
|------------|------------------------------------------------------------------------------------|
| `vec`      | Set the given type as being an array.                                              |
| `pointer`  | Set the type as being a pointer, this option may be removed in upcoming releases.  |
| `ref`      | Set the type as being a reference.                                                 |
| `floated`  | Set the type as being a float. It only works with numbers.                         |
| `local`    | Does required importations and magic to make a TypeCode type used as entry type.   |
| `optional` | Set the type as optional.

##### Reusability of defined types

TypeCode allows for code reusability through the `local` parameter. When setting
this parameter to a struct type, such as: 
```
mod Module
struct Example
    string Text

struct Container
    Example local Placeholder
```

The line `Example local Placeholder` tells the transpiler that the entry 
`Placeholder` is of type `Example` and, through the `local` parameter, that the
`Example` type is present within the TypeCode codebase.

It means that the transpiler will do proper imports statements depending on the
targeted programming language to ensure everything works out well.

## Declaring an enum's constant

To declare an enum's constant, use the syntax `[UpperCamelCaseName]`

## TypeCode file example

```
module EmailTypes
struct EmailIdentity
    string Email
    string Name
    string Surname
    bool IsRecipient

struct EmailData
    string Content
    EmailIdentity Recipient
    EmailIdentity Sender
    EmailIdentity vec CC
    EmailIdentity vec CCI

module BrandItems
enum Discount
    SummerDiscount
    DiscountCode10

struct PricedItem
    string Id
    int_u32 floated Price
    Discount vec ActiveDiscount
```

# Using the TypeCode transpiler

The TypeCode transpiler is waiting for commands formatted as 
`typecodet [dir] -o [file] -l [language]`, `typecodet` being an imaginary name
for the TypeCode transpiler executable.

The `[dir]` argument specify the directory where all the TypeCode files are
stored, please note that TypeCode files has to be alone in the root of a 
directory and end with the `.tc` file extension.

The `[file]` argument specify the path to the file where all the code generated
by the transpiler is saved to.

The `[language]` argument specify the language, thus generator, to use to
transpile TypeCode file content. Currently, the `rs`, `go` and `ts` languages
are available to transpilation.

## TypeCode command example

`typecodet ./common -o ./server/common_types.rs -l rs`
