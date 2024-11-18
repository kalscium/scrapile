/**
 * @file A somewhat fully-fleged rust-like programming language that, surprisingly, compiles to scratch
 * @author kalscium <kalscium@protonmail.com>
 * @license GPL-3.0
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "scrapile",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
