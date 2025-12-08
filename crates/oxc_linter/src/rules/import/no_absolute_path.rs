use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::Error,
};
use oxc_macros::declare_oxc_lint;
use oxc_span::{Span, Atom};

use crate::{context::LintContext, rule::Rule};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint-plugin-import(no-absolute-path): module importing itself is not allowed")]
#[diagnostic(severity(warning), help("Do not import modules using an absolute path"))]
struct NoAbsolutePathDiagnostic(#[label] pub Span, Atom);

#[derive(Debug, Default, Clone)]
pub struct NoAbsolutePath;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Forbid import of modules using absolute paths.
    ///
    /// ### Bad Example
    ///
    /// ```javascript
    /// // foo.js
    /// import foo from '/foo.js'
    /// const foo = require('/foo')
    /// ```
    NoAbsolutePath,
    nursery
);

impl Rule for NoAbsolutePath {
    fn run_once(&self, ctx: &LintContext<'_>) {
        // let module_record = ctx.semantic().module_record();
        // let resolved_absolute_path = &module_record.resolved_absolute_path;
        // for (request, spans) in &module_record.requested_modules {
        //     let Some(remote_module_record_ref) = module_record.loaded_modules.get(request) else {
        //         continue;
        //     };
        //     if remote_module_record_ref.value().resolved_absolute_path == *resolved_absolute_path {
        //         for span in spans {
        //             ctx.diagnostic(NoAbsolutePathDiagnostic(*span));
        //         }
        //     }
        // }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "import _ from 'lodash'",
        "import find from 'lodash.find'",
        "import foo from './foo'",
        "import foo from '../foo'",
        "import foo from 'foo'",
        "import foo from './'",
        "import foo from '@scope/foo'",
        "var _ = require('lodash')",
        "var find = require('lodash.find')",
        "var foo = require('./foo')",
        "var foo = require('../foo')",
        "var foo = require('foo')",
        "var foo = require('./')",
        "var foo = require('@scope/foo')",
        "var bar = require('./bar/index')",
    ];

    let fail = vec![
        "import bar from './no-self-import'",
        "var bar = require('./no-self-import')",
        "var bar = require('./no-self-import.js')",
    ];

    Tester::new_without_config(NoAbsolutePath::NAME, pass, fail)
        .with_import_plugin(true)
        .test_and_snapshot();
}
