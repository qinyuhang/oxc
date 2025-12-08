use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::Error,
};
use oxc_macros::declare_oxc_lint;
use oxc_span::{Atom, Span};
use oxc_syntax::module_record::ModuleRecord;

use crate::{context::LintContext, rule::Rule};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint-plugin-import(no-cycle): Dependency cycle detected")]
#[diagnostic(severity(warning), help("'{1}' imported multiple times."))]
struct NoDuplicatesDiagnostic(#[label] Span, String);

/// <https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-duplicates.md>
#[derive(Debug, Default, Clone)]
pub struct NoDuplicates;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Ensures that there is no resolvable path back to this module via its dependencies.
    ///
    /// This includes cycles of depth 1 (imported module imports me) to "∞" (or Infinity),
    /// if the maxDepth option is not set.
    ///
    /// ### Why is this bad?
    ///
    /// Dependency cycles lead to confusing architectures where bugs become hard to find.
    ///
    /// It is common to import an `undefined` value that is caused by a cyclic dependency.
    ///
    /// ### Example
    /// ```javascript
    /// // dep-b.js
    /// import './dep-a.js'
    /// export function b() { /* ... */ }
    /// ```
    ///
    /// ```javascript
    /// // dep-a.js
    /// import { b } from './dep-b.js' // reported: Dependency cycle detected.
    /// ```
    NoDuplicates,
    nursery
);

impl Rule for NoDuplicates {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}


#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r#"import "./malformed.js""#,
        "import { x } from './foo'; import { y } from './bar'",
        r#"import foo from "234artaf"; import { shoop } from "234q25ad""#,
        r#"import { x } from './foo'; import type { y } from './foo'""#, // parser: parsers.BABEL_OLD,
        "import x from './bar?optionX'; import y from './bar?optionY';", // options: [{ considerQueryString: true }], settings: { 'import/resolver': 'webpack' },
        "import x from './foo'; import y from './bar';", // options: [{ considerQueryString: true }], settings: { 'import/resolver': 'webpack' },
        "import * as ns from './foo'; import {y} from './foo'",
        "import {y} from './foo'; import * as ns from './foo'",
    ];

    let fail = vec![
        "import {x} from './foo'; import {y} from './foo'; import { z } from './foo'", // fix import {x,y, z } from './foo';  
        "import { x } from './bar'; import { y } from 'bar';", // fix import { x , y } from './bar'; settings: { 'import/resolve': { paths: [path.join(process.cwd(), 'tests', 'files')],},},
        "import x from './bar.js?optionX'; import y from './bar?optionX';", // settings: { 'import/resolver': 'webpack' },
        "import x from './bar?optionX'; import y from './bar?optionY';", // settings: { 'import/resolver': 'webpack' },
        "import x from './bar?optionX'; import y from './bar.js?optionX';", // options: [{ considerQueryString: true }], settings: { 'import/resolver': 'webpack' },
        "import { foo } from './es6/depth-three-indirect'",
        "import { foo } from './intermediate-ignore'",
        "import { foo } from './ignore'",
    ];

    Tester::new_without_config(NoDuplicates::NAME, pass, fail)
        .change_rule_path("cycles/depth-zero.js")
        .with_import_plugin(true)
        .test_and_snapshot();
}
